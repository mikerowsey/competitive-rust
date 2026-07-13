pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    base %= modulus;
    let mut result = 1u64;

    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }

        base = base * base % modulus;
        exp >>= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::mod_pow;

    #[test]
    fn simple_power() {
        assert_eq!(mod_pow(2, 10, 1_000_000_007), 1024);
    }

    #[test]
    fn zero_exponent() {
        assert_eq!(mod_pow(7, 0, 13), 1);
    }
}
