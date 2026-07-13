use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let bytes = input.next_bytes();

    let mut longest = 0;
    let mut streak = 0;
    let mut previous = None;

    for &byte in bytes {
        if Some(byte) == previous {
            streak += 1;
        } else {
            previous = Some(byte);
            streak = 1;
        }
        longest = longest.max(streak);
    }
    output.writeln(longest);
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
    fn single_character() {
        assert_eq!(run("A\n"), "1\n");
    }

    #[test]
    fn all_same() {
        assert_eq!(run("AAAAAAA\n"), "7\n");
    }

    #[test]
    fn all_different() {
        assert_eq!(run("ABCDEFG\n"), "1\n");
    }

    #[test]
    fn official_sample() {
        assert_eq!(run("ATTCGGGA\n"), "3\n");
    }

    #[test]
    fn longest_at_beginning() {
        assert_eq!(run("AAAABCD\n"), "4\n");
    }

    #[test]
    fn longest_at_end() {
        assert_eq!(run("ABCDDDDD\n"), "5\n");
    }
}
