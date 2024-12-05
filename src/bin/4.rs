use std::fmt::Debug;

fn main() {
    let search = std::fs::read_to_string("input/4").unwrap();
    let grid = parse_grid(&search);
    println!("XMAS appears {} times", word_search(&grid));
    println!("There are {} X-MASes", x_mas_search(&grid));
}

fn parse_grid(s: &str) -> Vec<&str> {
    s.lines().collect()
}

fn word_search<Row: AsRef<[u8]> + Debug>(grid: &[Row]) -> usize {
    // All rows should have the same length.
    grid.iter()
        .skip(1)
        .find(|row| row.as_ref().len() != grid[0].as_ref().len())
        .ok_or(())
        .unwrap_err();

    let mut nb_instances = 0;
    for start_y in 0..grid.len() {
        for start_x in 0..grid[0].as_ref().len() {
            if grid[start_y].as_ref()[start_x] == b'X' {
                for direction in [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ] {
                    let mut x = start_x;
                    let mut y = start_y;
                    if [b'M', b'A', b'S'].iter().all(|&c| {
                        x = x.wrapping_add_signed(direction.0);
                        y = y.wrapping_add_signed(direction.1);
                        grid.get(y).and_then(|row| row.as_ref().get(x).copied()) == Some(c)
                    }) {
                        nb_instances += 1;
                    }
                }
            }
        }
    }

    nb_instances
}

fn x_mas_search<Row: AsRef<[u8]> + Debug>(grid: &[Row]) -> usize {
    // All rows should have the same length.
    grid.iter()
        .skip(1)
        .find(|row| row.as_ref().len() != grid[0].as_ref().len())
        .ok_or(())
        .unwrap_err();

    let mut nb_instances = 0;
    for start_y in 0..grid.len() {
        for start_x in 0..grid[0].as_ref().len() {
            if grid[start_y].as_ref()[start_x] == b'A' {
                let get = |dx, dy| {
                    grid.get(start_y.wrapping_add_signed(dy))
                        .and_then(|row| row.as_ref().get(start_x.wrapping_add_signed(dx)).copied())
                };
                let check_diagonal = |dx, dy| match get(dx, dy) {
                    Some(b'M') => get(-dx, -dy) == Some(b'S'),
                    Some(b'S') => get(-dx, -dy) == Some(b'M'),
                    _ => false,
                };
                if check_diagonal(-1, -1) && check_diagonal(-1, 1) {
                    nb_instances += 1;
                }
            }
        }
    }

    nb_instances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const SEARCH: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let grid = parse_grid(SEARCH);
        assert_eq!(word_search(&grid), 18);
        assert_eq!(x_mas_search(&grid), 9);
    }
}
