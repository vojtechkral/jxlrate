use std::ops;
#[derive(Clone, Copy, Debug)]
pub struct Rational(pub u32, pub u32);

impl ops::Mul<u32> for Rational {
    type Output = u32;

    fn mul(self, rhs: u32) -> Self::Output {
        (rhs as u64 * self.0 as u64) as u32 / self.1
    }
}
