use competitive_rust::{Output, Scanner};

fn solve(scan: &mut Scanner, out: &mut Output) {
    let n = scan.next_u64();

    match n {
        1 => out.writeln(1),
        2 | 3 => out.writeln("NO SOLUTION"),
        _ => {
            let evens = (2..=n).step_by(2);
            let odds = (1..=n).step_by(2);

            out.write_iter(evens.chain(odds));
            out.endl();
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
    fn n1() {
        assert_eq!(run("1\n"), "1\n");
    }

    #[test]
    fn n2() {
        assert_eq!(run("2\n"), "NO SOLUTION\n");
    }

    #[test]
    fn n3() {
        assert_eq!(run("3\n"), "NO SOLUTION\n");
    }

    #[test]
    fn n4() {
        assert_eq!(run("4\n"), "2 4 1 3\n");
    }

    #[test]
    fn n5() {
        assert_eq!(run("5\n"), "2 4 1 3 5\n");
    }

    #[test]
    fn n7() {
        assert_eq!(run("7\n"), "2 4 6 1 3 5 7\n");
    }
}
