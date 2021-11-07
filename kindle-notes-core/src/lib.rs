//! Core capabilities to process kindle clippings
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let items = split_notes(&contents);
    // let r: Vec<&str> = items.iter().map(|x| x.split('\n')).collect();
    for item in items {
        for i in item.split('\n') {
            println!("{:?}", i);
        }
    }
    // println!("TEXT:\n {:?}", r);
    Ok(())
}

pub fn split_notes(contents: &str) -> Vec<&str> {
    contents.split("==========").filter(|x| !x.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_contents_to_two() {
        let items = split_notes("==========\nDesigning Data-Intensive Applications");
        println!("{:?}", items);
        assert_eq!(1, items.len());
    }
}
