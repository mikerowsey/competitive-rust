use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let value_count = input.next_usize();
    let target_sum = input.next_u32();

    let mut values = Vec::with_capacity(value_count);
    for index in 0..value_count {
        values.push((index + 1, input.next_u32()));
    }

    values.sort_unstable_by_key(|entry| entry.1);

    let mut left = 0usize;
    let mut right = value_count.saturating_sub(1);
    while left < right {
        let sum = values[left].1 + values[right].1;
        if sum > target_sum {
            right -= 1;
        } else if sum < target_sum {
            left += 1;
        } else {
            output.write(values[left].0);
            output.write_byte(b' ');
            output.writeln(values[right].0);
            return;
        }
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
