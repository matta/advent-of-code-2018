use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
struct Circle {
    // Clockwise is from front to back. The current marble is at the
    // front of the deque.
    deque: VecDeque<i64>,
}

impl Circle {
    fn rotate_clockwise(&mut self, n: usize) {
        for _ in 0..n {
            self.deque.rotate_left(1);
        }
    }

    fn rotate_counter_clockwise(&mut self, n: usize) {
        for _ in 0..n {
            self.deque.rotate_right(1);
        }
    }

    fn place(&mut self, marble: i64) {
        self.deque.push_front(marble);
    }

    fn remove(&mut self) -> i64 {
        self.deque.pop_front().unwrap()
    }
}

pub fn compute_part_one(players: usize, last_marble: i64) -> i64 {
    let mut circle = Circle::default();
    circle.place(0);

    let mut scores: Vec<i64> = vec![0; players];

    for score in 1..=last_marble {
        let player = (score - 1) as usize % players;
        if score % 23 == 0 {
            scores[player] += score;
            circle.rotate_counter_clockwise(7);
            scores[player] += circle.remove();
        } else {
            circle.rotate_clockwise(2);
            circle.place(score);
        }
    }

    scores.iter().max().copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PLAYERS: usize = 493;
    const INPUT_LAST_MARBLE: i64 = 71863;

    #[test]
    fn test() {
        assert_eq!(compute_part_one(9, 32), 32);
        assert_eq!(compute_part_one(10, 1618), 8317);
        assert_eq!(compute_part_one(13, 7999), 146373);
        assert_eq!(compute_part_one(17, 1104), 2764);
        assert_eq!(compute_part_one(21, 6111), 54718);
        assert_eq!(compute_part_one(30, 5807), 37305);
        assert_eq!(compute_part_one(INPUT_PLAYERS, INPUT_LAST_MARBLE), 367802);
        assert_eq!(
            compute_part_one(INPUT_PLAYERS, INPUT_LAST_MARBLE * 100),
            2996043280
        );
    }
}
