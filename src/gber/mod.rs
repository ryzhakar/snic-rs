#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Term {
    pub coefficient: u64,
    pub exponent: u8,
}

impl Term {
    pub fn calculate_value(&self, base: u64) -> u64 {
        self.coefficient * base.pow(self.exponent as u32)
    }

    pub fn calculate_components(&self, base: u64) -> Vec<u64> {
        let mut components = vec![];
        for _ in 0..self.coefficient {
            components.push(base.pow(self.exponent as u32));
        }
        components
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GBERepresentation {
    pub base: u64,
    pub terms: [Term; 64],
    pub remainder: u64,
}

impl GBERepresentation {
    pub fn new(decimal_number: u64, base: u64) -> Self {
        let mut remainder = decimal_number;
        let mut terms = [Term{coefficient: 0, exponent: 0}; 64];
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

    pub fn calculate_components(&self) -> Vec<u64> {
        let mut components: Vec<u64> = self.terms.iter().flat_map(
            |term| term.calculate_components(self.base)
        ).collect();
        components.push(self.remainder);
        components
    }

    pub fn to_decimal(&self) -> u64 {
        let mut decimal = 0;
        for term in self.terms.iter() {
            decimal += term.calculate_value(self.base);
        }
        decimal + self.remainder
    }
}


pub fn quantized_kernel_from(number: u64, base: u64) -> Option<(u64, u64, Term)> {
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

pub fn integer_log(number: u64, base: u64) -> u8 {
    // Brute-force integer logarithm.
    // Floating point logarithms
    // lose precision dramatically for large numbers
    let mut n = number;
    let mut log = 0;
    while n >= base {
        n /= base;
        log += 1;
    }
    log
}
