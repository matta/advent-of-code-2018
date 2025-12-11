fn compute(input: &str, compute: fn(&mut dyn Iterator<Item = i32>) -> i32) -> i32 {
    let mut it = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    compute(&mut it)
}

fn sum_metadata(it: &mut dyn Iterator<Item = i32>) -> i32 {
    match (it.next(), it.next()) {
        (Some(child_count), Some(metadata_count)) => {
            let child_sum = (0..child_count).map(|_| sum_metadata(it)).sum::<i32>();
            let metadata_sum = (0..metadata_count).map(|_| it.next().unwrap()).sum::<i32>();
            child_sum + metadata_sum
        }
        _ => panic!("parsing error"),
    }
}

fn node_value(it: &mut dyn Iterator<Item = i32>) -> i32 {
    match (it.next(), it.next()) {
        (Some(child_count), Some(metadata_count)) => {
            let child_sums: Vec<i32> = (0..child_count).map(|_| node_value(it)).collect();
            if child_sums.is_empty() {
                (0..metadata_count).map(|_| it.next().unwrap()).sum::<i32>()
            } else {
                (0..metadata_count)
                    .map(|_| {
                        let index = it.next().unwrap() - 1;
                        if let Some(count) = child_sums.get(index as usize) {
                            *count
                        } else {
                            0
                        }
                    })
                    .sum::<i32>()
            }
        }
        _ => panic!("parsing error"),
    }
}

pub fn compute_part_one(input: &str) -> i32 {
    compute(input, sum_metadata)
}

pub fn compute_part_two(input: &str) -> i32 {
    compute(input, node_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(compute_part_one(INPUT), 45194);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(compute_part_two(INPUT), 22989);
    }
}
