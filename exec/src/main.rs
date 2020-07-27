use std::io;
use terms::parsing::parse_l_group_term::parse;
use cnf::normal_cnf;

fn main() -> io::Result<()> {
    println!("Please enter a term.");
    let mut buffer = String::new();    
    io::stdin().read_line(&mut buffer)?;
    
    let term = parse(&buffer).expect("Something went wrong ...");
    println!("You entered: {}", term.to_string());
    println!("The cnf is: {}", normal_cnf::CNF::from(term).to_string());
    return Ok(());
}
