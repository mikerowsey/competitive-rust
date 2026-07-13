use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let applicants_count = input.next_usize();
    let apartments_count = input.next_usize();
    let tolerance = input.next_i64();

    let mut desired_sizes = Vec::with_capacity(applicants_count);
    for _ in 0..applicants_count {
        desired_sizes.push(input.next_i64());
    }

    let mut apartment_sizes = Vec::with_capacity(apartments_count);
    for _ in 0..apartments_count {
        apartment_sizes.push(input.next_i64());
    }

    desired_sizes.sort_unstable();
    apartment_sizes.sort_unstable();

    let mut applicant_index = 0usize;
    let mut apartment_index = 0usize;
    let mut matches = 0u64;

    while applicant_index < applicants_count && apartment_index < apartments_count {
        let desired_size = desired_sizes[applicant_index];
        let apartment_size = apartment_sizes[apartment_index];

        if (desired_size - apartment_size).abs() <= tolerance {
            applicant_index += 1;
            apartment_index += 1;
            matches += 1;
        } else if desired_size - apartment_size > tolerance {
            apartment_index += 1;
        } else {
            applicant_index += 1;
        }
    }

    output.writeln(matches);
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
    fn sample() {
        assert_eq!(run("4 3 5\n60 45 80 60\n30 60 75\n"), "2\n");
    }
}
