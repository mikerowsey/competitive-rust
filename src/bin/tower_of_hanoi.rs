use competitive_rust::{Output, Scanner};

fn hanoi(n: u64, from: u8, to: u8, output: &mut Output) {
    if n == 0 {
        return;
    }

    let spare = 6 - from - to;

    hanoi(n - 1, from, spare, output);

    output.write(from);
    output.write(" ");
    output.writeln(to);

    hanoi(n - 1, spare, to, output);
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64();

    output.writeln((1u64 << n) - 1);

    hanoi(n, 1, 3, output);
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
    fn n1() {
        assert_eq!(
            run("1\n"),
            "1\n1 3\n",
        );
    }

    #[test]
    fn n2() {
        assert_eq!(
            run("2\n"),
            "3\n1 2\n1 3\n2 3\n",
        );
    }

    #[test]
    fn n3() {
        let out = run("3\n");

        let mut lines = out.lines();

        assert_eq!(lines.next(), Some("7"));
        assert_eq!(lines.count(), 7);
    }
}