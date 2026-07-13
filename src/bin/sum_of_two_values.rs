use competitive_rust::{find_two_sum_indices, Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let value_count = input.next_usize();
    let target_sum = input.next_u32();

    let mut values = Vec::with_capacity(value_count);
    for index in 0..value_count {
        values.push((index + 1, input.next_u32()));
    }

    values.sort_unstable_by_key(|entry| entry.1);

    if let Some((left_index, right_index)) = find_two_sum_indices(&values, target_sum) {
        output.write(left_index);
        output.write_byte(b' ');
        output.writeln(right_index);
        return;
    }

    output.writeln("IMPOSSIBLE");
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
        assert_eq!(run("4 8\n2 7 5 1\n"), "4 2\n");
    }

    #[test]
    fn impossible() {
        assert_eq!(run("3 10\n1 2 3\n"), "IMPOSSIBLE\n");
    }
}
