use competitive_rust::{ModMatrix2x2, Output, Scanner};
const MOD: u64 = 1_000_000_007;

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64();
    let result = match n {
        0 => 0,
        1 => 1,
        _ => ModMatrix2x2::<MOD>::new(0, 1, 1, 1).pow(n - 1).a11,
    };
    output.writeln(result);
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
    fn zero() {
        assert_eq!(run("0\n"), "0\n");
    }

    #[test]
    fn one() {
        assert_eq!(run("1\n"), "1\n");
    }

    #[test]
    fn forty_two() {
        assert_eq!(run("42\n"), "267914296\n");
    }
}
