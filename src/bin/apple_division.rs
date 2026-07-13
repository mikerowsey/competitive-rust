use competitive_rust::{Output, Scanner};

fn min_diff(weights: &[i64], index: usize, current: i64, total: i64) -> i64 {
    if index == weights.len() {
        return (total - 2 * current).abs();
    }

    std::cmp::min(
        min_diff(weights, index + 1, current, total),
        min_diff(weights, index + 1, current + weights[index], total),
    )
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64() as usize;

    let mut weights = Vec::with_capacity(n);
    for _ in 0..n {
        weights.push(input.next_i64());
    }

    let total: i64 = weights.iter().sum();

    output.writeln(min_diff(&weights, 0, 0, total));
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
        assert_eq!(run("5\n3 2 7 4 1\n"), "1\n",);
    }

    #[test]
    fn single() {
        assert_eq!(run("1\n10\n"), "10\n",);
    }

    #[test]
    fn equal_split() {
        assert_eq!(run("2\n5 5\n"), "0\n",);
    }

    #[test]
    fn powers_of_two() {
        assert_eq!(run("4\n1 2 4 8\n"), "1\n",);
    }

    #[test]
    fn all_equal() {
        assert_eq!(run("4\n5 5 5 5\n"), "0\n",);
    }

    #[test]
    fn increasing() {
        assert_eq!(run("6\n1 2 3 4 5 6\n"), "1\n",);
    }
}
