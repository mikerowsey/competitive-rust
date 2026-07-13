# Competitive Rust Engineering Repo

## Mission Statement

Build professional software engineering habits through competitive programming in Rust.

This repository uses CSES as a training ground for:

- algorithmic reasoning and complexity discipline
- clean, idiomatic Rust
- repeatable local verification
- maintainable project conventions as solved problem count grows

The objective is not just Accepted submissions.
The objective is solutions and infrastructure that hold up under professional code review.

## Philosophy (Rust Edition)

Priorities, in order:

1. Best algorithmic approach first.
2. Idiomatic, explicit Rust implementation.
3. Clean, descriptive, maintainable code.
4. Fast execution details (constant factors, I/O, memory layout).

Practical rules:

- Optimize asymptotic complexity before micro-optimizations.
- Prefer readability over clever tricks.
- Use integer widths intentionally.
- Keep allocations predictable and proportional to constraints.
- Favor reusable helpers only when they are genuinely reusable across problems.

CSES-specific conventions:

- Assume judge input is valid.
- Keep single-problem binaries straightforward and low ceremony.
- Keep bundled outputs reproducible from source.

## Repository Status

- Progress is tracked directly by solution files in [src/bin](src/bin) and problem notes in [notes](notes).
- Rust binaries in [src/bin](src/bin): one binary per solved problem.
- Notes in [notes](notes): 37 markdown files.

## Project Layout

- [src/bin](src/bin): one binary per solved problem.
- [src/lib.rs](src/lib.rs): shared exports for io, math, search, and reusable algorithms.
- [src/io](src/io): fast scanner/output utilities.
- [src/math](src/math): reusable math helpers (factorial, modular power, permutations, 2x2 mod matrix).
- [src/search](src/search): reusable search/grid helpers (directions, bounds, DFS utility).
- [src/algo](src/algo): reusable algorithmic patterns (two-pointers utilities).
- [notes](notes): per-problem notes with pattern/technique/insight.
- [bundles](bundles): generated single-file submissions.
- [scripts/run_bundle.sh](scripts/run_bundle.sh), [scripts/bundle.py](scripts/bundle.py): bundling pipeline.

## Quick Start

Run one solution:

```bash
cargo run --bin missing_number
```

Run tests:

```bash
cargo test
cargo test --bin weird_algorithm
```

Format:

```bash
cargo fmt --all
```

Bundle all problems for submission:

```bash
scripts/run_bundle.sh
```

Bundle one problem:

```bash
python3 scripts/bundle.py missing_number
```

## Reuse Policy

Shared code is extracted only when it is already reused or clearly reusable across multiple problems.

Current reusable modules include:

- [src/math/mod_pow.rs](src/math/mod_pow.rs)
- [src/math/permutation.rs](src/math/permutation.rs)
- [src/math/mod_matrix_2x2.rs](src/math/mod_matrix_2x2.rs)
- [src/search/grid.rs](src/search/grid.rs)
- [src/algo/two_pointers.rs](src/algo/two_pointers.rs)

This keeps per-problem code simple while growing a practical library over time.

## Suggested Daily Loop

```bash
cargo fmt --all
cargo test
scripts/run_bundle.sh
```

## Toolchain

- Rust toolchain is pinned in [rust-toolchain.toml](rust-toolchain.toml) for reproducibility.

## License

MIT. See [LICENSE.md](LICENSE.md).
