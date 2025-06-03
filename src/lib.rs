//! A simple complex number library that I built to use for a university class.
//!
//! Author: eos Shapland
//!
//! # Examples
//!
//! ```rust
//! use complex::*;
//!
//! // Variables can be constructed using struct-builder notation,
//! let a = Complex{ re: 3.0, im: 4.0 }; // Ok
//!
//! // or using cartesian notation.
//! let b = 2.8 + 8.2*I; // also Ok
//!
//! ```

mod complex;

pub use complex::*;

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mul() {
        let a = Complex { re: 2.0, im: 2.0 };
        let b = Complex { re: 3.0, im: 3.0 };

        assert_eq!(a * b, Complex { re: 0.0, im: 12.0 });
    }

    #[test]
    fn test_I() {
        let a: f64 = 10.0;

        assert_eq!(a * I, Complex { re: 0.0, im: 10.0 });
    }

    #[test]
    fn test_mul_rhs() {
        let a = Complex { re: 4.0, im: 5.0 };

        assert_eq!(a * 5.0, Complex { re: 20.0, im: 25.0 });
    }

    fn test_add() {
        let a = 2.0 * I;
        let b = 3.0;

        assert_eq!(a + b, Complex { re: 3.0, im: 2.0 });
    }
}
