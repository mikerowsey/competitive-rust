use std::ops::Mul;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MatrixMod2<const MOD: u64> {
    pub a00: u64,
    pub a01: u64,
    pub a10: u64,
    pub a11: u64,
}

impl<const MOD: u64> MatrixMod2<MOD> {
    pub const fn new(a00: u64, a01: u64, a10: u64, a11: u64) -> Self {
        Self { a00, a01, a10, a11 }
    }

    pub const fn identity() -> Self {
        Self::new(1, 0, 0, 1)
    }

    pub fn pow(mut self, mut exponent: u64) -> Self {
        let mut result = Self::identity();

        while exponent > 0 {
            if exponent & 1 == 1 {
                result = result * self;
            }

            self = self * self;
            exponent >>= 1;
        }

        result
    }
}

impl<const MOD: u64> Mul for MatrixMod2<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            (self.a00 * rhs.a00 + self.a01 * rhs.a10) % MOD,
            (self.a00 * rhs.a01 + self.a01 * rhs.a11) % MOD,
            (self.a10 * rhs.a00 + self.a11 * rhs.a10) % MOD,
            (self.a10 * rhs.a01 + self.a11 * rhs.a11) % MOD,
        )
    }
}
