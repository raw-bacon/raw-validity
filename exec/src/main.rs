use std::io;
use::parsing::parse_l_group_term::parse;

fn main() -> io::Result<()> {
    println!("Please enter a term.");
    let mut buffer = String::new();    
    io::stdin().read_line(&mut buffer)?;
    
    let term = parse(&buffer).expect("Something went wrong ...");
    println!("{}", term.to_string());
    return Ok(());
}
