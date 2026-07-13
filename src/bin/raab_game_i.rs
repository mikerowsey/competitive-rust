use competitive_rust::{Output, Scanner};

fn append_block(permutation: &mut Vec<u64>, start: u64, size: u64, first_player_heavy: bool) {
    if size == 1 {
        permutation.push(start);
        return;
    }

    if first_player_heavy {
        permutation.push(start + size - 1);
        for value in 0..(size - 1) {
            permutation.push(start + value);
        }
    } else {
        for value in 1..size {
            permutation.push(start + value);
        }
        permutation.push(start);
    }
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let test_cases = input.next_u64();

    for _ in 0..test_cases {
        let n = input.next_u64();
        let mut a = input.next_u64();
        let mut b = input.next_u64();

        if (a == 0 && b == 0) || (a > 0 && b > 0 && a + b <= n) {
            output.writeln("YES");

            let mut swapped = false;
            if a < b {
                std::mem::swap(&mut a, &mut b);
                swapped = true;
            }

            let mut first_player = Vec::with_capacity(n as usize);
            let mut second_player = Vec::with_capacity(n as usize);

            let mut current = 1u64;

            if a == 0 && b == 0 {
                for value in 1..=n {
                    first_player.push(value);
                    second_player.push(value);
                }
            } else {
                let large_block_size = a - b + 2;

                for value in 0..large_block_size {
                    first_player.push(current + value);
                }
                append_block(&mut second_player, current, large_block_size, true);
                current += large_block_size;

                for _ in 0..(b - 1) {
                    first_player.push(current);
                    first_player.push(current + 1);
                    append_block(&mut second_player, current, 2, true);
                    current += 2;
                }

                while current <= n {
                    first_player.push(current);
                    second_player.push(current);
                    current += 1;
                }
            }

            if swapped {
                std::mem::swap(&mut first_player, &mut second_player);
            }

            output.writeln_iter(first_player.iter());
            output.writeln_iter(second_player.iter());
        } else {
            output.writeln("NO");
        }
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
    fn impossible_case() {
        assert_eq!(run("1\n3 2 2\n"), "NO\n");
    }

    #[test]
    fn zero_zero_case() {
        assert_eq!(run("1\n3 0 0\n"), "YES\n1 2 3\n1 2 3\n");
    }
}
