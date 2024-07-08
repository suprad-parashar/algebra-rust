mod equation;
mod expression;
mod term;

use equation::linear_system::LinearEquations;

fn main() {
    //Correct
    // let equation_system = LinearEquations::from_vec(vec![
    //     "3x - 9z = 33",
    //     "8x + 11 = x + 4y + z - 3 - 2 + 1",
    //     "4x + 6y + 6 = -5z",
    // ])
    // .unwrap();

    // Correct
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
