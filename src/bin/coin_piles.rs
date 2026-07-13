use competitive_rust::{Output, Scanner};

fn possible(a: u64, b: u64) -> bool {
    a <= 2 * b && b <= 2 * a && (a + b) % 3 == 0
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let cases = input.next_u64();

    for _ in 0..cases {
        let a = input.next_u64();
        let b = input.next_u64();

        if possible(a, b) {
            output.writeln("YES");
        } else {
            output.writeln("NO");
        }
    }
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
    fn yes() {
        assert_eq!(run("1\n2 1\n"), "YES\n");
    }

    #[test]
    fn no_sum_not_divisible_by_three() {
        assert_eq!(run("1\n3 1\n"), "NO\n");
    }

    #[test]
    fn no_too_imbalanced() {
        assert_eq!(run("1\n9 2\n"), "NO\n");
    }

    #[test]
    fn both_zero() {
        assert_eq!(run("1\n0 0\n"), "YES\n");
    }

    #[test]
    fn equal_piles() {
        assert_eq!(run("1\n6 6\n"), "YES\n");
    }

    #[test]
    fn multiple_cases() {
        assert_eq!(
            run("4\n\
                 2 1\n\
                 3 1\n\
                 6 6\n\
                 9 2\n"),
            "YES\n\
             NO\n\
             YES\n\
             NO\n"
        );
    }
}
