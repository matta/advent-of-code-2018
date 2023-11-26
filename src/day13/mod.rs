use std::collections::BTreeMap;

use crate::point::{CardinalDirection, Point2D};

type Point = Point2D<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Turn {
        match *self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }

    fn applied(&self, dir: CardinalDirection) -> CardinalDirection {
        match *self {
            Turn::Left => dir.left(),
            Turn::Right => dir.right(),
            Turn::Straight => dir,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cart {
    current_direction: CardinalDirection,
    next_turn: Turn,
}

impl Cart {
    fn turn(&self, turn: Turn) -> Cart {
        Cart {
            current_direction: turn.applied(self.current_direction),
            next_turn: self.next_turn,
        }
    }

    fn choose_turn(&self) -> Cart {
        Cart {
            current_direction: self.next_turn.applied(self.current_direction),
            next_turn: self.next_turn.next(),
        }
    }
}

type Carts = BTreeMap<Point, Cart>;

#[allow(dead_code)]
fn print_map(map: &[Vec<u8>], carts: &Carts) {
    println!("Map:");
    for (y, line) in map.iter().enumerate() {
        for (x, byte) in line.iter().enumerate() {
            let ch = if let Some(cart) = carts.get(&Point::new(x, y)) {
                match cart.current_direction {
                    CardinalDirection::North => '^',
                    CardinalDirection::South => 'v',
                    CardinalDirection::East => '>',
                    CardinalDirection::West => '<',
                }
            } else {
                *byte as char
            };
            print!("{}", ch);
        }
        println!();
    }
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Carts) {
    let mut carts = BTreeMap::new();
    let map: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .cloned()
                .enumerate()
                .map(|(x, byte)| match byte {
                    b' ' | b'/' | b'\\' | b'-' | b'|' | b'+' => byte,
                    b'^' | b'v' | b'<' | b'>' => {
                        let dir = match byte {
                            b'^' => CardinalDirection::North,
                            b'v' => CardinalDirection::South,
                            b'<' => CardinalDirection::West,
                            b'>' => CardinalDirection::East,
                            _ => unreachable!(),
                        };
                        let replacement_byte = match byte {
                            b'^' | b'v' => b'|',
                            b'<' | b'>' => b'-',
                            _ => unreachable!(),
                        };
                        carts.insert(
                            Point::new(x, y),
                            Cart {
                                current_direction: dir,
                                next_turn: Turn::Left,
                            },
                        );
                        replacement_byte
                    }
                    _ => panic!("Invalid byte in map: {}", byte),
                })
                .collect::<Vec<u8>>()
        })
        .collect();
    (map, carts)
}

fn tick(map: &[Vec<u8>], carts: &Carts) -> (Option<Point>, Carts) {
    let mut next_carts = carts.clone();
    let mut first_collision_at = None;
    for (from_pos, from_cart) in carts {
        if next_carts.remove(from_pos).is_none() {
            // This cart was removed in a prior collision.
            assert!(first_collision_at.is_some());
            continue;
        }
        let next_pos = from_pos.cardinal_neighbor(from_cart.current_direction);
        let next_cart = match (map[next_pos.y][next_pos.x], from_cart.current_direction) {
            (b'|', CardinalDirection::North | CardinalDirection::South) => *from_cart,
            (b'-', CardinalDirection::West | CardinalDirection::East) => *from_cart,
            (b'/', CardinalDirection::North | CardinalDirection::South) => {
                from_cart.turn(Turn::Right)
            }
            (b'/', CardinalDirection::East | CardinalDirection::West) => from_cart.turn(Turn::Left),
            (b'\\', CardinalDirection::North | CardinalDirection::South) => {
                from_cart.turn(Turn::Left)
            }
            (b'\\', CardinalDirection::East | CardinalDirection::West) => {
                from_cart.turn(Turn::Right)
            }
            (b'+', _) => from_cart.choose_turn(),
            invalid => panic!("invalid cart state: {:?}", invalid),
        };
        if next_carts.insert(next_pos, next_cart).is_some() {
            if first_collision_at.is_none() {
                first_collision_at = Some(next_pos);
            }
            next_carts.remove(&next_pos);
        }
    }

    (first_collision_at, next_carts)
}

pub fn compute_part_one(input: &str) -> Point {
    let (map, mut carts) = parse(input);
    let mut ticks = 1;
    loop {
        // print_map(&map, &carts);
        let (possible_collision, next_carts) = tick(&map, &carts);
        if let Some(pos) = possible_collision {
            return pos;
        }
        carts = next_carts;
        ticks += 1;
        if ticks > 200 {
            unreachable!("looped for too many ticks: {}", ticks);
        }
    }
}

pub fn compute_part_two(input: &str) -> Point {
    let (map, mut carts) = parse(input);
    let mut ticks = 1;

    // Cart count must be odd for the loop to terminate.
    assert!(carts.len() % 2 == 1);

    loop {
        // print_map(&map, &carts);
        let (_, next_carts) = tick(&map, &carts);
        carts = next_carts;
        if carts.len() == 1 {
            return *carts.keys().next().unwrap();
        }
        // println!("Step: {}  Cart count: {}", ticks, carts.len());
        ticks += 1;
        if ticks > 100_000 {
            unreachable!("looped for too many ticks: {}", ticks);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");
    const EXAMPLE_INPUT: &str = r#"
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
"#;

    #[test]
    fn test_part_one_example() {
        assert_eq!(compute_part_one(EXAMPLE_INPUT), Point::new(7, 3));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(compute_part_one(INPUT), Point::new(115, 138));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(compute_part_two(INPUT), Point::new(0, 98));
    }
}
