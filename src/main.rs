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

const CONTACTS_FILE_NAME: &str = "contents.json";

fn load_contacts() -> Option<Vec<Contact>> {
    let mut buffer = String::new();
    fs::File::open(CONTACTS_FILE_NAME).ok()?.read_to_string(&mut buffer);
    match serde_json::from_str::<Vec<Contact>>(&buffer) {
        Ok(contacts) => Some(contacts),
        Err(_) => None,
    }
}

const EMAIL_CONTENT_FILE_NAME: &str = "email.txt";

fn load_email_content() -> Option<String> {
    let mut buffer = String::new();
    fs::File::open(EMAIL_CONTENT_FILE_NAME).ok()?.read_to_string(&mut buffer);
    Some(buffer)
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
    let contacts = match load_contacts() {
        Some(contacts) => {
            println!("Successfully loaded {} contacts from {}...", contacts.len(), CONTACTS_FILE_NAME);
            contacts
        },
        None => panic!("Error reading {}", CONTACTS_FILE_NAME),
    };

    let email_contents = match load_email_content() {
        Some(contents) => contents,
        None => panic!("Error reading email contents."),
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
                println!("Email sent to {}", contact.email)
            },
            Err(err) => {
                println!("Error sending to {}: {}", contact.email, err)
            }
        }
    }
}
