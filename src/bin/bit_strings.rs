use competitive_rust::{mod_pow, Output, Scanner};

const MOD: u64 = 1_000_000_007;

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64();
    output.writeln(mod_pow(2, n, MOD));
}

fn main() {
    let mut input = Scanner::new();
    let mut output = Output::new();

    solve(&mut input, &mut output);

    output.print();
}

#[cfg(test)]
mod tests {
    use super::*;
    use competitive_rust::{Output, Scanner};

    fn run(input: &str) -> String {
        let mut input = Scanner::from_bytes(input.as_bytes());
        let mut output = Output::new();

        solve(&mut input, &mut output);

        String::from_utf8(output.into_bytes()).unwrap()
    }

    #[test]
    fn n3() {
        assert_eq!(run("3\n"), "8\n");
    }

    #[test]
    fn n10() {
        assert_eq!(run("10\n"), "1024\n");
    }

    #[test]
    fn n30() {
        assert_eq!(run("30\n"), "73741817\n");
    }

    #[test]
    fn n0() {
        assert_eq!(run("0\n"), "1\n");
    }
}
