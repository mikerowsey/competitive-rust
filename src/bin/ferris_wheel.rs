use competitive_rust::{min_pairs_with_limit, Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let children_count = input.next_usize();
    let max_weight = input.next_u64();

    let mut weights = Vec::with_capacity(children_count);
    for _ in 0..children_count {
        weights.push(input.next_u64());
    }

    weights.sort_unstable();

    let gondolas = min_pairs_with_limit(&weights, max_weight);

    output.writeln(gondolas);
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
        assert_eq!(run("4 10\n7 2 3 9\n"), "3\n");
    }
}
