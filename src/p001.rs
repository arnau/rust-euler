// Problem 1
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we
// get 3, 5, 6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.
pub fn sum_of_multiples(m: u32) -> u32 {
    (1..m)
        .filter(|x| (x % 3 == 0) || (x % 5 == 0))
        .fold(0, |sum, x| sum + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_multiples_10() {
        assert_eq!(sum_of_multiples(10), 23);
    }

    #[test]
    fn test_sum_of_multiples_1000() {
        assert_eq!(sum_of_multiples(1000), 233168);
    }
}
