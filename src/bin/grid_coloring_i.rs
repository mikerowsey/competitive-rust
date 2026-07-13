use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_usize();
    let m = input.next_usize();

    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        grid.push(input.next_string().into_bytes());
    }

    for row in 0..n {
        for column in 0..m {
            let first_choice = if (row + column) % 2 == 0 { b'A' } else { b'C' };
            let second_choice = if (row + column) % 2 == 0 { b'B' } else { b'D' };

            if grid[row][column] == first_choice {
                output.write_byte(second_choice);
            } else {
                output.write_byte(first_choice);
            }
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
        assert_eq!(run("2 2\nAB\nCD\n"), "BC\nDA\n");
    }
}
