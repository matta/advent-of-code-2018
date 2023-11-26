use std::collections::HashSet;

fn parse_state(input: &str) -> Vec<bool> {
    Vec::from_iter(input.chars().map(|ch| match ch {
        '#' => true,
        '.' => false,
        _ => panic!("invalid input"),
    }))
}

fn parse(input: &str) -> (Vec<bool>, HashSet<Vec<bool>>) {
    let mut state = Vec::new();
    let mut notes: HashSet<Vec<bool>> = HashSet::new();
    for line in input.lines() {
        if let Some(unparsed_state) = line.strip_prefix("initial state: ") {
            state = parse_state(unparsed_state);
        } else if let Some((unparsed_state, next_state)) = line.split_once(" => ") {
            match next_state {
                "#" => {
                    notes.insert(parse_state(unparsed_state));
                }
                "." => {}
                _ => panic!("invalid input"),
            }
        }
    }

    (state, notes)
}

fn pad(state: &mut Vec<bool>, offset: &mut i32) {
    // Ensure a padding of 3 empty planters to allow patterns to grow plants
    // at the start or end.
    let padding = 3;
    while state.len() < padding || state.iter().take(padding).any(|has_plant| *has_plant) {
        state.insert(0, false);
        *offset -= 1;
    }
    while state
        .iter()
        .skip(state.len() - padding)
        .any(|has_plant| *has_plant)
    {
        state.push(false);
    }
}

pub fn compute(input: &str, generations: i64) -> i64 {
    let (mut state, notes) = parse(input);
    let mut offset = 0;
    let mut sums = Vec::new();

    pad(&mut state, &mut offset);
    state.insert(0, false);
    state.insert(0, false);
    offset -= 2;

    for generation in 1..=generations {
        let mut next = Vec::new();
        next.push(false);
        next.push(false);
        for window in state.windows(5) {
            next.push(notes.contains(window));
        }
        while next[next.len() - 3..next.len()] != [false, false, false] {
            next.push(false);
        }
        state = next;

        let sum = (offset..)
            .zip(state.iter())
            .map(|(pos, plant)| if *plant { pos as i64 } else { 0 })
            .sum::<i64>();
        if sums.len() == 3 {
            sums.remove(0);
        }
        sums.push(sum);
        if sums.len() == 3 && sums[1] - sums[0] == sums[2] - sums[1] {
            let sum_delta = sums[2] - sums[1];
            let remaining_generations = generations - generation;
            sums.push(sum + sum_delta * remaining_generations);
            break;
        }
        if generation > 500 {
            panic!(
                "sum delta did not stabilize within {} generations",
                generation
            );
        }

        pad(&mut state, &mut offset);
    }

    *sums.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");
    const EXAMPLE_INPUT: &str = "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";

    #[test]
    fn test() {
        assert_eq!(compute(EXAMPLE_INPUT, 20), 325);
        assert_eq!(compute(INPUT, 20), 2542);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(compute(EXAMPLE_INPUT, 50_000_000_000), 50_000_000_501);
        assert_eq!(compute(INPUT, 50_000_000_000), 2_550_000_000_883);
    }
}
