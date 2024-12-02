use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reports: Vec<_> = BufReader::new(File::open("input/2").unwrap())
        .lines()
        .map(|line| parse_report(&line.unwrap()))
        .collect();
    println!(
        "{} are safe",
        reports.iter().filter(|report| is_safe(report)).count()
    );
    println!(
        "{} are safe when dampened",
        reports
            .iter()
            .filter(|report| is_safe_dampened(report))
            .count()
    );
}

fn parse_report(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn is_safe(report: &[u8]) -> bool {
    if report[0] < report[1] {
        report.windows(2).all(|pair| {
            let (left, right) = (pair[0], pair[1]);
            matches!(right.checked_sub(left), Some(1..=3))
        })
    } else {
        report.windows(2).all(|pair| {
            let (left, right) = (pair[0], pair[1]);
            matches!(left.checked_sub(right), Some(1..=3))
        })
    }
}

fn is_safe_dampened(report: &[u8]) -> bool {
    let mut dampened = vec![0; report.len() - 1];
    is_safe(report)
        || (0..report.len()).any(|idx| {
            dampened[..idx].copy_from_slice(&report[..idx]);
            dampened[idx..].copy_from_slice(&report[idx + 1..]);
            is_safe(&dampened)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        fn check(s: &str, safe: bool, safe_dampened: bool) {
            let report = parse_report(s);
            assert_eq!(is_safe(&report), safe);
            assert_eq!(is_safe_dampened(&report), safe_dampened);
        }

        check("7 6 4 2 1", true, true);
        check("1 2 7 8 9", false, false);
        check("9 7 6 2 1", false, false);
        check("1 3 2 4 5", false, true);
        check("8 6 4 4 1", false, true);
        check("1 3 6 7 9", true, true);
    }
}
