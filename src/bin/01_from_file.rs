use serde::{Serialize, Deserialize};
use serde_json;

use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    is_alive: bool
}

fn main() {
    //-----------------------------------------------
    // Deserialize brian struct from file.
    //-----------------------------------------------

    // Load from file
    let path = "samples/simple.json";
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    // Deserialize file buffer
    //  Docs - https://docs.serde.rs/serde_json/fn.from_reader.html
    let result = serde_json::from_reader(reader);
    if result.is_ok() {
        let brian: Person = result.unwrap();

        // This should come back as a String - so no quotes around the value.
        println!("Name: {}", brian.name);

        // This leverages the Debug trait.
        println!("Deserialized brian is {:?}", brian);
    } else {
        println!("Unable to deserialize brian");
    }
}
