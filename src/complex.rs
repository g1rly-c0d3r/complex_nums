//! The actuall module

use std::{f64, fmt};

/// Imaginary unit (`0 + 1*i`)
pub const I: Complex = Complex { re: 0.0, im: 1.0 };

/// Complex Numberical type, with real and Imaginary parts existing in the Reals(`f64`)
///
///
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl fmt::Display for Complex {
    /// Prints complex numbers in Cartesian form: `a + b*I`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.im.signum() {
            1.0 => write!(f, "{0} + {1}*I", self.re, self.im),
            -1.0 => write!(f, "{0} - {1}*I", self.re, self.im.abs()),
            _ => write!(f, "Complex part is NaN!"),
        }
    }
}

impl std::ops::Mul<Complex> for Complex {
    type Output = Complex;
    /// Cartesian multiplication of two complex numbers.
    /// standard expansion of `(a + b*I)*(c + d*I)` (foilling),
    /// where `a,b,c,d` exist in the Reals.
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            re: (self.re * rhs.re) - (self.im * rhs.im),
            im: (self.re * rhs.im) + (self.im * rhs.re),
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Mul<Complex> for f64 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self * rhs.re,
            im: self * rhs.im,
        }
    }
}

impl std::ops::Add<f64> for Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        Complex {
            re: self.re + rhs,
            im: self.im,
        }
    }
}

impl std::ops::Add<Complex> for f64 {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self + rhs.re,
            im: rhs.im,
        }
    }
}
