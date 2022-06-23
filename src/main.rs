use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    name: String,
    email: String,
    phone: String,
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Contact(name={}, email={}, phone={})", self.name, self.email, self.phone);
        Ok(())
    }
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

const CONTACTS_FILE_NAME: &str = "contents.json";

fn load_from_file() -> Result<Vec<Contact>, LoadError> {
    let mut file = match fs::File::open(CONTACTS_FILE_NAME) {
        Ok(file) => file,
        Err(err) => return Err(LoadError::FileNotFoundError(err)),
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(err) => return Err(LoadError::IOError(err)),
    }

    let contacts: Vec<Contact> = return match serde_json::from_str::<Vec<Contact>>(&buffer) {
        Ok(contacts) => Ok(contacts),
        Err(err) => Err(LoadError::JsonError(err)),
    };
}

const EMAIL_CONTENT_FILE_NAME: &str = "email.txt";

fn load_email_content() -> Result<String, LoadError> {
    let mut file = match fs::File::open(EMAIL_CONTENT_FILE_NAME) {
        Ok(file) => file,
        Err(err) => return Err(LoadError::FileNotFoundError(err)),
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Ok(_) => {},
        Err(err) => return Err(LoadError::IOError(err))
    };

    Ok(buffer)
}

struct EmailSendProperties {
    to: String,
    from: String,
    subject: String,
    body: String,
}

fn send_email(props: EmailSendProperties) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}, {}, {}, {}", props.to, props.from, props.subject, props.body);
    Ok(())
}

fn main() {
    let contacts = match load_from_file() {
        Ok(contacts) => {
            println!("Successfully loaded {} contacts from {}...", contacts.len(), CONTACTS_FILE_NAME);
            contacts
        },
        Err(err) => {
            panic!("{}", err);
        }
    };

    let email_contents = match load_email_content() {
        Ok(contents) => contents,
        Err(err) => {
            panic!("{}", err);
        }
    };

    for contact in contacts {
        let formatted_contents = email_contents.replace("{name}", &contact.name);

        match send_email(EmailSendProperties {
            to: contact.email.clone(),
            from: String::from("ryanmcly@gmail.com"),
            subject: String::from("Test Subject"),
            body: formatted_contents
        }) {
            Ok(_) => {
                println!("Email send to {}", contact.email)
            },
            Err(err) => {
                println!("Error sending to {}: {}", contact.email, err)
            }
        }
    }
}
