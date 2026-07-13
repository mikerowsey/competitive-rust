use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_u64() as usize;
    for i in 0..(1usize << n) {
        let gray = i ^ (i >> 1);
        for bit in (0..n).rev() {
            output.write_byte(b'0' + ((gray >> bit) & 1) as u8);
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
        let mut scan = Scanner::from_bytes(input.as_bytes());
        let mut out = Output::new();

        solve(&mut scan, &mut out);

        String::from_utf8(out.into_bytes()).unwrap()
    }

    #[test]
    fn n1() {
        assert_eq!(
            run("1\n"),
            "0\n\
             1\n"
        );
    }

    #[test]
    fn n2() {
        assert_eq!(
            run("2\n"),
            "00\n\
             01\n\
             11\n\
             10\n"
        );
    }

    #[test]
    fn n3() {
        assert_eq!(
            run("3\n"),
            "000\n\
             001\n\
             011\n\
             010\n\
             110\n\
             111\n\
             101\n\
             100\n"
        );
    }
}
