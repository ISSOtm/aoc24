use regex::{Captures, Regex};

fn main() {
    let input = std::fs::read_to_string("input/3").unwrap();

    fn parse_instr_operands(captures: &Captures) -> u32 {
        let lhs: u32 = captures[1].parse().unwrap();
        let rhs: u32 = captures[2].parse().unwrap();
        lhs * rhs
    }

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = regex
        .captures_iter(&input)
        .fold(0, |sum, captures| sum + parse_instr_operands(&captures));
    println!("Sum = {sum}");

    let full_regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let (sum, _) = full_regex
        .captures_iter(&input)
        .fold((0, true), |(sum, enabled), captures| match &captures[0] {
            "do()" => (sum, true),
            "don't()" => (sum, false),
            _ => (
                if enabled {
                    sum + parse_instr_operands(&captures)
                } else {
                    sum
                },
                enabled,
            ),
        });
    println!("Enabled sum = {sum}");
}
