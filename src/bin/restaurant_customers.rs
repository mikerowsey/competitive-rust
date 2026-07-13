use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let customer_count = input.next_usize();

    let mut events = Vec::with_capacity(customer_count * 2);

    for _ in 0..customer_count {
        let arrival = input.next_u32();
        let departure = input.next_u32();
        events.push((arrival, 1i32));
        events.push((departure, -1i32));
    }

    events.sort_unstable_by(|left, right| {
        if left.0 != right.0 {
            left.0.cmp(&right.0)
        } else {
            left.1.cmp(&right.1)
        }
    });

    let mut current_customers = 0i32;
    let mut max_customers = 0i32;
    for (_, delta) in events {
        current_customers += delta;
        max_customers = max_customers.max(current_customers);
    }

    output.writeln(max_customers);
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
        assert_eq!(run("3\n5 8\n2 4\n3 9\n"), "2\n");
    }
}
