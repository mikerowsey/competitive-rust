use competitive_rust::{Output, Scanner};
fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64();
    let sum = n * (n + 1) / 2;

    if sum & 1 != 0 {
        output.writeln("NO");
        return;
    }

    let mut target = sum / 2;
    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in (1..=n).rev() {
        if i <= target {
            target -= i;
            a.push(i);
        } else {
            b.push(i);
        }
    }

    output.writeln("YES");

    output.writeln(a.len());
    output.write_iter(&a);
    output.endl();

    output.writeln(b.len());
    output.write_iter(&b);
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

    #[test]
    fn n1() {
        assert_eq!(run("1\n"), "NO\n");
    }

    #[test]
    fn n2() {
        assert_eq!(run("2\n"), "NO\n");
    }

    #[test]
    fn n3() {
        assert_eq!(
            run("3\n"),
            "YES\n\
             1\n\
             3\n\
             2\n\
             2 1\n"
        );
    }

    #[test]
    fn n4() {
        assert_eq!(
            run("4\n"),
            "YES\n\
             2\n\
             4 1\n\
             2\n\
             3 2\n"
        );
    }

    #[test]
    fn n7() {
        assert_eq!(
            run("7\n"),
            "YES\n\
             3\n\
             7 6 1\n\
             4\n\
             5 4 3 2\n"
        );
    }
}
