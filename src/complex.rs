//! The actual module

use std::f64::consts::PI;
use std::{f64, fmt};

/// Imaginary unit:
/// `I*I = -1.0`
pub const I: Complex = Complex { re: 0.0, im: 1.0 };

/// Complex Numberical type, with real and Imaginary parts existing in the Reals(`f64`)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    /// Getter function for the real part of `self`
    pub fn re(&self) -> f64 {
        self.re
    }

    /// Getter function for the Imaginary part of `self`
    pub fn im(&self) -> f64 {
        self.im
    }

    /// Returns the complex conjugate of self.
    /// i.e. `(a + b*I).star() == a - b*I`
    pub fn star(&self) -> Complex {
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

    /// Returns argument of z, `-pi<arg(z)<=pi`.
    pub fn arg(&self) -> f64 {
        let mut theta = (self.im / self.re).atan();

        if self.im > 0.0 && self.re < 0.0 {
            theta += PI;
        } else if self.im < 0.0 && self.re <= 0.0 {
            theta -= PI;
        }

        if theta == 0.0 && self.re < 0.0 {
            theta += PI;
        }

        if theta <= -PI {
            theta += PI;
        }

        theta
    }

    /// Returns `self ** exp`, where x  is an integer.
    /// Uses the polar form of `self`, so has the same time complexity as the stdlib `powi`
    /// function for floats.
    pub fn powi(&self, exp: i32) -> Complex {
        let mut a = self.polar();
        a.r = a.r.powi(exp);
        a.theta *= exp as f64;

        a.cartesian()
    }

    /// The complex logarithm function.
    /// `ln(z) = log_e(|z|) + i arg(z)`
    pub fn ln(&self) -> Complex {
        self.abs().ln() + I * self.arg()
    }

    pub fn exp(&self) -> Complex {
        self.re.exp() * (self.im.cos() + I * self.im.sin())
    }

    /// Complex power function.
    pub fn powc(&self, z: Complex) -> Complex {
        (z * self.ln()).exp()
    }

    /// Returns the polar form of `self`. The angle theta is in reference to the +real axis.
    /// `theta` is checked so that its range is [0,2pi)
    fn polar(&self) -> Polar {
        let theta = self.arg();

        let r = self.abs();

        Polar { r, theta }
    }
}

/// polar form of a complex number,
/// z = r * exp(i * theta)
#[derive(Debug, PartialEq, Copy, Clone)]
struct Polar {
    /// radius,
    /// `r = z.star()`
    r: f64,
    /// angle with respect to the +x axis,
    /// `tan(theta) = z.im / z.re`
    /// I also bounds check to make sure that theta is between 0 and 2 pi
    theta: f64,
}

impl Polar {
    /// Turns a polar complex number back into `a + b*I` form.
    fn cartesian(&self) -> Complex {
        Complex {
            re: self.r * self.theta.cos(),
            im: self.r * self.theta.sin(),
        }
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

    /// Division works by rationalizing the denominator (multiplying both operands by `rhs.star()`), and combining terms.
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

impl std::ops::Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        -self.re - self.im * I
    }
}
