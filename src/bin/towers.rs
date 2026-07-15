use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_usize();
    let mut towers: Vec<u64> = Vec::with_capacity(n);
    for _ in 0..n {
        let x = input.next_u64();
        let y = towers.partition_point(|&p| p <= x);
        if y == towers.len() {
            towers.push(x);
        } else {
            towers[y] = x;
        }
    }
    output.writeln(towers.len());
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
        let mut scan = Scanner::from_bytes(input.as_bytes());
        let mut out = Output::new();
        solve(&mut scan, &mut out);
        String::from_utf8(out.into_bytes()).unwrap()
    }

    #[test]
    fn test1() {
        assert_eq!(run("10\n10 4 5 9 4 10 2 7 4 6\n"), "4\n",);
    }

    #[test]
    fn test2() {
        assert_eq!(run("10\n1 2 3 4 5 8 7 1 9 1\n"), "7\n",);
    }

    #[test]
    fn test3() {
        assert_eq!(run("10\n10 9 8 7 6 10 4 3 2 1\n"), "2\n",);
    }
}