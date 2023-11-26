pub fn compute_part_one(prefix: usize) -> u64 {
    let mut v: Vec<u8> = Vec::new();

    v.push(3);
    v.push(7);

    let mut a: usize = 0;
    let mut b: usize = 1;

    while v.len() < prefix + 10 {
        let sum = v[a] + v[b];
        a += v[a] as usize + 1;
        b += v[b] as usize + 1;

        let tens = sum / 10;
        let ones = sum % 10;
        if tens > 0 {
            v.push(tens);
        }
        v.push(ones);

        a %= v.len();
        b %= v.len();
    }

    let mut ret = 0;
    for digit in v.iter().skip(prefix).take(10) {
        ret *= 10;
        ret += *digit as u64;
    }
    ret
}

pub fn compute_part_two(digits_num: usize) -> u64 {
    let digits = {
        let mut digits: Vec<u8> = Vec::new();
        let mut n = digits_num;
        loop {
            digits.push((n % 10) as u8);
            n /= 10;
            if n == 0 {
                break;
            }
        }
        digits.reverse();
        digits
    };
    let mut v: Vec<u8> = Vec::new();

    v.push(3);
    v.push(7);


    let mut a: usize = 0;
    let mut b: usize = 1;
    loop {
        a %= v.len();
        b %= v.len();

        let sum = v[a] + v[b];
        a += v[a] as usize + 1;
        b += v[b] as usize + 1;

        let skip = v.len() as isize - digits.len() as isize + 1;
        if sum < 10 {
            v.push(sum);
        } else {
            v.push(1);
            v.push(sum - 10);
        }

        if skip >= 0 {
            let skip = skip as usize;
            if let Some(offset) = v[skip..]
                .windows(digits.len())
                .position(|window| window == digits)
            {
                return (skip + offset).try_into().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: usize = 509671;

    #[test]
    fn test_part_one_example() {
        assert_eq!(compute_part_one(9), 5158916779);
        assert_eq!(compute_part_one(5), 0124515891);
        assert_eq!(compute_part_one(18), 9251071085);
        assert_eq!(compute_part_one(2018), 5941429882);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(compute_part_one(INPUT), 2810862211);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(compute_part_two(INPUT), 20227889);
    }
}
