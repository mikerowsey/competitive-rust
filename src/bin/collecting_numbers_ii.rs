use competitive_rust::{Output, Scanner};

fn is_break(value: usize, n: usize, positions: &[usize]) -> i32 {
    if value <= 1 || value > n {
        return 0;
    }
    if positions[value] < positions[value - 1] {
        1
    } else {
        0
    }
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_usize();
    let swap_count = input.next_usize();

    let mut values_at_position = vec![0usize; n + 1];
    let mut positions = vec![0usize; n + 1];

    for position in 1..=n {
        let value = input.next_usize();
        values_at_position[position] = value;
        positions[value] = position;
    }

    let mut rounds = 1i32;
    for value in 2..=n {
        rounds += is_break(value, n, &positions);
    }

    for _ in 0..swap_count {
        let left_position = input.next_usize();
        let right_position = input.next_usize();

        let left_value = values_at_position[left_position];
        let right_value = values_at_position[right_position];

        let mut affected_values = [left_value, left_value + 1, right_value, right_value + 1];
        affected_values.sort_unstable();

        let mut breaks_before = 0i32;
        let mut previous = usize::MAX;
        for &value in &affected_values {
            if value == previous {
                continue;
            }
            breaks_before += is_break(value, n, &positions);
            previous = value;
        }

        values_at_position.swap(left_position, right_position);
        positions[left_value] = right_position;
        positions[right_value] = left_position;

        let mut breaks_after = 0i32;
        let mut previous = usize::MAX;
        for &value in &affected_values {
            if value == previous {
                continue;
            }
            breaks_after += is_break(value, n, &positions);
            previous = value;
        }

        rounds += breaks_after - breaks_before;
        output.writeln(rounds);
    }
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
    fn basic_swaps() {
        assert_eq!(run("5 3\n4 2 1 5 3\n2 3\n1 5\n2 3\n"), "2\n3\n4\n");
    }
}
