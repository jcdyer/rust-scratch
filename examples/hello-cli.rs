extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Hello CLI")
        .version("1.0")
        .author("J. Cliff Dyer <jcd@sdf.org>")
        .about("Says hello to specified user")
        .arg(Arg::with_name("name")
             .short("n")
             .long("name")
             .value_name("NAME")
             .help("The name of the greeted person")
             .takes_value(true))
        .arg(Arg::with_name("lang")
             .short("l")
             .long("lang")
             .value_name("LANGUAGE")
             .help("The language to greet in")
             .takes_value(true))
        .get_matches();
    let name = matches.value_of("name").unwrap_or("world");
    let lang = matches.value_of("lang").unwrap_or("en");

    match lang {
        "en" => println!("Hello {}", name),
        "es" => println!("Buenos Dias {}", name),
        "de" => println!("Guten tag {}", name),
        "en_AU" => println!("G'day {}", name),
        "en_TX" => println!("Howdy {}", name),
        lang => {
            println!("I don't speak {}", lang);
            println!("Hello {}", name);
        }
    }
}
