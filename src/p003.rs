// Problem 3
//
// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?
pub fn largest_prime_factor(n: u64) -> u64 {
    let mut number = n;
    let mut last = 1;
    let mut factor = 2;

    while factor <= number {
        while number % factor == 0 {
            last = factor;
            number = number / factor;
        }

        factor += 1;
    }

    if number > 1 {
        last = number;
    }

    last
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor_of_13_195() {
        assert_eq!(largest_prime_factor(13_195), 29);
    }

    #[test]
    fn test_largest_prime_factor_of_600_851_475_143() {
        assert_eq!(largest_prime_factor(600_851_475_143), 6857);
    }
}
