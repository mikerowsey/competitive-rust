use competitive_rust::{Scanner, Output};

fn solve(input: &mut Scanner, output: &mut Output) {
    let mut n = input.next_u64();
    output.write(n);
    while n != 1 {
        n = if n & 1 == 0 { n / 2 } else { 3 * n + 1 };
        output.write(" ");
        output.write(n);
    }
    output.endl();
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
    fn one() {
        assert_eq!(run("1\n"), "1\n");
    }

    #[test]
    fn forty_two() {
        assert_eq!(
            run("42\n"),
            "42 21 64 32 16 8 4 2 1\n"
        );
    }

    #[test]
    fn thirty_seven() {
        assert_eq!(
            run("37\n"),
            "37 112 56 28 14 7 22 11 34 17 52 26 13 40 20 10 5 16 8 4 2 1\n"
        );
    }
}