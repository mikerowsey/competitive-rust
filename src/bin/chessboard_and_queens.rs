use competitive_rust::{Output, Scanner};
const N: usize = 8;

fn count_queens(
    row: usize,
    board: &[[bool; N]; N],
    columns: u16,
    diagonal_down: u16,
    diagonal_up: u16,
) -> u64 {
    if row == N {
        return 1;
    }

    let mut total = 0;

    for column in 0..N {
        if !board[row][column] {
            continue;
        }

        let column_bit = 1u16 << column;
        let diagonal_down_bit = 1u16 << (row + column);
        let diagonal_up_bit = 1u16 << (row + N - 1 - column);

        if columns & column_bit != 0
            || diagonal_down & diagonal_down_bit != 0
            || diagonal_up & diagonal_up_bit != 0
        {
            continue;
        }

        total += count_queens(
            row + 1,
            board,
            columns | column_bit,
            diagonal_down | diagonal_down_bit,
            diagonal_up | diagonal_up_bit,
        );
    }

    total
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let mut board = [[false; N]; N];

    for row in &mut board {
        let input_row = input.next_bytes();

        for (cell, &byte) in row.iter_mut().zip(input_row) {
            *cell = byte == b'.';
        }
    }

    output.writeln(count_queens(0, &board, 0, 0, 0));
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
    fn empty_board() {
        assert_eq!(
            run("........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n"),
            "92\n"
        );
    }

    #[test]
    fn official_sample() {
        assert_eq!(
            run("........\n\
             ........\n\
             ..*.....\n\
             ........\n\
             ........\n\
             .....**.\n\
             ...*....\n\
             ........\n"),
            "65\n"
        );
    }

    #[test]
    fn completely_blocked_board() {
        assert_eq!(
            run("********\n\
                 ********\n\
                 ********\n\
                 ********\n\
                 ********\n\
                 ********\n\
                 ********\n\
                 ********\n"),
            "0\n"
        );
    }

    #[test]
    fn blocked_first_row() {
        assert_eq!(
            run("********\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n"),
            "0\n"
        );
    }
}
