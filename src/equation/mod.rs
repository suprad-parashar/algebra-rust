use crate::expression::Expression;
use std::fmt::Display;
pub mod linear_system;

pub struct Equation {
    pub lhs: Expression,
    pub rhs: Expression,
}

impl Equation {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        Self { lhs, rhs }
    }

    pub fn equate_to_zero(&self) -> Expression {
        let mut lhs = self.lhs.clone();
        lhs.add_expression(self.rhs.get_negated());
        lhs
    }

    pub fn move_variables_to_lhs(&self) -> Equation {
        let mut lhs = self.equate_to_zero();
        let mut rhs = Expression::empty();
        let constant_term = lhs.remove_term('_').unwrap();
        rhs.add_term(constant_term.get_negated());
        Equation::new(lhs, rhs)
    }
}

impl TryFrom<&str> for Equation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sides = value.split('=').collect::<Vec<&str>>();
        if sides.len() != 2 {
            return Err("Invalid equation. Can only have one equals sign.");
        }
        let lhs = Expression::try_from(sides[0])?;
        let rhs = Expression::try_from(sides[1])?;
        Ok(Self::new(lhs, rhs))
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
    }
}
