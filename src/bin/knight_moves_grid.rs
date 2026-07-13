use std::collections::VecDeque;

use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let n = input.next_i32();

    let dr = [-2, -2, -1, -1, 1, 1, 2, 2];
    let dc = [-1, 1, -2, 2, -2, 2, -1, 1];

    let size = (n as usize) * (n as usize);
    let mut distance = vec![-1i32; size];
    let mut pending = VecDeque::new();

    distance[0] = 0;
    pending.push_back(0i32);

    while let Some(index) = pending.pop_front() {
        let row = index / n;
        let column = index % n;
        let current_distance = distance[index as usize];

        for mv in 0..8 {
            let next_row = row + dr[mv];
            let next_column = column + dc[mv];

            if next_row < 0 || next_row >= n || next_column < 0 || next_column >= n {
                continue;
            }

            let next_index = next_row * n + next_column;
            if distance[next_index as usize] != -1 {
                continue;
            }

            distance[next_index as usize] = current_distance + 1;
            pending.push_back(next_index);
        }
    }

    for row in 0..n {
        for column in 0..n {
            if column > 0 {
                output.write_byte(b' ');
            }
            output.write(distance[(row * n + column) as usize]);
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
    fn n_one() {
        assert_eq!(run("1\n"), "0\n");
    }

    #[test]
    fn n_three() {
        assert_eq!(run("3\n"), "0 3 2\n3 -1 1\n2 1 4\n");
    }
}
