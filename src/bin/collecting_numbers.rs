use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_usize();

    let mut positions = vec![0usize; n + 1];
    for index in 0..n {
        let value = input.next_usize();
        positions[value] = index;
    }

    let mut rounds = 1u64;
    for value in 1..=n {
        if positions[value] < positions[value - 1] {
            rounds += 1;
        }
    }

    output.writeln(rounds);
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
        assert_eq!(run("5\n4 2 1 5 3\n"), "3\n");
    }
}
