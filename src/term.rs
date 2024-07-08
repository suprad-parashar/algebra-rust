use std::{convert::TryFrom, fmt::Debug, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub struct Term {
    pub coefficient: f64,
    pub variable: char,
}

impl Term {
    pub fn new(coefficient: f64, variable: char) -> Self {
        Self {
            coefficient,
            variable,
        }
    }

    pub fn constant(coefficient: f64) -> Self {
        Self {
            coefficient,
            variable: '_',
        }
    }

    pub fn negate(&mut self) {
        self.coefficient *= -1.0;
    }

    pub fn get_negated(&self) -> Self {
        Self {
            coefficient: -self.coefficient,
            variable: self.variable,
        }
    }

    pub fn is_constant(&self) -> bool {
        self.variable == '_'
    }

    pub fn is_same_variable(&self, other: &Self) -> bool {
        self.variable == other.variable
    }
}

impl TryFrom<&str> for Term {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut coef_string = String::new();
        let mut variable = '_';
        for c in value.chars() {
            if c.is_ascii_digit() {
                if variable != '_' {
                    return Err("Coefficient must come before variable");
                }
                coef_string.push(c);
            } else if c == '-' || c == '+' {
                if coef_string.is_empty() {
                    coef_string.push(c);
                } else {
                    return Err("Cannot have two operators in a term");
                }
            } else if c.is_ascii_alphabetic() {
                if variable == '_' {
                    variable = c;
                } else {
                    return Err("Cannot have two variables in a term");
                }
            } else if c == '.' {
                coef_string.push(c);
            } else if c == ' ' {
                continue;
            } else {
                return Err("Invalid character in term");
            }
        }
        let default_value = {
            if variable == '_' {
                0.0
            } else if coef_string.is_empty() {
                1.0
            } else if coef_string.trim() == "-" {
                -1.0
            } else {
                1.0
            }
        };
        Ok(Self {
            coefficient: coef_string.parse::<f64>().unwrap_or(default_value),
            variable,
        })
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variable == '_' {
            write!(f, "{}", self.coefficient)
        } else if self.coefficient == 1.0 {
            write!(f, "{}", self.variable)
        } else if self.coefficient == -1.0 {
            write!(f, "-{}", self.variable)
        } else {
            write!(f, "{}{}", self.coefficient, self.variable)
        }
    }
}
