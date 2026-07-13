use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let mut frequencies = [0u64; 26];

    for &byte in input.next_bytes() {
        frequencies[(byte - b'A') as usize] += 1;
    }

    let odd_count = frequencies
        .iter()
        .filter(|&&frequency| frequency & 1 == 1)
        .count();

    if odd_count > 1 {
        output.writeln("NO SOLUTION");
        return;
    }

    let middle = frequencies
        .iter()
        .position(|&frequency| frequency & 1 == 1)
        .map(|index| b'A' + index as u8);

    for (index, &frequency) in frequencies.iter().enumerate() {
        for _ in 0..frequency / 2 {
            output.write_byte(b'A' + index as u8);
        }
    }

    if let Some(byte) = middle {
        output.write_byte(byte);
    }

    for (index, &frequency) in frequencies.iter().enumerate().rev() {
        for _ in 0..frequency / 2 {
            output.write_byte(b'A' + index as u8);
        }
    }
    output.endl();
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

    fn is_palindrome(text: &str) -> bool {
        text.bytes().eq(text.bytes().rev())
    }

    #[test]
    fn sample() {
        let output = run("AAAACACBA\n");
        let palindrome = output.trim_end();

        assert!(is_palindrome(palindrome));
        assert_eq!(palindrome.len(), 9);
    }

    #[test]
    fn no_solution() {
        assert_eq!(run("ABC\n"), "NO SOLUTION\n");
    }

    #[test]
    fn single_character() {
        assert_eq!(run("A\n"), "A\n");
    }

    #[test]
    fn even_palindrome() {
        assert_eq!(run("AABB\n"), "ABBA\n");
    }

    #[test]
    fn one_odd_frequency() {
        assert_eq!(run("AAABB\n"), "ABABA\n");
    }

    #[test]
    fn all_same() {
        assert_eq!(run("AAAAAA\n"), "AAAAAA\n");
    }
}
