use competitive_rust::{Output, Scanner};

fn solve(input: &mut Scanner, output: &mut Output) {
    let movie_count = input.next_usize();

    let mut movies = Vec::with_capacity(movie_count);
    for _ in 0..movie_count {
        let start_time = input.next_u32();
        let end_time = input.next_u32();
        movies.push((start_time, end_time));
    }

    movies.sort_unstable_by_key(|movie| movie.1);

    let mut current_end_time = 0u32;
    let mut selected_movies = 0u32;
    for (start_time, end_time) in movies {
        if start_time >= current_end_time {
            current_end_time = end_time;
            selected_movies += 1;
        }
    }

    output.writeln(selected_movies);
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
        assert_eq!(run("3\n3 5\n4 9\n5 8\n"), "2\n");
    }
}
