pub fn compute_part_one(prefix: u32) -> u64 {
    let prefix: usize = prefix.try_into().unwrap();
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

pub fn compute_part_two(needle: u32) -> usize {
    dbg!(needle);
    let needle_len = {
        let mut c: u32 = 0;
        let mut n = needle;
        loop {
            c += 1;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        c
    };
    let trailing_modulus = 10_u32.checked_pow(needle_len).unwrap();

    let mut v: Vec<u8> = Vec::new();

    v.push(3);
    v.push(7);
    let mut trailing = 37_u32;
    trailing %= trailing_modulus;

    let mut a: usize = 0;
    let mut b: usize = 1;
    loop {
        a %= v.len();
        b %= v.len();

        let mut sum = v[a] + v[b];
        a += v[a] as usize + 1;
        b += v[b] as usize + 1;

        if sum >= 10 {
            v.push(1);
            trailing = (trailing * 10) % trailing_modulus + 1;
            if trailing == needle {
                return v.len() - needle_len as usize;
            }
            sum -= 10;
        }
        v.push(sum);
        trailing = (trailing * 10) % trailing_modulus + sum as u32;
        if trailing == needle {
            return v.len() - needle_len as usize;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: u32 = 509671;

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
