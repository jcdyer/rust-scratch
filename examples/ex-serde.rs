#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    name: String,
    email: String,
    address: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct RefContact<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow)]
    email: Cow<'a, str>,
    #[serde(borrow)]
    address: Cow<'a, str>,
}

fn main() {
    let j = r#"{
        "name": "Cliff", 
        "email": "jcd@sdf.org",
        "address": "2346 Huron St.\nDurham, NC 27707"
    }"#;
    let contact: Contact = serde_json::de::from_str(j).unwrap();
    let refcontact: RefContact = serde_json::de::from_str(j).unwrap();
    
    match refcontact.name {
        Cow::Borrowed(_) => println!("Borrowed name"),
        Cow::Owned(_) => println!("Owned name"),
    }
    match refcontact.email {
        Cow::Borrowed(_) => println!("Borrowed email"),
        Cow::Owned(_) => println!("Owned email"),
    }
    match refcontact.address {
        Cow::Borrowed(_) => println!("Borrowed address"),
        Cow::Owned(_) => println!("Owned address"),
    }
    println!("{:?}", contact);
    println!("{:?}", refcontact);
    println!("{} lives at {}", contact.name, contact.address);
    println!("{} lives at {}", refcontact.name, refcontact.address);
}
