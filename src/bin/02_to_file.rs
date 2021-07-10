use serde::{Serialize, Deserialize};
use serde_json;
use std::io::{self};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    is_alive: bool
}

fn main() {
    let kristin = Person {
        name: String::from("Kristin"),
        age: 43,
        is_alive: true
    };

    // I'm using STDOUT as a file handler so I don't have to cleanup
    // any tempory files.
    let output = io::stdout();

    if serde_json::to_writer(output, &kristin).is_ok() {
        println!("\n\nSuccessful write");
    } else {
        println!("\n\nUnsuccessful write");
    }
}
