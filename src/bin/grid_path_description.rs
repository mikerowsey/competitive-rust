use competitive_rust::{Output, Scanner, CARDINAL_DIRS};

const PATH_LENGTH: usize = 48;
const DC: [u8; 4] = [b'U', b'D', b'L', b'R'];

fn one_way_neighbor(visited: &[[bool; 9]; 9], x: usize, y: usize) -> bool {
    if x == 7 && y == 1 {
        return false;
    }

    let mut free_neighbors = 0;
    for &(dx, dy) in &CARDINAL_DIRS {
        let nx = (x as i32 + dx) as usize;
        let ny = (y as i32 + dy) as usize;
        if !visited[nx][ny] {
            free_neighbors += 1;
        }
    }

    free_neighbors == 1
}

fn dfs(
    visited: &mut [[bool; 9]; 9],
    description: &[u8; PATH_LENGTH],
    x: usize,
    y: usize,
    step: usize,
) -> i32 {
    if x == 7 && y == 1 {
        return if step == PATH_LENGTH { 1 } else { 0 };
    }

    if step == PATH_LENGTH {
        return 0;
    }

    if visited[x - 1][y] && visited[x + 1][y] && !visited[x][y - 1] && !visited[x][y + 1] {
        return 0;
    }
    if visited[x][y - 1] && visited[x][y + 1] && !visited[x - 1][y] && !visited[x + 1][y] {
        return 0;
    }

    visited[x][y] = true;

    let required = description[step];
    let mut forced_direction = -1i32;

    if required == b'?' {
        for (direction, &(dx, dy)) in CARDINAL_DIRS.iter().enumerate() {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if visited[nx][ny] {
                continue;
            }

            if one_way_neighbor(visited, nx, ny) {
                if forced_direction != -1 && forced_direction != direction as i32 {
                    visited[x][y] = false;
                    return 0;
                }
                forced_direction = direction as i32;
            }
        }
    }

    let mut result = 0;
    for (direction, &(dx, dy)) in CARDINAL_DIRS.iter().enumerate() {
        if required != b'?' && DC[direction] != required {
            continue;
        }
        if forced_direction != -1 && forced_direction != direction as i32 {
            continue;
        }

        let nx = (x as i32 + dx) as usize;
        let ny = (y as i32 + dy) as usize;
        if visited[nx][ny] {
            continue;
        }

        result += dfs(visited, description, nx, ny, step + 1);
    }

    visited[x][y] = false;
    result
}

fn solve(input: &mut Scanner, output: &mut Output) {
    let pattern = input.next_bytes();

    let mut description = [b'?'; PATH_LENGTH];
    description.copy_from_slice(pattern);

    let mut visited = [[false; 9]; 9];
    for (i, row) in visited.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = i == 0 || i == 8 || j == 0 || j == 8;
        }
    }

    output.writeln(dfs(&mut visited, &description, 1, 1, 0));
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
    fn all_wildcards() {
        assert_eq!(
            run("????????????????????????????????????????????????\n"),
            "88418\n"
        );
    }
}
