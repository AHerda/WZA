use std::{
    fmt::{Display, Formatter},
    ops::{Add, Deref, DerefMut, Div, Mul, Rem, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GaussianNumber {
    real: i64,
    imag: i64,
}

// impl Deref for GaussianNumber {
//     type Target = Self;

//     fn deref(&self) -> &Self {
//         self
//     }
// }

// impl DerefMut for GaussianNumber {
//     fn deref_mut(&mut self) -> &mut Self {
//         self
//     }
// }

impl Add for GaussianNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for GaussianNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for GaussianNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Self::Output { real, imag }
    }
}

impl Div for GaussianNumber {
    type Output = GaussianNumber;

    fn div(self, other: Self) -> Self::Output {
        let denominator = other.real.pow(2) + other.imag.pow(2);
        let real = (self.real * other.real + self.imag * other.imag) / denominator;
        let imag = (self.imag * other.real - self.real * other.imag) / denominator;
        Self::Output { real, imag }
    }
}

impl Rem for GaussianNumber {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        let (_, remainder) = self.div_with_remainder(other);
        remainder
    }
}

impl Display for GaussianNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.imag == 0 {
            return write!(f, "{}", self.real);
        }
        if self.real == 0 {
            return write!(f, "{}i", self.imag);
        }
        if self.imag < 0 {
            write!(f, "{} - {}i", self.real, -self.imag)
        } else {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
}

impl GaussianNumber {
    pub fn new(real: i64, imag: i64) -> Self {
        Self { real, imag }
    }

    pub fn norm(&self) -> i64 {
        self.real.pow(2) + self.imag.pow(2)
    }

    pub fn div_with_remainder(self, other: Self) -> (Self, Self) {
        let denominator = other.real.pow(2) + other.imag.pow(2);
        let real = (self.real * other.real + self.imag * other.imag) / denominator;
        let imag = (self.imag * other.real - self.real * other.imag) / denominator;
        let quotient = Self { real, imag };
        let remainder = self - quotient * other;
        (quotient, remainder)
    }

    pub fn gcd(&self, other: &Self) -> Self {
        let mut a = *self;
        let mut b = *other;

        if a.norm() < b.norm() {
            std::mem::swap(&mut a, &mut b);
        }
        while b != Self::new(0, 0) {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    pub fn lcm(&self, other: &Self) -> Self {
        let gcd = self.gcd(other);
        *self * *other / gcd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = GaussianNumber::new(1, 2);
        let b = GaussianNumber::new(3, 4);
        let expected = GaussianNumber::new(4, 6);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_sub() {
        let a = GaussianNumber::new(5, 6);
        let b = GaussianNumber::new(3, 4);
        let expected = GaussianNumber::new(2, 2);
        assert_eq!(a - b, expected);
    }

    #[test]
    fn test_mul() {
        let a = GaussianNumber::new(2, 3);
        let b = GaussianNumber::new(4, 5);
        let expected = GaussianNumber::new(-7, 22);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_div() {
        let a = GaussianNumber::new(10, 5);
        let b = GaussianNumber::new(2, 1);
        let expected = GaussianNumber::new(5, 0);
        assert_eq!(a / b, expected);
        assert_eq!(a, b * expected)
    }

    #[test]
    fn test_rem() {
        let a = GaussianNumber::new(10, 5);
        let b = GaussianNumber::new(2, 1);
        let expected = GaussianNumber::new(0, 0);
        assert_eq!(a % b, expected);

        let quotient = a / b;
        let remainder = a % b;
        assert_eq!(a, b * quotient + remainder);
    }

    #[test]
    fn test_norm() {
        let a = GaussianNumber::new(3, 4);
        let expected = 25;
        assert_eq!(a.norm(), expected);
        assert_eq!(a.norm(), 3_i64.pow(2) + 4_i64.pow(2));
    }

    #[test]
    fn test_div_with_remainder() {
        let a = GaussianNumber::new(10, 5);
        let b = GaussianNumber::new(2, 1);
        let expected_quotient = GaussianNumber::new(5, 0);
        let expected_remainder = GaussianNumber::new(0, 0);
        let (quotient, remainder) = a.div_with_remainder(b);
        assert_eq!(quotient, expected_quotient);
        assert_eq!(remainder, expected_remainder);
        assert_eq!(a, b * quotient + remainder);
    }

    #[test]
    fn test_gcd() {
        let a = GaussianNumber::new(10, 5);
        let b = GaussianNumber::new(15, 10);
        let expected = GaussianNumber::new(5, 0);
        assert_eq!(a.gcd(&b), expected);
        assert_eq!(a % expected, GaussianNumber::new(0, 0));
        assert_eq!(b % expected, GaussianNumber::new(0, 0));
    }

    #[test]
    fn test_lcm() {
        let a = GaussianNumber::new(10, 5);
        let b = GaussianNumber::new(15, 10);
        let expected = GaussianNumber::new(20, 35);
        assert_eq!(a.lcm(&b), expected);
        assert_eq!(expected % a, GaussianNumber::new(0, 0));
        assert_eq!(expected % b, GaussianNumber::new(0, 0));
    }
}
