#![cfg(test)]

use std::collections::HashMap;

fn part_one_counts(id: &str) -> (bool, bool) {
    let mut counts = HashMap::new();
    for ch in id.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    let has_count = |desired_count| {
        counts
            .values()
            .find(|count| **count == desired_count)
            .is_some()
    };

    (has_count(2), has_count(3))
}

fn part_one(input: &str) -> u32 {
    let (has2_count, has3_count) = input
        .lines()
        .map(part_one_counts)
        .fold((0, 0), |acc, elem| {
            (acc.0 + elem.0 as u32, acc.1 + elem.1 as u32)
        });
    return has2_count * has3_count;
}

fn part_two(input: &str) -> String {
    let codes: Vec<_> = input.lines().collect();

    for (i, i_code) in codes.iter().enumerate() {
        for j_code in codes.iter().skip(i + 1) {
            assert_eq!(i_code.len(), j_code.len());
            let different: Vec<_> = i_code
                .chars()
                .zip(j_code.chars())
                .enumerate()
                .filter_map(|(index, (i_ch, j_ch))| if i_ch != j_ch { Some(index) } else { None })
                .collect();
            let mut ret = i_code.to_string();
            if different.len() == 1 {
                ret.remove(different[0]);
                return ret;
            }
        }
    }
    unreachable!()
}

mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 4712);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), "lufjygedpvfbhftxiwnaorzmq");
    }
}
