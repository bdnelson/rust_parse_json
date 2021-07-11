use serde::{Serialize, Deserialize};
use serde_json;
use std::io::{self};

#[derive(Serialize, Deserialize, Debug)]
enum PrimaryHand {
    Right,
    Left,
    Both
}

#[derive(Serialize, Deserialize, Debug)]
enum EyeColor {
    Black,
    Blue,
    Brown,
    Green,
    Purple
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    is_alive: bool,
    primary_hand: PrimaryHand,
    eye_color: EyeColor
}

fn main() {
    let person = Person {
        name: String::from("Brian"),
        age: 47,
        is_alive: true,
        primary_hand: PrimaryHand::Right,
        eye_color: EyeColor::Green
    };
    let output = io::stdout();

    if serde_json::to_writer(output, &person).is_ok() {
        println!("\n\nSuccessful write\n");
    } else {
        println!("\n\nUnsuccessful write\n");
    }

    let json = r#" {"name":"Brian","age":47,"is_alive":true,"primary_hand":"Right","eye_color":"Green"} "#;
    let result = serde_json::from_str(&json);
    if result.is_ok() {
        let brian: Person = result.unwrap();

        // This should come back as a String - so no quotes around the value.
        println!("Name: {}", brian.name);
        match brian.eye_color {
            EyeColor::Green => println!("Eye color correct"),
            _ => println!("Eye color incorrect")
        }

        // This leverages the Debug trait.
        println!("Deserialized brian is {:?}", brian);
    } else {
        println!("Unable to deserialize brian");
    }
}
