use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("./src/Day01/data.txt")?;
    let reader = BufReader::new(file);

    let contents = reader.lines().map(|l| l.unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }
}
