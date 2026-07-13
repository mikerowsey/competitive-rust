use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64();
    let mut prev = input.next_u64();
    let mut moves = 0u64;
    for _ in 1..n {
        let cur = input.next_u64();
        if cur < prev {
            moves += prev - cur;
        } else {
            prev = cur;
        }
    }
    output.writeln(moves);
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
            run("5\n3 2 5 1 7\n"),
            "5\n",
        );
    }

    #[test]
    fn already_increasing() {
        assert_eq!(
            run("5\n1 2 3 4 5\n"),
            "0\n",
        );
    }

    #[test]
    fn all_equal() {
        assert_eq!(
            run("5\n7 7 7 7 7\n"),
            "0\n",
        );
    }

    #[test]
    fn strictly_decreasing() {
        assert_eq!(
            run("5\n5 4 3 2 1\n"),
            "10\n",
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(
            run("1\n42\n"),
            "0\n",
        );
    }
}