use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let bytes = input.next_bytes();
    let n = bytes.len();

    let mut count = [0usize; 26];
    for &ch in bytes {
        count[(ch - b'A') as usize] += 1;
    }

    if *count.iter().max().unwrap() > n.div_ceil(2) {
        output.writeln("-1");
        return;
    }

    let mut answer = Vec::with_capacity(n);
    let mut previous = usize::MAX;

    for position in 0..n {
        let mut placed = false;

        for current in 0..26 {
            if current == previous || count[current] == 0 {
                continue;
            }

            count[current] -= 1;

            let remaining = n - position - 1;
            let max_remaining = *count.iter().max().unwrap();

            if max_remaining <= remaining.div_ceil(2) {
                answer.push((b'A' + current as u8) as char);
                previous = current;
                placed = true;
                break;
            }

            count[current] += 1;
        }

        if !placed {
            output.writeln("-1");
            return;
        }
    }

    let answer: String = answer.into_iter().collect();
    output.writeln(answer);
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
    fn impossible() {
        assert_eq!(run("AAAA\n"), "-1\n");
    }

    #[test]
    fn simple() {
        assert_eq!(run("AAB\n"), "ABA\n");
    }
}
