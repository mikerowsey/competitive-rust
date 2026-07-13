use competitive_rust::{max_matches_with_tolerance, Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let applicants_count = input.next_usize();
    let apartments_count = input.next_usize();
    let tolerance = input.next_i64();

    let mut desired_sizes = Vec::with_capacity(applicants_count);
    for _ in 0..applicants_count {
        desired_sizes.push(input.next_i64());
    }

    let mut apartment_sizes = Vec::with_capacity(apartments_count);
    for _ in 0..apartments_count {
        apartment_sizes.push(input.next_i64());
    }

    desired_sizes.sort_unstable();
    apartment_sizes.sort_unstable();

    let matches = max_matches_with_tolerance(&desired_sizes, &apartment_sizes, tolerance);

    output.writeln(matches);
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
        assert_eq!(run("4 3 5\n60 45 80 60\n30 60 75\n"), "2\n");
    }
}
