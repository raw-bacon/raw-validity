use std::io;
use l_group_formulas::formula::LGroupFormula;
use l_group_validity::is_valid;

fn main() -> io::Result<()> {
    println!("Please enter an l-group equation or inequation.");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let formula = LGroupFormula::from(buffer.as_str());
    println!("You entered: {}", formula.to_string());
    let validity_string = match is_valid(formula.clone()) {
        true => "valid",
        false => "invalid"
    };

    println!("\n{} is {}", formula.to_string(), validity_string);

    return Ok(());
}
