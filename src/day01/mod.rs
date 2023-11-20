#![cfg(test)]

use std::collections::HashSet;

fn lines_of_i32_iterator<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a + Clone {
    input.lines().map(|e| e.parse::<i32>().unwrap())
}

fn part_one(input: &str) -> i32 {
    lines_of_i32_iterator(input).sum()
}

fn part_two(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut sum = 0;
    for num in lines_of_i32_iterator(input).cycle() {
        sum += num;
        if !seen.insert(sum) {
            return sum;
        }
    }
    unreachable!();
}

mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 543);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 621);
    }
}
