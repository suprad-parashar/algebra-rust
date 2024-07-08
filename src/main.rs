mod equation;
mod expression;
mod term;

use std::collections::{HashMap, HashSet};

use equation::linear_system::LinearEquations;
use equation::Equation;
use expression::Expression;
use nalgebra::DMatrix;
use term::Term;

fn main() {
    //Correct
    // let equation_system = LinearEquations::from_vec(vec![
    //     "3x - 9z = 33",
    //     "8x + 11 = x + 4y + z - 3 - 2 + 1",
    //     "4x + 6y + 6 = -5z",
    // ])
    // .unwrap();

    //ChatGPT Shit
    // let equation_system = LinearEquations::from_vec(vec![
    //     "2x + 3y - z = 1",
    //     "4x - y + 5z = 13",
    //     "-x + 2y + 3z = 7",
    // ])
    // .unwrap();

    //Correct
    // let equation_system = LinearEquations::from_vec(vec![
    //     "-6x + 6y - 6z = -30",
    //     "3x - y + 3z = -9",
    //     "-7x + 5y - 7z = -23",
    // ])
    // .unwrap();

    let equation_system = LinearEquations::from_vec(vec![
        "4x - 3y - 2z = 5",
        "-2x - y - z = -4",
        "-7x - 5y - 7z = -31",
    ])
    .unwrap();

    // println!("{:?}", Term::try_from("-a").unwrap());
    // println!("{:?}", Expression::try_from("3a - b").unwrap());

    // let equation_system = LinearEquations::from_vec(vec![
    //     "2a + 3b - c + 4d - 5e = 7",
    //     "-a + 4b + 2c - 3d + 2e = -8",
    //     "3a - b + 5c + d - 2e = 10",
    //     "4a + 2b - 3c + d + 3e = 6",
    //     "5a - 3b + 2c + 3d - e = -4",
    // ])
    // .unwrap();

    let solution = match equation_system.solve() {
        Ok(sol) => sol,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    for variable in equation_system.get_sorted_variables_list() {
        println!("{} = {:.4}", variable, solution.get(&variable).unwrap());
    }
}

fn solve(equations: Vec<&str>) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut maps: Vec<HashMap<char, i32>> = Vec::new();
    let mut variables: HashSet<char> = HashSet::new();
    for equation in equations.iter() {
        let map = parse_equation(equation).unwrap();
        println!("{:?}", map);
        for (key, _value) in &map {
            if *key != '_' {
                variables.insert(*key);
            }
        }
    }
    for equation in equations.iter() {
        let mut map = parse_equation(equation).unwrap();
        for variable in &variables {
            if !map.contains_key(variable) {
                map.insert(*variable, 0);
            }
        }
        maps.push(map);
    }
    let mut coefficients: Vec<Vec<i32>> = Vec::new();
    for map in maps.iter() {
        let mut row: Vec<i32> = Vec::new();
        for variable in &variables {
            row.push(*map.get(variable).unwrap());
        }
        coefficients.push(row);
    }
    let mut values: Vec<i32> = Vec::new();
    for map in maps.iter() {
        values.push(-*map.get(&'_').unwrap());
    }
    (coefficients, values)
}

fn parse_equation(equation: &str) -> Result<HashMap<char, i32>, &'static str> {
    let expressions = equation.split('=').collect::<Vec<&str>>();
    if expressions.len() != 2 {
        return Err("Invalid equation");
    }
    let lhs = expressions[0];
    let rhs = expressions[1];
    let mut map = parse(lhs);
    for (key, value) in parse(rhs) {
        if map.contains_key(&key) {
            map.insert(key, map.get(&key).unwrap() - value);
        } else {
            map.insert(key, -value);
        }
    }
    Ok(map)
}

fn parse(equation: &str) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut coef = String::new();
    for c in equation.chars() {
        // println!("{:?}, {:?}, {:?}, {:?}", c, map, coef, before_eq);
        if c.is_ascii_digit() {
            coef.push(c);
        }
        if c.is_ascii_alphabetic() {
            let new_value = match coef.parse::<i32>() {
                Ok(val) => val,
                Err(_e) => 1,
            };
            if map.contains_key(&c) {
                map.insert(c, map.get(&c).unwrap() + new_value);
            } else {
                map.insert(c, new_value);
            }
            coef.clear();
        }
        if c == '-' {
            if coef.is_empty() {
                coef.push(c);
            } else {
                if map.contains_key(&'_') {
                    map.insert('_', map.get(&'_').unwrap() + coef.parse::<i32>().unwrap());
                } else {
                    map.insert('_', coef.parse::<i32>().unwrap());
                }
                coef.clear();
                coef.push(c);
            }
        }
        if c == '+' {
            if coef.is_empty() {
                continue;
            }
            if map.contains_key(&'_') {
                map.insert('_', map.get(&'_').unwrap() + coef.parse::<i32>().unwrap());
            } else {
                map.insert('_', coef.parse::<i32>().unwrap());
            }
            coef.clear();
        }
    }
    if !coef.is_empty() {
        if map.contains_key(&'_') {
            map.insert('_', map.get(&'_').unwrap() + coef.parse::<i32>().unwrap());
        } else {
            map.insert('_', coef.parse::<i32>().unwrap());
        }
    }
    map
}
