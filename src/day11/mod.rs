use std::ops::RangeInclusive;

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level -= 5;
    power_level
}

const COORDINATE_MAX: usize = 300;
type Sums = [[i32; COORDINATE_MAX + 1]; COORDINATE_MAX + 1];

// https://en.wikipedia.org/wiki/Summed-area_table
fn compute_sums(serial_number: i32) -> Sums {
    let mut sums = [[0; COORDINATE_MAX + 1]; COORDINATE_MAX + 1];
    for y in 1..=COORDINATE_MAX {
        for x in 1..=COORDINATE_MAX {
            let mut sum = power_level(x as i32, y as i32, serial_number);
            sum += sums[y - 1][x];
            sum += sums[y][x - 1];
            sum -= sums[y - 1][x - 1];
            sums[y][x] = sum;
        }
    }
    sums
}

pub fn compute(serial_number: i32, sizes: RangeInclusive<usize>) -> String {
    dbg!(serial_number);

    let sums = compute_sums(serial_number);

    let total_power = |x: usize, y: usize, size: usize| {
        sums[y][x] + sums[y + size][x + size] - sums[y][x + size] - sums[y + size][x]
    };

    let (sum, x, y, size) = sizes
        .flat_map(|size| {
            let maxdim = COORDINATE_MAX - size;
            (1..=maxdim)
                .flat_map(move |y| (1..=maxdim).map(move |x| (total_power(x, y, size), x, y, size)))
        })
        .max_by_key(|(sum, _, _, _)| *sum)
        .unwrap();
    dbg!(sum, x, y, size);
    format!("{},{},{}", x + 1, y + 1, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: i32 = 9424;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_examples() {
        assert_eq!(compute(18, 3..=3), "33,45,3");
        assert_eq!(compute(42, 3..=3), "21,61,3");
    }

    #[test]
    fn test_part_one() {
        // Paste only "243,72" into adventofcode.org
        assert_eq!(compute(INPUT, 3..=3), "243,72,3");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(compute(INPUT, 1..=COORDINATE_MAX), "229,192,11");
    }

    // For part two: https://en.wikipedia.org/wiki/Summed-area_table.
    // See also https://www.reddit.com/r/adventofcode/comments/a53r6i/comment/ebjogd7
}
