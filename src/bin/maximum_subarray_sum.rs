use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_usize();

    let mut prefix_sum = 0i64;
    let mut min_prefix = 0i64;
    let mut max_subarray = i64::MIN;

    for _ in 0..n {
        prefix_sum += input.next_i64();
        max_subarray = max_subarray.max(prefix_sum - min_prefix);
        min_prefix = min_prefix.min(prefix_sum);
    }

    output.writeln(max_subarray);
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
        assert_eq!(run("8\n-1 3 -2 5 3 -5 2 2\n"), "9\n");
    }

    #[test]
    fn all_negative() {
        assert_eq!(run("4\n-8 -3 -6 -2\n"), "-2\n");
    }
}
