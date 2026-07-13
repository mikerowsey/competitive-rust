use competitive_rust::{factorial, Output, Scanner};
fn next_permutation(v: &mut [u8]) -> bool {
    let n = v.len();

    for i in (0..n - 1).rev() {
        if v[i] < v[i + 1] {
            for j in (i + 1..n).rev() {
                if v[i] < v[j] {
                    v.swap(i, j);
                    break;
                }
            }

            v[i + 1..].reverse();
            return true;
        }
    }

    false
}

fn permutation_count(bytes: &[u8]) -> usize {
    let mut frequency = [0usize; 26];

    for &byte in bytes {
        frequency[(byte - b'a') as usize] += 1;
    }

    let mut total = factorial(bytes.len());

    for count in frequency {
        total /= factorial(count);
    }

    total
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let mut bytes = input.next_bytes().to_vec();
    bytes.sort_unstable();
    output.writeln(permutation_count(&bytes));

    loop {
        output.write(bytes.as_slice());
        output.endl();

        if !next_permutation(&mut bytes) {
            break;
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
    fn single_character() {
        assert_eq!(
            run("a\n"),
            "1\n\
             a\n"
        );
    }

    #[test]
    fn two_distinct() {
        assert_eq!(
            run("ab\n"),
            "2\n\
             ab\n\
             ba\n"
        );
    }

    #[test]
    fn repeated_characters() {
        assert_eq!(
            run("aabb\n"),
            "6\n\
             aabb\n\
             abab\n\
             abba\n\
             baab\n\
             baba\n\
             bbaa\n"
        );
    }

    #[test]
    fn all_same() {
        assert_eq!(
            run("aaaa\n"),
            "1\n\
             aaaa\n"
        );
    }

    #[test]
    fn three_distinct() {
        assert_eq!(
            run("abc\n"),
            "6\n\
             abc\n\
             acb\n\
             bac\n\
             bca\n\
             cab\n\
             cba\n"
        );
    }
}
