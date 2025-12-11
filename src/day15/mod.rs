use std::collections::{BTreeSet, VecDeque};

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

    fn as_char(&self) -> char {
        match self {
            Kind::Elf => 'E',
            Kind::Goblin => 'G',
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char(),)
    }
}

#[derive(Debug, Clone, Copy)]
struct Creature {
    kind: Kind,
    health: u8,
    round: u32,
}

impl Creature {
    fn new(kind: Kind) -> Creature {
        Creature {
            kind,
            health: 200,
            round: 0,
        }
    }

    fn as_char(&self) -> char {
        self.kind.as_char()
    }
}

impl std::fmt::Display for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.kind, self.health)
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Cavern,
    Wall,
    Creature(Creature),
}

impl Tile {
    fn new(ch: char) -> Tile {
        match ch {
            '#' => Tile::Wall,
            '.' => Tile::Cavern,
            _ => panic!("invalid input: '{}'", ch),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Tile::Cavern => '.',
            Tile::Wall => '#',
            Tile::Creature(c) => c.as_char(),
        }
    }
}

type Pos = crate::point::Point2D<usize>;

#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    elf_attack_power: u8,
    elf_died: bool,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let tiles = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '#' | '.' => Tile::new(ch),
                        'E' | 'G' => Tile::Creature(Creature::new(Kind::new(ch))),
                        ch => panic!("invalid input: '{}'", ch),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Grid {
            tiles,
            elf_attack_power: 3,
            elf_died: false,
        }
    }

    fn eprint(&self) {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                eprint!("{}", tile.as_char());
            }
            let mut prefix = "    ";
            for tile in row.iter() {
                if let Tile::Creature(c) = tile {
                    eprint!("{}{}", prefix, c.health);
                    prefix = " ";
                }
            }
            eprintln!();
        }
    }

    fn next_creature_position(&self, pos: &Pos) -> Option<Pos> {
        for (x, tile) in self.tiles[pos.y].iter().enumerate().skip(pos.x + 1) {
            if let Tile::Creature(_) = tile {
                return Some(Pos::new(x, pos.y));
            }
        }
        for (y, row) in self.tiles.iter().enumerate().skip(pos.y + 1) {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::Creature(_) = tile {
                    return Some(Pos::new(x, y));
                }
            }
        }
        None
    }

    fn is_cavern(&self, pos: &Pos) -> bool {
        matches!(self.tiles[pos.y][pos.x], Tile::Cavern)
    }

    fn in_range_of_enemies(&self, enemy_kind: Kind) -> BTreeSet<Pos> {
        let mut in_range = BTreeSet::new();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Creature(c) if c.kind == enemy_kind => {
                        let pos = Pos::new(x, y);
                        for neighbor in pos.cardinal_neighbors() {
                            if self.is_cavern(&neighbor) {
                                in_range.insert(neighbor);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        in_range
    }

    fn maybe_attack_in_range_target(&mut self, pos: &Pos) -> bool {
        let attacker_kind = self.creature_kind(pos);
        let enemy_kind = attacker_kind.enemy();
        // eprintln!("   maybe attack {} -> {}", attacker_kind, enemy_kind);

        let mut victim: Option<(Creature, Pos)> = None;
        for neighbor in pos.cardinal_neighbors() {
            if let Tile::Creature(neighbor_creature) = self.tiles[neighbor.y][neighbor.x] {
                if neighbor_creature.kind == enemy_kind {
                    match victim {
                        None => victim = Some((neighbor_creature, neighbor)),
                        Some((victim_creature, _)) => {
                            if neighbor_creature.health < victim_creature.health {
                                victim = Some((neighbor_creature, neighbor));
                            }
                        }
                    }
                }
            }
        }
        if let Some((mut victim, victim_pos)) = victim.take() {
            // eprintln!("   victim: {:?}", victim);
            // eprintln!("   victim pos: {}", victim_pos);
            let attack_power = match attacker_kind {
                Kind::Goblin => 3,
                Kind::Elf => self.elf_attack_power,
            };
            // eprintln!("   attack power: {}", attack_power);
            victim.health = victim.health.saturating_sub(attack_power);
            self.tiles[victim_pos.y][victim_pos.x] = if victim.health > 0 {
                // eprintln!("   victim health: {}", victim.health);
                Tile::Creature(victim)
            } else {
                // eprintln!("   victim dies!");
                if victim.kind == Kind::Elf {
                    self.elf_died = true;
                }
                Tile::Cavern
            };
            true
        } else {
            // eprintln!("   ...no enemy in range");
            false
        }
    }

    fn get(&self, pos: &Pos) -> &Tile {
        &self.tiles[pos.y][pos.x]
    }

    fn get_mut(&mut self, pos: &Pos) -> &mut Tile {
        &mut self.tiles[pos.y][pos.x]
    }

    fn get_creature_mut(&mut self, pos: &Pos) -> &mut Creature {
        match self.get_mut(pos) {
            Tile::Creature(c) => c,
            tile => panic!("position {} is not a creature: {:?}", pos, tile),
        }
    }

    fn creature_kind(&self, pos: &Pos) -> Kind {
        match self.get(pos) {
            Tile::Creature(c) => c.kind,
            tile => panic!("pos {} is not a creature: {:?}", pos, tile),
        }
    }

    fn turn(&mut self, from_pos: &Pos) -> bool {
        let enemy = self.creature_kind(from_pos).enemy();
        // eprintln!("  enemy: {}", enemy);
        if !self.has_any(enemy) {
            // eprintln!("   no enemies!");
            return false;
        }

        if self.maybe_attack_in_range_target(from_pos) {
            return true;
        }

        // TODO: use enemy var
        let in_range_of_enemies = self.in_range_of_enemies(enemy);
        if in_range_of_enemies.is_empty() {
            return true;
        }

        if let Some(to_pos) = self.compute_move_to(in_range_of_enemies, from_pos) {
            self.move_creature(from_pos, &to_pos);
            self.maybe_attack_in_range_target(&to_pos);
        }
        true
    }

    fn move_creature(&mut self, from: &Pos, to: &Pos) {
        // eprintln!("   move {} -> {}", from, to);
        assert_eq!(from.manhattan_distance(*to), 1);
        assert!(matches!(self.get(from), Tile::Creature(_)));
        *self.get_mut(to) = *self.get(from);
        *self.get_mut(from) = Tile::Cavern;
    }

    fn compute_move_to(&self, in_range_of_enemies: BTreeSet<Pos>, from_pos: &Pos) -> Option<Pos> {
        type Cell = Option<(u32, Pos)>;

        let mut flood: Vec<Vec<Cell>> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|_| None).collect())
            .collect();

        let mut queue = VecDeque::new();
        for pos in in_range_of_enemies.into_iter() {
            assert!(self.is_cavern(&pos));
            let wave = Some((0, pos));
            flood[pos.y][pos.x] = wave;
            queue.push_back(pos);
        }

        while let Some(pos) = queue.pop_front() {
            let wave = match flood[pos.y][pos.x] {
                Some((dist, pos)) => Some((dist + 1, pos)),
                _ => panic!("bug: logic error"),
            };
            for adj_pos in pos.cardinal_neighbors() {
                if !self.is_cavern(&adj_pos) {
                    continue;
                }
                let adj = &mut flood[adj_pos.y][adj_pos.x];
                if adj.is_none() || wave < *adj {
                    *adj = wave;
                    queue.push_back(adj_pos);
                }
            }
        }

        from_pos
            .cardinal_neighbors()
            .filter_map(|pos| {
                if let Some((dist, target_pos)) = flood[pos.y][pos.x] {
                    Some((dist, target_pos, pos))
                } else {
                    None
                }
            })
            .min()
            .map(|(_dist, _target_pos, pos)| pos)
    }

    fn round(&mut self, round: u32) -> bool {
        let mut pos = Pos::default();
        while let Some(next_pos) = self.next_creature_position(&pos) {
            pos = next_pos;

            let creature = self.get_creature_mut(&pos);
            if creature.round >= round {
                continue;
            }
            creature.round = round;

            let took_action = self.turn(&pos);
            if !took_action {
                return false;
            }

            // In part two bail early if an Elf dies.
            if self.elf_attack_power > 3 && self.elf_died {
                return false;
            }
        }
        true
    }

    fn creature_iter(&self) -> impl Iterator<Item = &Creature> {
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|e| {
                if let Tile::Creature(c) = e {
                    Some(c)
                } else {
                    None
                }
            })
    }

    fn total_hit_points(&self) -> u32 {
        self.creature_iter().map(|c| c.health as u32).sum()
    }

    fn count_kind(&self, kind: Kind) -> usize {
        self.creature_iter().filter(|c| c.kind == kind).count()
    }

    fn has_any(&self, kind: Kind) -> bool {
        self.creature_iter().any(|c| c.kind == kind)
    }

    fn battle(&mut self) -> u32 {
        let trace = false;
        let mut rounds = 0;
        if trace {
            eprintln!("{}", rounds);
            self.eprint();
        }
        while self.round(rounds + 1) {
            rounds += 1;
            if trace {
                eprintln!("{}", rounds);
                self.eprint();
            }
            assert_ne!(rounds, 200);
        }
        if trace {
            eprintln!("Finished:");
            self.eprint();
        }
        let total_hit_points = self.total_hit_points();
        dbg!(rounds);
        dbg!(total_hit_points);
        dbg!(rounds * total_hit_points);
        rounds * total_hit_points
    }
}

pub fn compute_part_one(input: &str) -> u32 {
    Grid::parse(input).battle()
}

pub fn compute_part_two(input: &str) -> u32 {
    let g = Grid::parse(input);
    let elf_count = g.count_kind(Kind::Elf);

    let mut low: u16 = 4;
    let mut high: u16 = 200;
    let mut lowest_outcome = None;

    while low <= high {
        let power = (low + high) / 2;
        eprintln!("Elf damage: {}", power);

        let mut g = g.clone();
        g.elf_attack_power = power.try_into().unwrap();
        let outcome = g.battle();
        if g.count_kind(Kind::Elf) == elf_count {
            eprintln!("attack power {} worked!", power);
            lowest_outcome = Some(outcome);
            high = power - 1;
        } else {
            low = power + 1;
        }
    }

    lowest_outcome.expect("bug: never found an attack power that caused an Elf victory")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    const EXAMPLE_FIRST: &str = r#"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
"#;
    #[test]
    fn test_part_one_example_first() {
        assert_eq!(compute_part_one(EXAMPLE_FIRST), 27730);
    }
    #[test]
    fn test_part_two_example_first() {
        assert_eq!(compute_part_two(EXAMPLE_FIRST), 4988);
    }

    const EXAMPLE_SECOND: &str = r#"
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
"#;
    #[test]
    fn test_part_one_example_second() {
        assert_eq!(compute_part_one(EXAMPLE_SECOND), 36334);
    }

    const EXAMPLE_THIRD: &str = r#"
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
"#;
    #[test]
    fn test_part_one_example_third() {
        assert_eq!(compute_part_one(EXAMPLE_THIRD), 39514);
    }

    #[test]
    fn test_part_two_example_third() {
        assert_eq!(compute_part_two(EXAMPLE_THIRD), 31284);
    }

    // Combat ends after 35 full rounds
    // Goblins win with 793 total hit points left
    // Outcome: 35 * 793 = 27755
    const EXAMPLE_FOURTH: &str = r#"
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
"#;
    #[test]
    fn test_part_one_example_fourth() {
        assert_eq!(compute_part_one(EXAMPLE_FOURTH), 27755);
    }
    #[test]
    fn test_part_two_example_fourth() {
        assert_eq!(compute_part_two(EXAMPLE_FOURTH), 3478);
    }

    // Combat ends after 54 full rounds
    // Goblins win with 536 total hit points left
    // Outcome: 54 * 536 = 28944
    const EXAMPLE_FIFTH: &str = r#"
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
"#;
    #[test]
    fn test_part_one_example_fifth() {
        assert_eq!(compute_part_one(EXAMPLE_FIFTH), 28944);
    }

    #[test]
    fn test_part_two_example_fifth() {
        assert_eq!(compute_part_two(EXAMPLE_FIFTH), 6474);
    }

    // Combat ends after 20 full rounds
    // Goblins win with 937 total hit points left
    // Outcome: 20 * 937 = 18740
    const EXAMPLE_SIXTH: &str = r#"
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
"#;

    #[test]
    fn test_part_one_example_sixth() {
        assert_eq!(compute_part_one(EXAMPLE_SIXTH), 18740);
    }

    #[test]
    fn test_part_two_example_sixth() {
        assert_eq!(compute_part_two(EXAMPLE_SIXTH), 1140);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(compute_part_one(INPUT), 248235);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(compute_part_two(INPUT), 46784);
    }
}
