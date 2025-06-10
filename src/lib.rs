//! A simple complex number library that I built to use for a university class.
//!
//! Author: eos Shapland
//!
//! # Examples
//!
//! ```rust
//! use complex_nums::*;
//!
//! // Variables can be constructed using struct-builder notation,
//! let a = Complex{ re: 3.0, im: 4.0 }; // Ok
//!
//! // or using cartesian notation.
//! let b = 2.8 + 8.2*I; // also Ok
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
//! assert_eq!(a * b, Complex { re: -19.200000000000003, im: 19.3 });
//! // real-complex Multiplication also follows this rule,
//! // and effectively scales the complex number by the real number.
//! assert_eq!(a * c, Complex { re: 9.2, im: 13.799999999999999 });
//! assert_eq!(c * b, Complex { re: 6.8999999999999995, im: 34.04 });
//!
//! // addition is handled element-wise, so real parts add, and imaginary parts add.
//! assert_eq!(a + c, Complex { re: 6.6, im: 3.0 });
//! assert_eq!(c + a, Complex { re: 6.6, im: 3.0 });
//! assert_eq!(a + b, Complex { re: 3.5, im: 10.4});
//!
//! // Subtraction is implemented the same way
//! // (if the complex number is the rhs, then the negative is distributed)
//! assert_eq!(a - b, Complex { re: 0.5, im: -4.4});
//! assert_eq!(a - c, Complex { re: -2.5999999999999996, im: 3.0 });
//! assert_eq!(c - b, Complex { re: 3.0999999999999996, im: -7.4});
//!
//! // Division is a little more complex,
//! // please see the documentation for the div operator.
//! assert_eq!(a / b, Complex { re: 0.44202771443606387, im: -0.18067005788458165});
//! assert_eq!(a / c, Complex { re: 0.4347826086956522, im: 0.6521739130434783});
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
//! assert_eq!(a.bar(), 3.0 - 4.0*I);
//! // z^n where n an integer.
//! // VERY SLOW for large n, this will be fixed once polar form is implemented
//! assert_eq!(a.pow(2), -7. + 24.*I);
//! // Magnitude modulus of a complex number
//! assert_eq!(a.abs(), 5.0)
//! ```

mod complex;

pub use crate::complex::*;

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Complex { re: 2.0, im: 2.0 };
        let b = Complex { re: 3.0, im: 3.0 };
        let c = 2.5;

        assert_eq!(a * b, Complex { re: 0.0, im: 12.0 });
        assert_eq!(b * a, Complex { re: 0.0, im: 12.0 });
        assert_eq!(a * c, Complex { re: 5.0, im: 5.0 });
        assert_eq!(c * b, Complex { re: 7.5, im: 7.5 });
    }

    #[test]
    fn test_I() {
        let a: f64 = 10.0;

        assert_eq!(a * I, Complex { re: 0.0, im: 10.0 });
    }

    #[test]
    fn test_add() {
        let a = 2.0 * I;
        let b = 1.0 + 4.0 * I;
        let c = 3.0;

        assert_eq!(a + c, Complex { re: 3.0, im: 2.0 });
        assert_eq!(c + a, Complex { re: 3.0, im: 2.0 });
        assert_eq!(a + b, Complex { re: 1.0, im: 6.0 });
    }

    #[test]
    fn test_sub() {
        let a = 3.6 + 12.0 * I;
        let b = 5.4 + 2.0 * I;
        let c = 6.7;

        assert_eq!(
            a - b,
            Complex {
                re: -1.8000000000000003,
                im: 10.0
            }
        );

        assert_eq!(a - c, Complex { re: -3.1, im: 12.0 });
        assert_eq!(
            c - b,
            Complex {
                re: 1.2999999999999998,
                im: -2.0
            }
        );
    }

    #[test]
    fn test_bar() {
        let a = 3.0 + 4.7 * I;

        assert_eq!(a.bar(), Complex { re: 3.0, im: -4.7 });
    }

    #[test]
    fn test_abs() {
        let a = 3.0 + 4.0 * I;

        assert_eq!(a.abs(), 5.0);
    }

    #[test]
    fn test_pow() {
        let a = 3.0 + 4.0 * I;

        assert_eq!(a.pow(2), Complex { re: -7.0, im: 24.0 });
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
}
