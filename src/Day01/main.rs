use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let result = read_data_file("./src/Day01/data.txt");

    let contents = match result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {:?}", error),
    };

    println!("Contents:\n{contents}")
}

fn read_data_file(filepath: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }
}