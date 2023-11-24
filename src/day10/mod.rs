use std::collections::HashSet;

use itertools::{Itertools, MinMaxResult};
use lazy_regex::regex_captures;

use crate::point::Point2D;

type Point = Point2D<i32>;

#[derive(Debug, Clone, Copy)]
struct MovingPoint {
    position: Point,
    velocity: Point,
}

impl MovingPoint {
    fn step(&self) -> MovingPoint {
        MovingPoint {
            position: self.position + self.velocity,
            velocity: self.velocity,
        }
    }
}

// Parse lines of the form:
// position=< 21188,  31669> velocity=<-2, -3>
fn parse_line(line: &str) -> MovingPoint {
    let (_, px, py, dx, dy) = regex_captures!(
        r#"position=<\s*(-?\d+)\s*,\s*(-?\d+)\s*>\s*velocity=<\s*(-?\d+)\s*,\s*(-?\d+)\s*>"#,
        line
    )
    .unwrap();

    MovingPoint {
        position: Point::new(px.parse().unwrap(), py.parse().unwrap()),
        velocity: Point::new(dx.parse().unwrap(), dy.parse().unwrap()),
    }
}

fn parse_lines(input: &str) -> Vec<MovingPoint> {
    input.lines().map(parse_line).collect()
}

fn compute_bounds(points: &[MovingPoint]) -> (Point, Point) {
    let x = points.iter().map(|e| e.position.x).minmax();
    let y = points.iter().map(|e| e.position.y).minmax();
    if let (MinMaxResult::MinMax(min_x, max_x), MinMaxResult::MinMax(min_y, max_y)) = (x, y) {
        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    } else {
        unreachable!()
    }
}

fn compute_area(points: &[MovingPoint]) -> i64 {
    let (min, max) = compute_bounds(points);
    (max.x - min.x) as i64 * (max.y - min.y) as i64
}

fn step(points: &[MovingPoint]) -> Vec<MovingPoint> {
    points.iter().map(|point| point.step()).collect()
}

fn format_points(points: &[MovingPoint]) -> String {
    let (min, max) = compute_bounds(points);
    dbg!(min, max);

    let positions: HashSet<Point> = HashSet::from_iter(points.iter().map(|p| p.position));

    let mut s = String::new();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let ch = if positions.contains(&Point::new(x, y)) {
                'X'
            } else {
                '.'
            };
            s.push(ch);
        }
        s.push('\n');
    }
    s
}

pub fn compute(input: &str) -> (String, i32) {
    let mut points = parse_lines(input);
    let mut area = compute_area(&points);
    let mut seconds = 0;
    loop {
        let next = step(&points);
        let next_area = compute_area(&next);
        if next_area > area {
            break;
        }
        points = next;
        area = next_area;
        seconds += 1;
    }

    (format_points(&points), seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test() {
        let expected_answer = "
X....X..XXXXX...XXXXXX..X....X..X....X..X....X..X....X..X.....
X....X..X....X..X.......X....X..X....X..X....X..X...X...X.....
.X..X...X....X..X........X..X....X..X....X..X...X..X....X.....
.X..X...X....X..X........X..X....X..X....X..X...X.X.....X.....
..XX....XXXXX...XXXXX.....XX......XX......XX....XX......X.....
..XX....X.......X.........XX......XX......XX....XX......X.....
.X..X...X.......X........X..X....X..X....X..X...X.X.....X.....
.X..X...X.......X........X..X....X..X....X..X...X..X....X.....
X....X..X.......X.......X....X..X....X..X....X..X...X...X.....
X....X..X.......X.......X....X..X....X..X....X..X....X..XXXXXX
";
        let (answer, seconds) = compute(INPUT);
        assert_eq!(answer, expected_answer.trim_start());
        assert_eq!(seconds, 10521);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(compute_part_two(INPUT), 22989);
    // }
}
