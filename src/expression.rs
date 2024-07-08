use std::{collections::HashMap, convert::TryFrom, fmt::Display};

use crate::term::Term;

#[derive(Clone, Debug)]
pub struct Expression {
    pub terms: HashMap<char, Term>,
}

impl Expression {
    pub fn new(terms: Vec<Term>) -> Self {
        let mut term_map = HashMap::new();
        for term in terms {
            Self::insert_to_map(&mut term_map, term);
        }
        Self { terms: term_map }
    }

    pub fn empty() -> Self {
        Self {
            terms: HashMap::new(),
        }
    }

    fn insert_to_map(map: &mut HashMap<char, Term>, term: Term) {
        if map.contains_key(&term.variable) {
            let old_term: Term = *map.get(&term.variable).unwrap();
            let new_term = Term::new(old_term.coefficient + term.coefficient, term.variable);
            map.insert(term.variable, new_term);
        } else {
            map.insert(term.variable, term);
        }
    }

    pub fn add_term(&mut self, term: Term) {
        Self::insert_to_map(&mut self.terms, term);
    }

    pub fn add_expression(&mut self, expression: Expression) {
        for term in expression.terms.values() {
            self.add_term(*term);
        }
    }

    pub fn negate(&mut self) {
        for term in self.terms.values_mut() {
            term.negate();
        }
    }

    pub fn get_negated(&self) -> Self {
        let mut terms = HashMap::new();
        for term in self.terms.values() {
            terms.insert(term.variable, term.get_negated());
        }
        Self { terms }
    }

    pub fn get_term(&self, variable: char) -> Term {
        *self.terms.get(&variable).unwrap_or(&Term {
            coefficient: 0.0,
            variable,
        })
    }

    pub fn remove_term(&mut self, variable: char) -> Option<Term> {
        self.terms.remove(&variable)
    }
}

impl TryFrom<&str> for Expression {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // let term_strs: Vec<&str> = value.split(['+', '-'].as_ref()).collect();
        // println!("{:?}", term_strs);
        // Ok(Self::empty())
        let mut expression = Expression::empty();
        let mut term_string = String::new();
        for c in value.chars() {
            if c == '+' || c == '-' {
                if !term_string.is_empty() {
                    expression.add_term(Term::try_from(term_string.as_str())?);
                    // println!("\"{}\"", term_string);
                    // println!("{}", Term::try_from(term_string.as_str())?);
                    term_string.clear();
                }
                term_string.push(c);
            } else {
                term_string.push(c);
            }
        }
        if !term_string.is_empty() {
            expression.add_term(Term::try_from(term_string.as_str())?)
            // println!("\"{}\"", term_string);
            // println!("{}", Term::try_from(term_string.as_str())?);
        }
        Ok(expression)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for term in self.terms.values() {
            if first {
                write!(f, "{}", term)?;
                first = false;
            } else if term.coefficient < 0.0 {
                write!(f, " - {}", term.get_negated())?;
            } else {
                write!(f, " + {}", term)?;
            }
        }
        Ok(())
    }
}
