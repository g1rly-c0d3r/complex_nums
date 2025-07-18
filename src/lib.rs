//! A simple complex number library that I built to use for a university class.
//!
//! Author: g1rly-c0d3r
//!
//! # Examples
//!
//! ```rust
//! use complex_nums::*;
//!
//! // Complex numbers are constructed using cartesian notation.
//! let b = 2.8 + 8.2*I;
//!
//! ```
//! Cartesian notation is the prefered way of instantiating and using this library.
//!
//!
//! Addition, Subtraction, Multiplication, and Division are implemented for both real-complex and complex-complex operations.
//! Comutative operations are comutative, non-commutative operations are implemented for both
//! directions.
//!
//! ```rust
//! use complex_nums::*;
//!
//! let a = 2.0 + 3.0*I;
//! let b = 1.5 + 7.4*I;
//! let c = 4.6;
//!
//! // Complex-Complex Multiplication takes the form (a+b*I)*(c+d*I),
//! // and follows standard 'foiling' or distributing rules.
//! assert_eq!(a * b,  -19.200000000000003 + 19.3*I );
//! // real-complex Multiplication also follows this rule,
//! // and effectively scales the complex number by the real number.
//! assert_eq!(a * c,  9.2 + 13.799999999999999*I );
//! assert_eq!(c * b, 6.8999999999999995 + 34.04 *I);
//!
//! // addition is handled element-wise, so real parts add, and imaginary parts add.
//! assert_eq!(a + c, 6.6 + 3.0 *I);
//! assert_eq!(c + a, 6.6 + 3.0 *I);
//! assert_eq!(a + b, 3.5 + 10.4*I);
//!
//! // Subtraction is implemented the same way
//! // (if the complex number is the rhs, then the negative is distributed)
//! assert_eq!(a - b, 0.5 -4.4*I);
//! assert_eq!(a - c, -2.5999999999999996 + 3.0*I );
//! assert_eq!(c - b, 3.0999999999999996 - 7.4*I);
//!
//! // Division is a little more complex,
//! // please see the documentation for the div operator.
//! assert_eq!(a / b, 0.44202771443606387 - 0.18067005788458165*I);
//! assert_eq!(a / c, 0.4347826086956522 + 0.6521739130434783*I);
//!
//! ```
//!
//! The standard complex operators are implemented as well:
//! ```rust
//! use complex_nums::*;
//!
//! let a = 3.0 + 4.0*I;
//!
//! // complex conjugate
//! assert_eq!(a.star(), 3.0 - 4.0*I);
//! // Different implementation of exponentiation, converting to polar,
//! // exponentiating, and then converting back.
//! // Slightly less accurate than the other one,
//! // but much, much faster. (1 us vs ~80 ns to compute z^(2^10))
//! assert!( (a.powi(2) - (-7. + 24.*I)).re() < 1e-10 &&
//!             (a.powi(2) - (-7. + 24.*I)).im() < 1e-10 );
//!
//! // Magnitude modulus of a complex number
//! assert_eq!(a.abs(), 5.0)
//! ```

mod complex;

pub use crate::complex::*;

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_mul() {
        let a = 2. + 2. * I;
        let b = 3.0 + 3.0 * I;
        let c = 2.5;

        assert_eq!(a * b, 12.0 * I);
        assert_eq!(b * a, 12.0 * I);
        assert_eq!(a * c, 5.0 + 5.0 * I);
        assert_eq!(c * b, 7.5 + 7.5 * I);
    }

    #[test]
    fn test_I() {
        let a: f64 = 10.0;

        assert_eq!(a * I, 10.0 * I);
    }

    #[test]
    fn test_add() {
        let a = 2.0 * I;
        let b = 1.0 + 4.0 * I;
        let c = 3.0;

        assert_eq!(a + c, 3.0 + 2.0 * I);
        assert_eq!(c + a, 3.0 + 2.0 * I);
        assert_eq!(a + b, 1.0 + 6.0 * I);
    }

    #[test]
    fn test_sub() {
        let a = 3.6 + 12.0 * I;
        let b = 5.4 + 2.0 * I;
        let c = 6.7;

        assert_eq!(a - b, -1.8000000000000003 + 10.0 * I);

        assert_eq!(a - c, -3.1 + 12.0 * I);
        assert_eq!(c - b, 1.2999999999999998 + -2.0 * I);
    }

    #[test]
    fn test_star() {
        let a = 3.0 + 4.7 * I;

        assert_eq!(a.star(), 3.0 - 4.7 * I);
    }

    #[test]
    fn test_arg() {
        let a = 1. + 0. * I;
        let b = 1. + 2. * I;
        let c = 1. + I;
        let d = 2. + I;
        let e = I;

        // 1st quadrant
        assert_eq!(a.arg(), 0.);
        assert!((b.arg() - 2.0_f64.atan()).abs() < 1e-10);
        assert!((c.arg() - PI / 4.).abs() < 1e-10);
        assert!((d.arg() - 0.5_f64.atan()).abs() < 1e-10);
        assert!((e.arg() - PI / 2.).abs() < 1e-10);

        // 2nd quadrant
        assert!(((I * a).arg() - PI / 2.).abs() < 1e-10);
        assert!(((I * b).arg() - (PI - 0.5_f64.atan())).abs() < 1e-10);
        assert!(((I * c).arg() - 3. * PI / 4.).abs() < 1e-10);
        assert!(((I * d).arg() - (PI - 2_f64.atan())).abs() < 1e-10);
        assert!(((I * e).arg() - PI).abs() < 1e-10);

        // 3rd quadrant
        assert!(((I * I * a).arg() - PI).abs() < 1e-10);
        assert!(((I * I * b).arg() - (2_f64.atan() - PI)).abs() < 1e-10);
        assert!(((I * I * c).arg() - (1_f64.atan() - PI)).abs() < 1e-10);
        assert!(((I * I * d).arg() - (0.5_f64.atan() - PI)).abs() < 1e-10);
        assert!(((I * I * e).arg() - -0.5 * PI).abs() < 1e-10);

        // 4th quadrant
        assert!(((I * I * I * a).arg() - -0.5 * PI).abs() < 1e-10);
        assert!(((I * I * I * b).arg() - -0.5_f64.atan()).abs() < 1e-10);
        assert!(((I * I * I * c).arg() - -1_f64.atan()).abs() < 1e-10);
        assert!(((I * I * I * d).arg() - -2_f64.atan()).abs() < 1e-10);
        assert!(((I * I * I * e).arg() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_abs() {
        let a = 3.0 + 4.0 * I;

        assert_eq!(a.abs(), 5.0);
    }

    #[test]
    fn test_powi() {
        let a = 3.0 + 4.0 * I;
        let b = -3. + 4. * I;
        let c = 3. - 4. * I;
        let d = -3. - 4. * I;

        assert!(
            (a.powi(2) - (-7.0 + 24.0 * I)).re() < 1e-10
                && (a.powi(2) - (-7.0 + 24.0 * I)).im() < 1e-10
        );
        assert!(
            (b.powi(2) - (7.0 + 24.0 * I)).re() < 1e-10
                && (b.powi(2) - (7.0 + 24.0 * I)).im() < 1e-10
        );
        assert!(
            (c.powi(2) - (7.0 + 24.0 * I)).re() < 1e-10
                && (c.powi(2) - (7.0 + 24.0 * I)).im() < 1e-10
        );
        assert!(
            (d.powi(2) - (-7.0 + 24.0 * I)).re() < 1e-10
                && (d.powi(2) - (-7.0 + 24.0 * I)).im() < 1e-10
        );
    }

    #[test]
    fn test_div() {
        let a = 3. + 4. * I;
        let b = 6. + 7. * I;
        let c = 2.;

        assert_eq!(a / b, 0.5411764705882353 + I * 0.03529411764705882);
        assert_eq!(a / c, 1.5 + I * 2.);
        assert_eq!(c / b, 0.1411764705882353 - I * 0.16470588235294117);
    }

    #[test]
    fn test_neg() {
        let a = 3. + 4. * I;
        let b = -2. + 3. * I;
        let c = 4. - 6. * I;
        let d = -1. - 6. * I;

        assert_eq!(-a, -3. - 4. * I);
        assert_eq!(-b, 2. - 3. * I);
        assert_eq!(-c, -4. + 6. * I);
        assert_eq!(-d, 1. + 6. * I);
    }

    #[test]
    fn test_ln() {
        let z = 3. + 4. * I;

        assert!(
            (z.ln().re() - 1.6094379124341003).abs() < 1e-10
                && (z.ln().im() - 0.927295218002).abs() < 1e-10
        );
    }

    #[test]
    fn test_powc() {
        let z = 3. - 4.0 * I;

        assert_eq!(z.ln(), 5_f64.ln() + I * (-4_f64 / 3.0).atan());
    }
}
