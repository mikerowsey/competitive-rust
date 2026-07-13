use competitive_rust::{Output, Scanner};

fn number_at(row: u64, col: u64) -> u64 {
    let max = row.max(col);
    let diagonal = 1 + max * (max - 1);

    if max & 1 == 0 {
        diagonal + row - col
    } else {
        diagonal + col - row
    }
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let queries = input.next_u64();

    for _ in 0..queries {
        let row = input.next_u64();
        let col = input.next_u64();

        output.writeln(number_at(row, col));
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
    fn sample_1() {
        assert_eq!(run("1\n2 3\n"), "8\n");
    }

    #[test]
    fn sample_2() {
        assert_eq!(run("1\n1 1\n"), "1\n");
    }

    #[test]
    fn sample_3() {
        assert_eq!(run("1\n4 2\n"), "15\n");
    }

    #[test]
    fn diagonal() {
        assert_eq!(run("1\n5 5\n"), "21\n");
    }

    #[test]
    fn large_row() {
        assert_eq!(run("1\n10 1\n"), "100\n");
    }

    #[test]
    fn large_coordinates() {
        assert_eq!(run("1\n1000 1000\n"), "999001\n");
    }

    #[test]
    fn multiple_queries() {
        assert_eq!(
            run("3\n\
                 2 3\n\
                 1 1\n\
                 4 2\n"),
            "8\n1\n15\n"
        );
    }
}
