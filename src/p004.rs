// Problem 4
//
// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.
pub fn largest_palindrome_product() -> usize {
    let mut max: usize = 0;
    let mut x = 999;

    while x >= 100 {
        let mut y = x;

        while y >= 100 {
            let product = x * y;

            if product < max { break; }

            if is_palindrome(product) {
                max = product;
            }
            y -= 1;
        }
        x -= 1;
    }

    max
}

pub fn is_palindrome(n: usize) -> bool {
    let s1: String = n.to_string();
    let s2: String = s1.chars().rev().collect();

    s1 == s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_palindrome_product() {
        assert_eq!(largest_palindrome_product(), 906609);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(99));
        assert!(!is_palindrome(988));
        assert!(is_palindrome(9009));
    }
}
