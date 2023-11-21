#![cfg(test)]

use std::collections::HashSet;

use lazy_regex::regex_captures;

#[derive(Debug)]
struct Claim {
    id: u16,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

// Parse lines of the form: #1 @ 393,863: 11x29
fn parse_claim(line: &str) -> Claim {
    let (_, id, x, y, width, height) =
        regex_captures!(r#"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"#, line).expect("bad input line");
    Claim {
        id: id.parse().unwrap(),
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
        width: width.parse().unwrap(),
        height: height.parse().unwrap(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ClaimedInch {
    Available,
    Disputed,
    Claimed(u16),
}

fn compute(input: &str) -> (usize, u16) {
    let claims: Vec<Claim> = input.lines().map(parse_claim).collect();
    let max_x = claims.iter().map(|c| c.x + c.width).max().unwrap();
    let max_y = claims.iter().map(|c| c.y + c.height).max().unwrap();
    let mut coverage = vec![ClaimedInch::Available; max_x * max_y];

    let mut disputed_ids = HashSet::new();

    for claim in claims.iter() {
        for y in claim.y..(claim.y + claim.height) {
            let start = max_x * y + claim.x;
            let end = start + claim.width;
            for c in &mut coverage[start..end] {
                match c {
                    ClaimedInch::Available => {
                        *c = ClaimedInch::Claimed(claim.id);
                    }
                    ClaimedInch::Disputed => {
                        disputed_ids.insert(claim.id);
                    }
                    ClaimedInch::Claimed(id) => {
                        disputed_ids.insert(*id);
                        disputed_ids.insert(claim.id);
                        *c = ClaimedInch::Disputed;
                    }
                }
            }
        }
    }

    let all_ids = HashSet::from_iter(claims.iter().map(|c| c.id));

    let disputed_count = coverage
        .iter()
        .filter(|c| **c == ClaimedInch::Disputed)
        .count();

    let undisputed_ids: Vec<_> = all_ids.difference(&disputed_ids).copied().collect();
    assert_eq!(1, undisputed_ids.len());
    let undisputed_id = undisputed_ids[0];

    (disputed_count, undisputed_id)
}

mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test() {
        let (disputed_count, undisputed_id) = compute(INPUT);
        assert_eq!(disputed_count, 98005);
        assert_eq!(undisputed_id, 331);
    }
}
