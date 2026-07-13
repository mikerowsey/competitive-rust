#[inline]
pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn five() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn ten() {
        assert_eq!(factorial(10), 3_628_800);
    }
}
