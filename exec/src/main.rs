use std::io;

fn main() -> io::Result<()> {
    println!("What app would ");
    let mut buffer = String::new();    
    io::stdin().read_line(&mut buffer)?;
    let s = String::from("hello computer\n");
    if buffer == s {
        println!("Oh, that's cute, :)");
    }

    return Ok(());
}
