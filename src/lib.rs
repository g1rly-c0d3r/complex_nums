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
    }
}
