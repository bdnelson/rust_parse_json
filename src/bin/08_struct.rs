use serde::{Serialize, Deserialize};
use serde_json;
use std::io::{self};
use std::collections::HashMap;

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
struct Career {
    title: String,
    company: String,
    months_in_position: i32,
    direct_reports: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    is_alive: bool,
    primary_hand: PrimaryHand,
    eye_color: EyeColor,
    dets: (i32, i32, String),
    friends: [String; 4],
    favorite_numbers: Vec<i32>,
    emails: HashMap<String, String>,
    phones: HashMap<String, String>,
    career: Career
}

fn main() {
    let mut person = Person {
        name: String::from("Brian"),
        age: 47,
        is_alive: true,
        primary_hand: PrimaryHand::Right,
        eye_color: EyeColor::Green,
        dets: (4, 32, String::from("BDN")),
        friends: [
            String::from("Chance"),
            String::from("Justin"),
            String::from("Dusty"),
            String::from("Ryan")
        ],
        favorite_numbers: vec![13, 67, 74],
        emails: HashMap::new(),
        phones: HashMap::new(),
        career: Career {
            title: String::from("Technical Lead"),
            company: String::from("HeavyWorth"),
            months_in_position: 28,
            direct_reports: vec![String::from("Glenda"), String::from("Marabeth"), String::from("Jim")]
        }
    };
    person.emails.insert(String::from("work"), String::from("brian@forefrontlabs.com"));
    person.emails.insert(String::from("home"), String::from("bdnelson@gmail.com"));

    let output = io::stdout();

    if serde_json::to_writer(output, &person).is_ok() {
        println!("\n\nSuccessful write\n");
    } else {
        println!("\n\nUnsuccessful write\n");
    }

    let json = r#"
        {
            "name":"Brian",
            "age":47,
            "is_alive":true,
            "primary_hand":"Right",
            "eye_color":"Green",
            "dets":[4,32,"BDN"],
            "friends": ["Chance", "Justin", "Dusty", "Ryan"],
            "favorite_numbers": [0,555,67,74,13],
            "emails": {"home":"bdnelson@gmail.com","work":"brian@forefrontlabs.com"},
            "phones": {},
            "career":{
                "title":"Technical Lead",
                "company":"HeavyWorth",
                "months_in_position":28,
                "direct_reports": ["Glenda","Marabeth","Jim"]
            }
        }
    "#;
    let result = serde_json::from_str(&json);
    if result.is_ok() {
        let brian: Person = result.unwrap();

        // This should come back as a String - so no quotes around the value.
        println!("Name: {}", brian.name);
        match brian.eye_color {
            EyeColor::Green => println!("Eye color correct"),
            _ => println!("Eye color incorrect")
        }
        let (mental_age, iq, initials) = &brian.dets;
        println!("Mental age of {} with IQ of {} for {}", mental_age, iq, initials);

        println!("Number of friends: {}", brian.friends.len());

        // This leverages the Debug trait.
        println!("Deserialized brian is {:?}", brian);
    } else {
        println!("Unable to deserialize brian");
    }
}
