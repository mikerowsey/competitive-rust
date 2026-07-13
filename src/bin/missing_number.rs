use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64();
    let expected = n * (n + 1) / 2;
    let mut actual = 0;

    for _ in 0..n - 1 {
        actual += input.next_u64();
    }
    output.writeln(expected - actual);
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
    fn sample() {
        assert_eq!(
            run("5\n2 3 1 5\n"),
            "4\n",
        );
    }

    #[test]
    fn missing_first() {
        assert_eq!(
            run("5\n2 3 4 5\n"),
            "1\n",
        );
    }

    #[test]
    fn missing_last() {
        assert_eq!(
            run("5\n1 2 3 4\n"),
            "5\n",
        );
    }

    #[test]
    fn single() {
        assert_eq!(
            run("1\n"),
            "1\n",
        );
    }

    #[test]
    fn unordered() {
        assert_eq!(
            run("8\n8 2 5 3 6 1 7\n"),
            "4\n",
        );
    }
}