use crate::common_types::{InputInt, BaseInt, INPUT_BIT_WIDTH};
use crate::common_utilities;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Term {
    pub coefficient: BaseInt,
    pub exponent: u8,
}

impl Term {
    pub fn calculate_value(&self, base: BaseInt) -> InputInt {
        self.coefficient as InputInt * (base as InputInt).pow(self.exponent as u32)
           
    }

    pub fn calculate_components(&self, base: BaseInt) -> Vec<InputInt> {
        let mut components: Vec<InputInt> = vec![];
        for _ in 0..self.coefficient {
            components.push((base as InputInt).pow(self.exponent as u32));
        }
        components
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GBERepresentation {
    pub base: BaseInt,
    pub terms: [Term; INPUT_BIT_WIDTH],
    pub remainder: BaseInt,
}

impl GBERepresentation {
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

    pub fn calculate_components(&self) -> Vec<InputInt> {
        // Chain all the term-level components
        // Does not include the remainder
        self.terms.iter().flat_map(
            |term| term.calculate_components(self.base)
        ).collect()
    }

    pub fn to_decimal(&self) -> InputInt {
        let mut decimal: InputInt = 0;
        for term in self.terms.iter() {
            decimal += term.calculate_value(self.base) as InputInt;
        }
        decimal + self.remainder as InputInt
    }
}


pub fn quantized_kernel_from(number: InputInt, base: BaseInt) -> Option<(InputInt, BaseInt, Term)> {
    // The log will always be >= 1,
    // since the size is always greater than the base
    if number < base.into() {
        return None
    }
    let exponent = common_utilities::integer_log(number, base);

    // Always: number >= kernel
    let collapsed_kernel = (base as InputInt).pow(exponent as u32);
    // Always: base > coefficient
    let coefficient: BaseInt = (number / collapsed_kernel) as BaseInt;
    let full_component: InputInt = collapsed_kernel * coefficient as InputInt;
    let remainder: BaseInt = (number - full_component) as BaseInt;

    Some((collapsed_kernel, remainder, Term{coefficient, exponent}))
}
