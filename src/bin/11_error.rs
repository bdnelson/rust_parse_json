use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
enum PrimaryHand {
    Right,
    Left,
    Both
}
impl PrimaryHand {
    fn default() -> Self { PrimaryHand::Both }
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
    #[serde(default = "PrimaryHand::default")]
    primary_hand: PrimaryHand,
    eye_color: EyeColor
}

fn main() {
    let json = r#" {"name":"Brian","age":47,"eye_color":"Green","is_alive": true "#;
    let result = serde_json::from_str(&json);
    if result.is_ok() {
        let brian: Person = result.unwrap();
        // This leverages the Debug trait.
        println!("Deserialized brian is {:?}", brian);
    } else {
        let err = result.err().unwrap();
        println!("Unable to deserialize brian");
        println!("   Error --> {:?}", err);
        println!("   Category --> {:?}", err.classify());
        println!("   Line --> {:?}", err.line());
        println!("   Column --> {:?}", err.column());

        if err.is_data() {
            println!("The JSON does not match the expected specification.");
        } else if err.is_syntax() || err.is_eof() {
            println!("Please check the input JSON and try again.");
        } else {
            println!("Boom!");
            panic!();
        }
    }
}
