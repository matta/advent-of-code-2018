use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Kind {
    Elf,
    Goblin,
}

impl Kind {
    fn new(ch: char) -> Kind {
        match ch {
            'E' => Kind::Elf,
            'G' => Kind::Goblin,
            _ => panic!("invalid input: '{}'", ch),
        }
    }

    fn enemy(&self) -> Kind {
        match *self {
            Kind::Elf => Kind::Goblin,
            Kind::Goblin => Kind::Elf,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Creature {
    kind: Kind,
    health: u8,
}

impl Creature {
    fn new(kind: Kind) -> Creature {
        Creature { kind, health: 200 }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Cavern,
    Wall,
}

impl Tile {
    fn new(ch: char) -> Tile {
        match ch {
            '#' => Tile::Wall,
            '.' => Tile::Cavern,
            _ => panic!("invalid input: '{}'", ch),
        }
    }
}

type Pos = crate::point::Point2D<usize>;

struct Grid {
    tiles: Vec<Vec<Tile>>,
    creatures: BTreeMap<Pos, Creature>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut creatures = BTreeMap::new();
        let tiles = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        '#' | '.' => Tile::new(ch),
                        'E' | 'G' => {
                            creatures.insert(Pos::new(x, y), Creature::new(Kind::new(ch)));
                            Tile::Cavern
                        }
                        ch => panic!("invalid input: '{}'", ch),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Grid { tiles, creatures }
    }

    fn eprint(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            let mut creatures = Vec::new();
            for (x, tile) in row.iter().enumerate() {
                let pos = Pos::new(x, y);
                let ch = if let Some(creature) = self.creatures.get(&pos) {
                    creatures.push(creature);
                    match creature.kind {
                        Kind::Elf => 'E',
                        Kind::Goblin => 'G',
                    }
                } else {
                    match tile {
                        Tile::Cavern => '.',
                        Tile::Wall => '#',
                    }
                };
                eprint!("{}", ch);
            }
            let mut prefix = "    ";
            for c in creatures {
                eprint!(
                    "{}{}({})",
                    prefix,
                    match c.kind {
                        Kind::Elf => 'E',
                        Kind::Goblin => 'G',
                    },
                    c.health
                );
                prefix = ", ";
            }
            eprintln!();
        }
    }

    fn next_creature_position(&self, pos: &Pos) -> Option<Pos> {
        // What we want is a "lower bound" type API on BTreeMap,  which would
        // run in O(log N) time, but this is currently not stabilized so let's
        // do it the slow way in O(N) time.
        //
        // See https://github.com/rust-lang/rust/issues/107540
        for k in self.creatures.keys() {
            if k > pos {
                return Some(*k);
            }
        }
        None
    }

    fn is_cavern(&self, pos: &Pos) -> bool {
        self.tiles[pos.y][pos.x] == Tile::Cavern && !self.creatures.contains_key(pos)
    }

    fn in_range_of_enemies(&self, enemy_kind: Kind) -> BTreeSet<Pos> {
        let mut in_range = BTreeSet::new();
        for (pos, creature) in self.creatures.iter() {
            if creature.kind != enemy_kind {
                continue;
            }
            for neighbor in pos.cardinal_neighbors() {
                if self.is_cavern(&neighbor) {
                    in_range.insert(neighbor);
                }
            }
        }
        in_range
    }

    fn maybe_attack_in_range_target(&mut self, pos: Pos) -> bool {
        let enemy_kind = self.creature_kind(&pos).enemy();

        let mut candidates = pos
            .cardinal_neighbors()
            .filter_map(|neighbor| match self.creatures.get(&neighbor) {
                Some(Creature { kind, health }) if *kind == enemy_kind => Some((*health, neighbor)),
                Some(_) | None => None,
            })
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            return false;
        }

        candidates.sort_unstable();
        let (victim_health, victim_pos) = candidates[0];
        if victim_health > 3 {
            self.creatures.get_mut(&victim_pos).unwrap().health -= 3;
        } else {
            assert!(self.creatures.remove(&victim_pos).is_some());
        }

        true
    }

    fn creature_kind(&self, pos: &Pos) -> Kind {
        self.creatures.get(pos).unwrap().kind
    }

    fn do_turn(&mut self, processed: &mut BTreeSet<Pos>, from_pos: Pos) -> bool {
        if processed.contains(&from_pos) {
            return true;
        }

        let enemy = self.creature_kind(&from_pos).enemy();
        if !self.creatures.values().any(|c| c.kind == enemy) {
            return false;
        }

        processed.insert(from_pos);
        if self.maybe_attack_in_range_target(from_pos) {
            return true;
        }

        let in_range_of_enemies = self.in_range_of_enemies(self.creature_kind(&from_pos).enemy());
        if in_range_of_enemies.is_empty() {
            return true;
        }

        if let Some(to_pos) = self.compute_move_to(in_range_of_enemies, from_pos) {
            assert!(processed.remove(&from_pos));
            assert!(processed.insert(to_pos));
            assert_eq!(from_pos.manhattan_distance(to_pos), 1);
            let creature = self.creatures.remove(&from_pos).unwrap();
            let inserted = self.creatures.insert(to_pos, creature).is_none();
            assert!(inserted);

            self.maybe_attack_in_range_target(to_pos);
        }
        true
    }

    fn compute_move_to(&self, in_range_of_enemies: BTreeSet<Pos>, from_pos: Pos) -> Option<Pos> {
        let mut queue = VecDeque::from_iter(in_range_of_enemies.into_iter().map(|pos| (0, pos)));
        let mut seen: HashSet<Pos> = HashSet::from_iter(queue.iter().map(|(_, pos)| *pos));
        let mut moves = Vec::new();

        while let Some((distance, pos)) = queue.pop_front() {
            for neighbor in pos.cardinal_neighbors() {
                if neighbor == from_pos {
                    moves.push((distance, pos));
                    // XXX: if moves.len() is 4 we can stop.
                    // XXX: ...or even less if from_pos has fewer accessible
                    //      neighbors.
                } else if self.is_cavern(&neighbor) && seen.insert(neighbor) {
                    queue.push_back((distance + 1, neighbor))
                }
            }
        }

        moves.sort_unstable();
        moves.first().map(|(_, pos)| *pos)
    }

    fn round(&mut self) -> bool {
        let mut pos = Pos::default();
        let mut processed = BTreeSet::new();
        while let Some(next_pos) = self.next_creature_position(&pos) {
            pos = next_pos;
            if !self.do_turn(&mut processed, pos) {
                return false;
            }
        }
        true
    }
}

pub fn compute_part_one(input: &str) -> u32 {
    let mut g = Grid::parse(input);

    eprintln!("Initially:");
    g.eprint();

    let mut round = 0;
    while g.round() {
        round += 1;
        // eprintln!("After {} round(s):", round);
        // g.eprint();
    }
    let total_hit_points = g
        .creatures
        .values()
        .map(|creature| {
            let hp: u32 = creature.health.into();
            hp
        })
        .sum::<u32>();
    dbg!(round);
    dbg!(total_hit_points);
    dbg!(round * total_hit_points);
    round * total_hit_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_a() {
        const EXAMPLE_A: &str = include_str!("example_a.txt");
        assert_eq!(compute_part_one(EXAMPLE_A), 27828);
    }

    #[test]
    fn test_part_one_example_b() {
        const EXAMPLE_B: &str = include_str!("example_b.txt");
        assert_eq!(compute_part_one(EXAMPLE_B), 27730);
    }

    #[test]
    fn test_part_one() {
        const INPUT: &str = include_str!("input.txt");
        assert_eq!(compute_part_one(INPUT), 248235);
    }
}
