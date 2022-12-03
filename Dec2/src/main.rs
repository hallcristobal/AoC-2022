use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let mut buffer = BufReader::new(file);
    let mut file_input = String::new();
    buffer.read_to_string(&mut file_input)?;
    
    Ok(())
}
