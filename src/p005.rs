// Problem 5
//
// 2520 is the smallest number that can be divided by each of the numbers from
//  1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the
// numbers from 1 to 20?
pub fn smallest_multiple(n: u32) -> u32 {
    let mut xs: Vec<u32> = vec![];
    let mut x = n;

    while x >= (n / 2) {
        let ys = power_prime_factors(prime_factors(x));
        xs = concat_vecs(&ys, &xs);

        x -= 1;
    }

    xs.iter().fold(1, |t, x| t * x)
}

fn concat_vecs<'a>(a: &'a Vec<u32>, b: &'a Vec<u32>) -> Vec<u32> {
    let mut s: Vec<u32> = vec![];

    for x in b { s.push(*x); }

    match b.iter().find(|&x| *x == a[0]) {
        Some(_) => {}
        None => {
            for x in a { s.push(*x); }
        }
    }

    s
}

fn power_prime_factors(xs: Vec<u32>) -> Vec<u32> {
    let max: &u32 = xs.iter().max().unwrap();

    xs.iter()
      .filter(|&x| x == max).map(|x| *x)
      .collect()
}

fn prime_factors(mut x: u32) -> Vec<u32> {
    let mut xs = vec![];
    let mut factor = 2;

    while factor <= (x / 2) {
        while x % factor == 0 {
            xs.push(factor);
            x = x / factor;
        }

        factor += 1;
    }

    if x > 1 {
        xs.push(x);
    }

    xs
}

// Based on https://projecteuler.net/thread=5;page=8
pub fn alt_smallest_multiple(n: u32) -> u32 {
    let mut f = 2;

    for x in 1..n {
        let p = f % x;

        if p != 0 {
            f = match x % p {
                0 => f * (x / p),
                _ => f * x
            };
        }
    }

    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_multiple_10() {
        assert_eq!(smallest_multiple(10), 2520);
    }

    #[test]
    fn test_smallest_multiple_20() {
        assert_eq!(smallest_multiple(20), 232792560);
    }

    #[test]
    fn test_alt_smallest_multiple_10() {
        assert_eq!(alt_smallest_multiple(10), 2520);
    }

    #[test]
    fn test_alt_smallest_multiple_20() {
        assert_eq!(alt_smallest_multiple(20), 232792560);
    }
}
