use serde::{Serialize, Deserialize};
use serde_json;

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

    //-----------------------------------------------
    // Serialize kristin struct as JSON to a string.
    //-----------------------------------------------
    let result = serde_json::to_string(&kristin);
    if result.is_ok() {
        let json = result.unwrap();
        println!("Serialized kristin is {}", json);
    } else {
        println!("Unable to serialize kristin");
    }

    //-----------------------------------------------
    // Deserialize brian struct from string.
    //-----------------------------------------------
    let json = r#"
        {
            "name": "Brian",
            "age": 47,
            "is_alive": true
        }
    "#;
    let result = serde_json::from_str(&json);
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
