use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64();

    for row in 0..n {
        for column in 0..n {
            if column > 0 {
                output.write_byte(b' ');
            }
            output.write(row ^ column);
        }
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
    fn small() {
        assert_eq!(run("3\n"), "0 1 2\n1 0 3\n2 3 0\n");
    }
}
