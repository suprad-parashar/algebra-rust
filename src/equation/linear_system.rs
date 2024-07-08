use nalgebra::DMatrix;

use crate::equation::Equation;
use std::collections::{HashMap, HashSet};

pub struct LinearEquations {
    pub equations: Vec<Equation>,
}

impl LinearEquations {
    pub fn new(equations: Vec<Equation>) -> Self {
        Self { equations }
    }

    pub fn from_vec(equations: Vec<&str>) -> Result<Self, &'static str> {
        let mut eqs = Vec::new();
        for equation in equations {
            let eq = Equation::try_from(equation)?;
            eqs.push(eq);
        }
        Ok(Self::new(eqs))
    }

    pub fn get_variables_list(&self) -> HashSet<char> {
        let mut variables: HashSet<char> = HashSet::new();
        for equation in self.equations.iter() {
            let equation = equation.move_variables_to_lhs();
            let keys: HashSet<char> = equation.lhs.terms.keys().cloned().collect();
            variables = variables.union(&keys).cloned().collect();
        }
        variables
    }

    pub fn get_sorted_variables_list(&self) -> Vec<char> {
        let mut variables = self.get_variables_list().into_iter().collect::<Vec<char>>();
        variables.sort();
        variables
    }

    pub fn solve(&self) -> Result<HashMap<char, f64>, &'static str> {
        let variables = self.get_sorted_variables_list();
        if variables.len() != self.equations.len() {
            return Err("Number of variables and equations do not match");
        }

        let mut coefficients: Vec<Vec<f64>> = Vec::new();
        let mut values: Vec<f64> = Vec::new();
        for equation in self.equations.iter() {
            let equation = equation.move_variables_to_lhs();
            let mut row: Vec<f64> = Vec::new();
            for variable in &variables {
                let term = equation.lhs.get_term(*variable);
                row.push(term.coefficient);
            }
            coefficients.push(row);
            values.push(equation.rhs.get_term('_').coefficient);
        }

        let coefficients: DMatrix<f64> =
            DMatrix::from_fn(self.equations.len(), variables.len(), |i, j| {
                coefficients[i][j] as f64
            });
        println!("{:?}", coefficients);
        let values: DMatrix<f64> = DMatrix::from_fn(variables.len(), 1, |i, _j| values[i] as f64);
        println!("{:?}", values);

        let inverse_matrix = coefficients
            .try_inverse()
            .ok_or("Matrix is not invertible")?;

        let result = inverse_matrix * values;
        let mut map = HashMap::new();
        for (i, variable) in variables.iter().enumerate() {
            map.insert(*variable, result[(i, 0)]);
        }
        Ok(map)
    }
}
