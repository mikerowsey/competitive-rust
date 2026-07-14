#!/usr/bin/env python3

import sys
import os
import re
import subprocess
import tempfile
from pathlib import Path

# Change to parent directory (project root)
os.chdir(Path(__file__).parent.parent)


def read_file(path):
    """Read file contents."""
    with open(path, 'r') as f:
        return f.read()


def write_file(path, content):
    """Write content to file."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, 'w') as f:
        f.write(content)


def indent_lines(code, spaces=4):
    """Indent all non-empty lines."""
    prefix = ' ' * spaces
    return '\n'.join(prefix + line if line.strip() else '' for line in code.split('\n'))


def clean_code(code):
    """Remove test modules and test attributes from code."""
    code = re.sub(r'#\[cfg\(test\)\].*?mod\s+\w+\s+\{.*?\n\}', '', code, flags=re.DOTALL)
    code = re.sub(r'\s*#\[test\].*?\n', '', code)
    code = re.sub(r'#\[must_use\]\n', '', code)
    return code.strip()


def extract_code_without_imports(content):
    """Extract code and remove all use statements (including multi-line)."""
    content = clean_code(content)
    # Remove multi-line and single-line use statements
    content = re.sub(r'use\s+[^;]+;', '', content, flags=re.MULTILINE | re.DOTALL)
    # Clean up any leftover whitespace
    content = re.sub(r'\n\s*\n', '\n\n', content)
    return content.strip()


def symbol_used(code, symbol):
    return re.search(rf'\b{re.escape(symbol)}\b', code) is not None


def inline_io_modules():
    """Inline IO module code."""
    scanner_content = read_file('src/io/scanner.rs')
    output_content = read_file('src/io/output.rs')

    scanner_code = extract_code_without_imports(scanner_content)
    output_code = extract_code_without_imports(output_content)

    # Add back necessary imports for each submodule
    scanner_with_imports = "use std::io::{self, Read};\n\n" + scanner_code
    output_with_imports = "use std::fmt;\nuse std::io::{self, Write};\n\n" + output_code

    io_code = f"""mod io {{
    pub mod scanner {{
{indent_lines(scanner_with_imports, 8)}
    }}

    pub mod output {{
{indent_lines(output_with_imports, 8)}
    }}

    pub use output::{{Output, Writable}};
    pub use scanner::Scanner;
}}

use io::{{Scanner, Output}};
"""
    return io_code


def inline_math_modules(bin_code):
    """Inline only the math modules referenced by the solution."""
    modules = []
    uses = []

    if symbol_used(bin_code, 'factorial'):
        factorial_content = read_file('src/math/factorial.rs')
        factorial_code = extract_code_without_imports(factorial_content)
        modules.append(f"mod factorial {{\n{indent_lines(factorial_code, 4)}\n}}")
        uses.append('factorial::factorial')

    if symbol_used(bin_code, 'ModMatrix2x2'):
        matrix_content = read_file('src/math/mod_matrix_2x2.rs')
        matrix_code_raw = extract_code_without_imports(matrix_content)
        matrix_with_imports = "use std::ops::Mul;\n\n" + matrix_code_raw
        modules.append(f"mod mod_matrix_2x2 {{\n{indent_lines(matrix_with_imports, 4)}\n}}")
        uses.append('mod_matrix_2x2::ModMatrix2x2')

    if symbol_used(bin_code, 'mod_pow'):
        mod_pow_content = read_file('src/math/mod_pow.rs')
        mod_pow_code = extract_code_without_imports(mod_pow_content)
        modules.append(f"mod mod_pow {{\n{indent_lines(mod_pow_code, 4)}\n}}")
        uses.append('mod_pow::mod_pow')

    if symbol_used(bin_code, 'next_permutation'):
        permutation_content = read_file('src/math/permutation.rs')
        permutation_code = extract_code_without_imports(permutation_content)
        modules.append(f"mod permutation {{\n{indent_lines(permutation_code, 4)}\n}}")
        uses.append('permutation::next_permutation')

    module_code = ''
    use_code = ''
    if modules:
        module_code = '\n' + '\n\n'.join(modules) + '\n'
    if uses:
        use_code = ''.join(f'\nuse {path};' for path in uses) + '\n'

    return module_code, use_code


def inline_algo_modules(bin_code):
    """Inline only the algorithm modules referenced by the solution."""
    if not any(symbol_used(bin_code, sym) for sym in [
        'max_matches_with_tolerance', 'min_pairs_with_limit', 'find_two_sum_indices'
    ]):
        return '', ''

    two_pointers_content = read_file('src/algo/two_pointers.rs')
    two_pointers_code = extract_code_without_imports(two_pointers_content)

    module_code = f"\nmod two_pointers {{\n{indent_lines(two_pointers_code, 4)}\n}}\n"

    uses = []
    for sym in ['find_two_sum_indices', 'max_matches_with_tolerance', 'min_pairs_with_limit']:
        if symbol_used(bin_code, sym):
            uses.append(f'two_pointers::{sym}')

    use_code = ''
    if uses:
        use_code = ''.join(f'\nuse {path};' for path in uses) + '\n'

    return module_code, use_code


def inline_search_modules(bin_code):
    """Inline only the search modules referenced by the solution."""
    modules = []
    uses = []

    if symbol_used(bin_code, 'dfs_recursive'):
        dfs_content = read_file('src/search/dfs.rs')
        dfs_code = extract_code_without_imports(dfs_content)
        modules.append(f"mod dfs {{\n{indent_lines(dfs_code, 4)}\n}}")
        uses.append('dfs::dfs_recursive')

    if any(symbol_used(bin_code, sym) for sym in ['CARDINAL_DIRS', 'KNIGHT_DIRS', 'in_bounds']):
        grid_content = read_file('src/search/grid.rs')
        grid_code = extract_code_without_imports(grid_content)
        modules.append(f"mod grid {{\n{indent_lines(grid_code, 4)}\n}}")

        for sym in ['CARDINAL_DIRS', 'KNIGHT_DIRS', 'in_bounds']:
            if symbol_used(bin_code, sym):
                uses.append(f'grid::{sym}')

    module_code = ''
    use_code = ''
    if modules:
        module_code = '\n' + '\n\n'.join(modules) + '\n'
    if uses:
        use_code = ''.join(f'\nuse {path};' for path in uses) + '\n'

    return module_code, use_code


def bundle_solution(problem_name):
    """Bundle a solution with inlined dependencies."""
    bin_path = Path('src/bin') / f'{problem_name}.rs'

    if not bin_path.exists():
        print(f"Error: {bin_path} not found")
        return False

    bin_code = read_file(bin_path)

    # Skip bundling if external crates are used (serde, serde_json, etc)
    external_crates = ['serde', 'serde_json', 'regex', 'chrono', 'itertools', 'ndarray']
    for crate in external_crates:
        if crate in bin_code:
            print(f"Skipping {problem_name} (uses external crate: {crate})")
            return True

    bin_code = re.sub(r'use competitive_rust::\{[^}]+\};?\n*', '', bin_code)
    bin_code = clean_code(bin_code)

    bundle_content = '#![allow(dead_code)]\n\n' + bin_code + '\n\n'
    bundle_content += inline_io_modules()

    math_inline, math_uses = inline_math_modules(bin_code)
    algo_inline, algo_uses = inline_algo_modules(bin_code)
    search_inline, search_uses = inline_search_modules(bin_code)

    bundle_content += math_inline
    bundle_content += algo_inline
    bundle_content += search_inline
    bundle_content += math_uses
    bundle_content += algo_uses
    bundle_content += search_uses

    try:
        result = subprocess.run(
            ['rustfmt', '--edition', '2021'],
            input=bundle_content,
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            bundle_content = result.stdout
        else:
            print(f"Warning: rustfmt failed for {problem_name}")
    except Exception as e:
        print(f"Warning: Could not run rustfmt: {e}")

    bundle_path = Path('bundles') / f'{problem_name}.rs'
    write_file(bundle_path, bundle_content)

    print(f"Bundling {problem_name}...", end=' ')
    try:
        # Create a temporary directory for the binary output
        with tempfile.TemporaryDirectory() as tmpdir:
            output_binary = Path(tmpdir) / problem_name
            result = subprocess.run(
                ['rustc', '--crate-type', 'bin', '--edition', '2021', '-o', str(output_binary), str(bundle_path)],
                capture_output=True,
                text=True,
                timeout=10
            )
            if result.returncode == 0:
                print("✓")
                return True
            else:
                print("✗")
                print(f"  Compilation failed:\n{result.stderr}")
                return False
    except Exception as e:
        print(f"✗ Error: {e}")
        return False


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: bundle.py <problem_name>")
        sys.exit(1)

    problem = sys.argv[1]
    if not bundle_solution(problem):
        sys.exit(1)
