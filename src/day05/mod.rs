#![cfg(test)]

fn reduce(input: &str, omit_unit: Option<u8>) -> usize {
    let polymer = input.trim().as_bytes().to_vec();
    let reduced = polymer.iter().fold(Vec::new(), |mut reduced, &next_unit| {
        if omit_unit
            .map(|omit_unit| omit_unit.to_ascii_lowercase() == next_unit.to_ascii_lowercase())
            .unwrap_or(false)
        {
            return reduced;
        }
        match reduced.last().copied() {
            None => {
                reduced.push(next_unit);
                return reduced;
            }
            Some(prev_unit) => {
                if prev_unit.to_ascii_lowercase() == next_unit.to_ascii_lowercase()
                    && prev_unit != next_unit
                {
                    // React by destroying both units.
                    reduced.pop();
                    return reduced;
                } else {
                    reduced.push(next_unit);
                    return reduced;
                }
            }
        }
    });

    reduced.len()
}

fn part_one(input: &str) -> usize {
    reduce(input, None)
}

fn part_two(input: &str) -> usize {
    (b'a'..=b'z')
        .map(|unit| reduce(input, Some(unit)))
        .min()
        .unwrap()
}

mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 10878);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 6874);
    }
}
