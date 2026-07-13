use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let mut n = input.next_u64();
    let mut count = 0u64;
    while n >= 5 {
        n /= 5;
        count += n;
    }
    output.writeln(count);
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
    fn n5() {
        assert_eq!(run("5\n"), "1\n");
    }

    #[test]
    fn n10() {
        assert_eq!(run("10\n"), "2\n");
    }

    #[test]
    fn n25() {
        assert_eq!(run("25\n"), "6\n");
    }

    #[test]
    fn n100() {
        assert_eq!(run("100\n"), "24\n");
    }
}
