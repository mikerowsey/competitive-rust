use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_usize();

    let mut sticks = Vec::with_capacity(n);
    for _ in 0..n {
        sticks.push(input.next_i64());
    }

    sticks.sort_unstable();

    let median = sticks[n / 2];
    let mut result = 0i64;
    for stick in sticks {
        result += (stick - median).abs();
    }

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
        let mut input = Scanner::from_bytes(input.as_bytes());
        let mut output = Output::new();
        solve(&mut input, &mut output);
        String::from_utf8(output.into_bytes()).unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(run("5\n2 3 1 5 2\n"), "5\n");
    }
}
