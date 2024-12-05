use topological_sort::{DependencyLink, TopologicalSort};

fn main() {
    let (rules, mut updates) = parse_sections(&std::fs::read_to_string("input/5").unwrap());

    let sum = updates
        .iter()
        .filter(|update| check_update(update, &rules))
        .fold(0, |sum, update| sum + update[update.len() / 2] as u16);
    println!("Correct updates: {sum}");

    let sum = updates
        .iter_mut()
        .filter(|update| !check_update(update, &rules))
        .fold(0, |sum, update| {
            sum + correct_update(update, &rules)[update.len() / 2] as u16
        });
    println!("Incorrect updates: {sum}");
}

fn parse_sections(s: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let mut lines = s.lines();
    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (before, after) = line.split_once('|').expect(line);
            (before.parse().expect(before), after.parse().expect(after))
        })
        .collect();
    let updates = lines
        .map(|line| {
            line.split(',')
                .map(|page| page.parse().expect(page))
                .collect()
        })
        .collect();
    (rules, updates)
}

fn check_update(update: &[u8], rules: &[(u8, u8)]) -> bool {
    let mut seen_after = vec![false; rules.len()];
    for &page in update {
        for ((before, after), have_seen_after) in std::iter::zip(rules, &mut seen_after) {
            if *after == page {
                *have_seen_after = true;
            }
            if *have_seen_after && *before == page {
                // We are seeing `before` after `after`, violating the rule.
                return false;
            }
        }
    }
    true
}

fn correct_update(update: &[u8], rules: &[(u8, u8)]) -> Vec<u8> {
    let topo_sort: TopologicalSort<u8> = rules
        .iter()
        .filter(|(before, after)| update.contains(before) && update.contains(after))
        .map(|&(prec, succ)| DependencyLink { prec, succ })
        .collect();
    let corrected: Vec<_> = topo_sort.collect();
    assert!(check_update(&corrected, rules));
    corrected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        static EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let (rules, updates) = parse_sections(EXAMPLE);
        assert_eq!(
            updates
                .iter()
                .map(|update| check_update(update, &rules))
                .collect::<Vec<_>>(),
            [true, true, true, false, false, false]
        );

        assert_eq!(&correct_update(&updates[3], &rules), &[97, 75, 47, 61, 53]);
        assert_eq!(&correct_update(&updates[4], &rules), &[61, 29, 13]);
        assert_eq!(&correct_update(&updates[5], &rules), &[97, 75, 47, 29, 13]);
    }
}
