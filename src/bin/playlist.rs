use competitive_rust::{Output, Scanner};
use std::collections::HashMap;

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_usize();
    let mut seen = HashMap::with_capacity(n);
    let mut maximum = 0;
    let mut current = 0;
    for i in 0..n {
        let x: usize = input.next_usize();
        current += 1;
        if let Some(prev) = seen.insert(x, i) {
            current = current.min(i - prev);
        }
        maximum = maximum.max(current);
    }
    maximum = maximum.max(current);
    output.writeln(maximum);
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
        let mut scan = Scanner::from_bytes(input.as_bytes());
        let mut out = Output::new();

        solve(&mut scan, &mut out);

        String::from_utf8(out.into_bytes()).unwrap()
    }

    #[test]
    fn test1() {
        assert_eq!(run("10\n3 3 3 3 5 1 5 1 1 4\n"), "3\n",);
    }

    #[test]
    fn test2() {
        assert_eq!(run("10\n45 9 37 81 69 99 49 71 90 30\n"), "10\n",);
    }

    #[test]
    fn test3() {
        assert_eq!(run("10\n1 1 1 1 1 1 1 1 1 1\n"), "1\n",);
    }

    #[test]
    fn duplicate_after_window_moves() {
        assert_eq!(run("7\n1 2 3 4 2 5 6\n"), "5\n");
    }

    #[test]
    fn multiple_duplicates() {
        assert_eq!(run("6\n1 2 3 2 3 4\n"), "3\n");
    }

    #[test]
    fn duplicate_at_end() {
        assert_eq!(run("5\n1 2 3 4 1\n"), "4\n");
    }

    #[test]
    fn alternating() {
        assert_eq!(run("6\n1 2 1 2 1 2\n"), "2\n");
    }

    #[test]
    fn repeat_after_left_moves_twice() {
        assert_eq!(run("8\n1 2 3 1 2 3 4 5\n"), "5\n");
    }

    #[test]
    fn cses_sample_style() {
        assert_eq!(run("8\n2 1 2 3 4 2 5 6\n"), "5\n");
    }
}
