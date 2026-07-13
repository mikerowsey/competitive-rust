use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let coin_count = input.next_usize();

    let mut coins = Vec::with_capacity(coin_count);
    for _ in 0..coin_count {
        coins.push(input.next_u64());
    }
    coins.sort_unstable();

    let mut smallest_missing_sum = 1u64;
    for coin in coins {
        if coin > smallest_missing_sum {
            break;
        }
        smallest_missing_sum += coin;
    }

    output.writeln(smallest_missing_sum);
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
        assert_eq!(run("5\n2 9 1 2 7\n"), "6\n");
    }
}
