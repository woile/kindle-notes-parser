//! Core capabilities to process kindle clippings
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

static SEPARATOR: &str = "==========";
pub struct Config {
    pub filename: String,
    pub output_path: String,
    pub output_folder: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let mut args = args.iter();
        args.next();
        let filename = match args.next() {
            Some(arg) => arg.to_owned(),
            None => return Err("Didn't get a file name"),
        };

        let output_path = match args.next() {
            Some(arg) => arg.to_owned(),
            None => String::from("."),
        };

        let output_folder = match args.next() {
            Some(arg) => arg.to_owned(),
            None => String::from("kindle-notes"),
        };

        Ok(Config {
            filename,
            output_path,
            output_folder,
        })
    }
}

fn create_note(filename: &Path, notes: &[&str]) -> std::io::Result<()> {
    let mut book_buffer = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    for note in notes {
        writeln!(&mut book_buffer, "{}\n", note).unwrap();
    }
    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let notes_filepath = Path::new(&config.filename);
    let contents = fs::read_to_string(notes_filepath)?;
    let items = split_notes(&contents);
    let books = classify(&items);
    let output_path = Path::new(&config.output_path).join(config.output_folder);
    fs::create_dir_all(&output_path)?;

    println!(
        "Writing notes to {}",
        output_path.to_str().expect("Path could not be created.")
    );

    books.par_iter().for_each(|(book_name, notes)| {
        let mut filename = output_path.clone().join(book_name);
        filename.set_extension("md");
        let notes = clean(notes);
        create_note(&filename, &notes).unwrap();
    });

    Ok(())
}

pub fn split_notes(contents: &str) -> Vec<&str> {
    contents
        .split(SEPARATOR)
        .filter(|x| !(x.trim().is_empty()))
        .collect()
}

fn clean<'a>(notes: &[&'a str]) -> Vec<&'a str> {
    notes
        .windows(2)
        .map(|win_notes| {
            let first_note = win_notes.first().unwrap();
            if let Some(second_note) = win_notes.last() {
                if first_note.contains(second_note) || second_note.contains(first_note) {
                    return second_note.to_owned();
                }
            }
            first_note.to_owned()
        })
        .collect()
}

fn classify<'a>(items: &[&'a str]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut books = HashMap::new();
    for item in items {
        let fragments: Vec<&str> = item.trim_start().splitn(3, '\n').collect();
        let book_name = fragments.first().unwrap().to_owned().trim();
        let extract = fragments.last().unwrap().to_owned().trim();
        if extract.is_empty() {
            continue;
        }
        books
            .entry(book_name)
            .or_insert_with(Vec::new)
            .push(extract);
    }
    books
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
