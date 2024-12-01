use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = BufReader::new(File::open("input/1").unwrap());

    let (left, right) = process_lines(f.lines().map(|line| line.unwrap()));
    println!("Distance = {}", distance(&left, &right));
    println!("Similarity = {}", similarity(&left, &right));
}

fn process_lines<S: AsRef<str>, It: Iterator<Item = S>>(lines: It) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = lines
        .map(|line| {
            let (left, right) = line
                .as_ref()
                .split_once(|c: char| c.is_whitespace())
                .unwrap();
            (
                left.trim().parse::<u32>().expect(left),
                right.trim().parse::<u32>().expect(right),
            )
        })
        .collect();
    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

fn distance(left: &[u32], right: &[u32]) -> u32 {
    std::iter::zip(left, right).fold(0, |sum, (&left, &right)| sum + left.abs_diff(right))
}

fn similarity(left: &[u32], right: &[u32]) -> u32 {
    let mut right_counts = vec![];
    for &n in right {
        if right_counts.len() <= n as usize {
            right_counts.resize(n as usize + 1, 0);
        }
        right_counts[n as usize] += 1;
    }

    left.iter()
        .fold(0, |sum, &n| sum + n * right_counts[n as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const DATA: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
        let (left, right) = process_lines(DATA.lines());
        assert_eq!(distance(&left, &right), 11);
        assert_eq!(similarity(&left, &right), 31);
    }
}
