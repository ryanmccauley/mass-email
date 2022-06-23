use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::fs;
use serde_json::Value;

struct Contact {
    name: String,
    email: String,
    phone: i32,
}

#[derive(Debug)]
enum LoadError {
    FileNotFoundError(std::io::Error),
    IOError(std::io::Error),
    JsonError(serde_json::Error),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::FileNotFoundError(file_not_found_error) => write!(f, "{}", file_not_found_error),
            LoadError::IOError(io_error) => write!(f, "{}", io_error),
            LoadError::JsonError(json_error) => write!(f, "{}", json_error),
        }
    }
}

impl Error for LoadError {}

const FILE_NAME: &str = "contents.json";

fn load_from_file() -> Result<(), LoadError> {
    let mut file = match fs::File::open(FILE_NAME) {
        Ok(file) => file,
        Err(err) => return Err(LoadError::FileNotFoundError(err)),
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(err) => return Err(LoadError::IOError(err)),
    }

    let json = match serde_json::from_str::<serde_json::Value>(&buffer) {
        Ok(_) => {},
        Err(err) => return Err(LoadError::JsonError(err)),
    };

    println!("{}", json.0);

    Ok(())
}

fn main() {
    match load_from_file() {
        Ok(_) => println!("Succesfully loaded from file..."),
        Err(err) => println!("{}", err),
    }
}
