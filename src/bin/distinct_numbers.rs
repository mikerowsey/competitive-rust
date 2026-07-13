use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_usize();

    let mut numbers = Vec::with_capacity(n);
    for _ in 0..n {
        numbers.push(input.next_u64());
    }

    numbers.sort_unstable();

    let mut distinct_count = 0u64;
    let mut previous = 0u64;
    let mut first = true;
    for value in numbers {
        if first || value != previous {
            distinct_count += 1;
            previous = value;
            first = false;
        }
    }

    output.writeln(distinct_count);
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
        assert_eq!(run("5\n2 3 2 2 3\n"), "2\n");
    }
}
