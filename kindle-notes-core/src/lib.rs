//! Core capabilities to process kindle clippings
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

static SEPARATOR: &str = "==========";
pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let items = split_notes(&contents);
    let books = classify(&items);
    println!("{:?}", books);

    Ok(())
}

pub fn split_notes(contents: &str) -> Vec<&str> {
    contents
        .split(SEPARATOR)
        .filter(|x| !(x.trim().is_empty()))
        .collect()
}

fn classify<'a>(items: &[&'a str]) -> HashMap<&'a str, Vec<&'a str>> {
    items
        .iter()
        .map(|item| {
            let fragments: Vec<&str> = item.trim_start().splitn(3, '\n').collect();
            (
                fragments.first().unwrap().to_owned(),
                vec![fragments.last().unwrap().to_owned()],
            )
        })
        .collect()
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
