//! Generalized Base Exponential Representation (GBER)
//! Key aspects of GBER in SNIC:
//! 1. **Number Representation**: GBER represents any non-negative integer N as a sum of powers of a chosen base b:
//!    N = c₁b^p₁ + c₂b^p₂ + ... + cₖb^pₖ + r
//!    Where 0 < cᵢ < b, p₁ > p₂ > ... > pₖ ≥ 0, and 0 ≤ r < b
//! 
//! 2. **Uniqueness**: For any given base b and number N, there is only one correct GBER representation.
//! 
//! 3. **Completeness**: GBER can represent any non-negative integer for any base b > 1.

use crate::common_types::{InputInt, BaseInt, INPUT_BIT_WIDTH};
use crate::common_utilities;

/// Building block of GBER: `cb^p`
/// A positive integer representation
/// as multiplication of base exponents.
///
/// Example in base-3: `18 = 2 * 3 ^ 2`
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Term {
    /// Number of components.
    /// Always smaller than base - being equal to the base would increment the exponent.
    pub coefficient: BaseInt,
    /// The power of the base.
    pub exponent: u8,
}

impl Term {
    /// Transform the term back to its original integer value.
    pub fn calculate_value(&self, base: BaseInt) -> InputInt {
        self.coefficient as InputInt * (base as InputInt).pow(self.exponent as u32)
           
    }

    /// Calculate integer values of the terms components.
    pub fn calculate_components(&self, base: BaseInt) -> Vec<InputInt> {
        let component_value = (base as InputInt).pow(self.exponent as u32);
        return (0..self.coefficient).map(|_| component_value).collect()
    }
}

/// GBER of a number.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Decomposition {
    /// The base against which the number is decomposed.
    pub base: BaseInt,
    /// The resulting terms of the decomposition
    pub terms: [Term; INPUT_BIT_WIDTH],
    /// The last 0-power term as a remainder.
    pub remainder: BaseInt,
}

impl Decomposition {
    pub fn new(decimal_number: InputInt, base: BaseInt) -> Self {
        let mut remainder = decimal_number;
        let mut terms = [Term{coefficient: 0, exponent: 0}; INPUT_BIT_WIDTH];
        let mut index = 0;
        while index <= INPUT_BIT_WIDTH {
            match quantized_kernel_from(remainder, base) {
                Some((_kernel, new_remainder, term)) => {
                    terms[index] = term;
                    remainder = new_remainder as InputInt;
                    index += 1;
                },
                _ => break,
            };
        };
        Self{base, terms, remainder: remainder as BaseInt}
    }

    /// Chain all the term-level components.
    /// Does not include the remainder.
    pub fn calculate_components(&self) -> Vec<InputInt> {
        self.terms.iter().flat_map(
            |term| term.calculate_components(self.base)
        ).collect()
    }

    /// Return the original integer value of the GBER.
    pub fn to_decimal(&self) -> InputInt {
        let mut decimal: InputInt = 0;
        for term in self.terms.iter() {
            decimal += term.calculate_value(self.base) as InputInt;
        }
        decimal + self.remainder as InputInt
    }
}


fn quantized_kernel_from(number: InputInt, base: BaseInt) -> Option<(InputInt, InputInt, Term)> {
    // The log will always be >= 1,
    // since the size is always greater than the base
    if number < base.into() {
        return None
    }
    let exponent = common_utilities::integer_log(number, base);

    // Always: number >= kernel >= base > coefficient
    let collapsed_kernel = (base as InputInt).pow(exponent as u32);
    let coefficient: BaseInt = (number / collapsed_kernel) as BaseInt;
    let full_component: InputInt = collapsed_kernel * coefficient as InputInt;
    let temporary_remainder: InputInt = number - full_component;

    Some((collapsed_kernel, temporary_remainder, Term{coefficient, exponent}))
}
