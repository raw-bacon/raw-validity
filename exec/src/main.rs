use std::io;
use terms::l_group_term::LGroupTerm;
use cnf::normal_cnf;
use cnf::three_cnf;

fn main() -> io::Result<()> {
    println!("Please enter a term.");
    let mut buffer = String::new();    
    io::stdin().read_line(&mut buffer)?;
    
    let term = LGroupTerm::from(buffer.as_str());
    println!("You entered: {}", term.to_string());
    println!("The cnf is: {}", normal_cnf::CNF::from(term.clone()).to_string());
    println!("The 3-cnf is: {}", three_cnf::ThreeCNF::from(term).to_string());
    return Ok(());
}
