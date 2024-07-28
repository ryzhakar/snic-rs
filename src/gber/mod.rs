type InputInt = u32;
const INPUT_BIT_WIDTH: usize = 32;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Term {
    pub coefficient: InputInt,
    pub exponent: u8,
}

impl Term {
    pub fn calculate_value(&self, base: InputInt) -> InputInt {
        self.coefficient * base.pow(self.exponent as u32)
    }

    pub fn calculate_components(&self, base: InputInt) -> Vec<InputInt> {
        let mut components = vec![];
        for _ in 0..self.coefficient {
            components.push(base.pow(self.exponent as u32));
        }
        components
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GBERepresentation {
    pub base: InputInt,
    pub terms: [Term; INPUT_BIT_WIDTH],
    pub remainder: InputInt,
}

impl GBERepresentation {
    pub fn new(decimal_number: InputInt, base: InputInt) -> Self {
        let mut remainder = decimal_number;
        let mut terms = [Term{coefficient: 0, exponent: 0}; INPUT_BIT_WIDTH];
        let mut index = 0;
        while index <= 64 {
            match quantized_kernel_from(remainder, base) {
                Some((_kernel, new_remainder, term)) => {
                    terms[index] = term;
                    remainder = new_remainder;
                    index += 1;
                },
                _ => break,
            };
        };
        Self{base, terms, remainder}
    }

    pub fn calculate_components(&self) -> Vec<InputInt> {
        let mut components: Vec<InputInt> = self.terms.iter().flat_map(
            |term| term.calculate_components(self.base)
        ).collect();
        components.push(self.remainder);
        components
    }

    pub fn to_decimal(&self) -> InputInt {
        let mut decimal = 0;
        for term in self.terms.iter() {
            decimal += term.calculate_value(self.base);
        }
        decimal + self.remainder
    }
}


pub fn quantized_kernel_from(number: InputInt, base: InputInt) -> Option<(InputInt, InputInt, Term)> {
    // The log will always be >= 1,
    // since the size is always greater than the base
    if number < base {
        return None
    }
    let exponent = integer_log(number, base);
    let collapsed_kernel = base.pow(exponent as u32);
    let coefficient = number / collapsed_kernel;
    let remainder = number % (collapsed_kernel * coefficient);
    Some((collapsed_kernel, remainder, Term{coefficient, exponent}))

}

pub fn integer_log(number: InputInt, base: InputInt) -> u8 {
    // The floor of the log base of a number
    (number as f64).log(base as f64).floor() as u8
}
