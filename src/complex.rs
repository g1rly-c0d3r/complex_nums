//! The actual module

use std::{f64, fmt};

/// Imaginary unit:
/// `I*I = -1.0`
pub const I: Complex = Complex { re: 0.0, im: 1.0 };

/// Complex Numberical type, with real and Imaginary parts existing in the Reals(`f64`)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    /// Returns the complex conjugate of self.
    /// i.e. `(a + b*I).bar() == a - b*I`
    pub fn bar(&self) -> Complex {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }

    /// Returns magnitude (a.k.a modulus) of `self`.
    /// i.e. `(a + b*I).abs() == (a*a + b*b).sqrt()`
    pub fn abs(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    /// Returns `self` raised to (integer) power `exp`
    /// i.e. `(a + b*I).pow(n) == (a + b*I)*(a + b*I)*...(n times)...*(a + b*I)`
    /// `(a + b*I).pow(2) => (a + b*I)*(a + b*I) => (a*a - b*b) + I*(a*b + b*a)`.
    ///
    /// Possibly very slow for large values of `exp`, as the logic is niave repeated
    /// multiplication. TODO: use a better algoritm for exponentiation.
    pub fn pow(&self, exp: isize) -> Complex {
        let mut a = *self;
        for _ in 0..exp - 1 {
            a = a * *self;
        }
        a
    }
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

impl std::ops::Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, rhs: f64) -> Self::Output {
        Complex {
            re: self.re - rhs,
            im: self.im,
        }
    }
}

impl std::ops::Sub<Complex> for f64 {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self - rhs.re,
            // gotta distribute the negative
            // (this is why we write functions for this,
            // I would not do this accidentally and be scratching my head for weeks)
            im: -rhs.im,
        }
    }
}

impl std::ops::Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Complex;

    /// Division by reals is simple, and is the same as multiplying by a real.
    /// `(a + b*I) / c == (a/c) + (b/c)*I`
    fn div(self, rhs: f64) -> Self::Output {
        Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl std::ops::Div<Complex> for Complex {
    type Output = Complex;

    /// Division works by rationalizing the denominator (multiplying both operands by `rhs.bar()`), and combining terms.
    fn div(self, rhs: Complex) -> Self::Output {
        Complex {
            re: (self.re * rhs.re + self.im * rhs.im) / (rhs.re.powi(2) + rhs.im.powi(2)),
            im: (self.im * rhs.re - self.re * rhs.im) / (rhs.re.powi(2) + rhs.im.powi(2)),
        }
    }
}

impl std::ops::Div<Complex> for f64 {
    type Output = Complex;

    /// Division of a real number by a complex number is similar to complex-complex division,
    /// where we must rationalize the denominator.
    fn div(self, rhs: Complex) -> Self::Output {
        Complex {
            re: (self * rhs.re) / (rhs.re.powi(2) + rhs.im.powi(2)),
            im: -(self * rhs.im) / (rhs.re.powi(2) + rhs.im.powi(2)),
        }
    }
}
