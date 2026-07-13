use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_i64();
    for i in 1..=n {
        output.writeln((i.pow(4) - 9 * i.pow(2) + 24 * i - 16) / 2);
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
    fn n1() {
        assert_eq!(run("1\n"), "0\n");
    }

    #[test]
    fn n2() {
        assert_eq!(run("2\n"), "0\n6\n");
    }

    #[test]
    fn n3() {
        assert_eq!(run("3\n"), "0\n6\n28\n");
    }

    #[test]
    fn n5() {
        assert_eq!(
            run("5\n"),
            "0\n6\n28\n96\n252\n"
        );
    }

    #[test]
    fn n8() {
        assert_eq!(
            run("8\n"),
            "0\n6\n28\n96\n252\n550\n1056\n1848\n"
        );
    }
}