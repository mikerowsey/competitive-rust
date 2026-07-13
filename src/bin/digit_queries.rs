use competitive_rust::{Output, Scanner};

fn digit_at(mut position: i64) -> u8 {
    let mut digits = 1;
    let mut block_size = 9;

    while position > block_size {
        position -= block_size;
        digits += 1;
        block_size = 9 * 10_i64.pow(digits as u32 - 1) * digits;
    }

    position -= 1;

    let number = 10_i64.pow(digits as u32 - 1) + position / digits;

    number.to_string().as_bytes()[(position % digits) as usize]
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let queries = input.next_u64();

    for _ in 0..queries {
        let position = input.next_i64();
        output.write_byte(digit_at(position));
        output.endl();
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
    fn single_digits() {
        assert_eq!(run("1\n7\n"), "7\n");
        assert_eq!(run("1\n9\n"), "9\n");
    }

    #[test]
    fn two_digit_boundary() {
        assert_eq!(run("1\n10\n"), "1\n");
        assert_eq!(run("1\n11\n"), "0\n");
    }

    #[test]
    fn multiple_queries() {
        assert_eq!(
            run(
                "3\n\
                 7\n\
                 19\n\
                 12\n"
            ),
            "7\n4\n1\n"
        );
    }
}