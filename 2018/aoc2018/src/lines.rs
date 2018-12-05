use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn lineread(pathname: String) -> Vec<String> {
    let f = File::open(pathname).unwrap();
    return BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(lineread("Cargo.toml".to_string()).len(), 8);
    }
}
