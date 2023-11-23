use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

use crate::point::Point2D;

type Point = Point2D<i32>;

fn parse_points(input: &str) -> Vec<Point> {
    let mut points: Vec<Point> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(", ").unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    points.sort();
    points
}

#[derive(Debug)]
enum Claim {
    Min(i32, usize),
    Tie(i32),
}

pub fn part_one(input: &str) -> i32 {
    let points = parse_points(input);

    let (min, max) = bounds(&points);

    let mut claim_counts = vec![0; points.len()];
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let here = Point::new(x, y);
            let initial = Claim::Min(here.manhattan_distance(points[0]), 0);
            let min = points
                .iter()
                .enumerate()
                .skip(1)
                .fold(initial, |claim, (i, point)| {
                    let claim_dist = match claim {
                        Claim::Min(dist, _) => dist,
                        Claim::Tie(dist) => dist,
                    };
                    let dist = here.manhattan_distance(*point);
                    match dist.cmp(&claim_dist) {
                        Ordering::Less => Claim::Min(dist, i),
                        Ordering::Equal => Claim::Tie(dist),
                        Ordering::Greater => claim,
                    }
                });
            if let Claim::Min(_, index) = min {
                claim_counts[index] += 1;
            }
        }
    }

    let max_claim_count = claim_counts
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            let point = points[*index];
            point.x != min.x && point.x != max.x && point.y != min.y && point.y != max.y
        })
        .map(|(_, count)| *count)
        .max()
        .unwrap();

    dbg!((max.x - min.x) * (max.y - min.y));
    dbg!(claim_counts.iter().sum::<i32>());
    dbg!(&max_claim_count);

    max_claim_count
}

pub fn part_two(input: &str) -> usize {
    let points = parse_points(input);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    let (min, max) = bounds(&points);

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let here = Point::new(x, y);
            seen.insert(here);
            queue.push_back(here);
        }
    }

    let mut region_count = 0;
    while let Some(here) = queue.pop_front() {
        let sum = points
            .iter()
            .map(|there| here.manhattan_distance(*there))
            .sum::<i32>();
        if sum < 10_000 {
            region_count += 1;
            for neighbor in here.neighbors() {
                if seen.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    region_count
}

fn bounds(points: &[Point]) -> (Point, Point) {
    let mut min = points[0];
    let mut max = points[0];
    for p in points.iter() {
        min.x = min.x.min(p.x);
        min.y = min.y.min(p.y);
        max.x = max.x.max(p.x);
        max.y = max.y.max(p.y);
    }
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 4475);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 35237);
    }
}
