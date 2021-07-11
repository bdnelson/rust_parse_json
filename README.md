Parsing JSON in Rust using Serde
================================

This set of samples is to help me get comfortable with parsing JSON data in Rust using the Serde crates.  Serde is an actively maintained and widely used crate (almost 50 million downloads all-time) so this approach is a community and time tested solution to parsing JSON.

I will be concentrating on the most automated approaches to parsing of JSON using Serde (derive approach), but I will eventually throw some examples of poor JSON structures that I have had to parse at work.

**This project is only meant for training/exploration.  It should not be used for anything that matters.  Reference the Serde documentation for any critical answers.**


## Serde Resources

* [Home Page](https://serde.rs/)
* Main crate (serde)
    * [GitHub](https://github.com/serde-rs/serde)
    * [Crates.io](https://crates.io/crates/serde)
    * [Docs](https://docs.serde.rs/serde/)
* JSON crate (serde_json)
    * [GitHub](https://github.com/serde-rs/json)
    * [Crates.io](https://crates.io/crates/serde_json)
    * [Docs](https://docs.serde.rs/serde/json)

## Topics to Cover

* Serialization
    - [x] To memory
    - [x] To IO buffer
* Deserialization
    - [x] From memory
    - [x] From IO buffer
* Advanced Structures
    - [x] Enums
    - [x] Tuples
    - [x] Arrays
    - [x] Vectors
    - [ ] Hashes
    - [ ] Objects
* [ ] Optional attributes
* [ ] Default values
* [ ] Error handling

## Examples

* `00_simple` - Serialize/deserialize simple structures in memory.
* `01_from_file` - Deserialize simple structure from file IO.
* `02_to_file` - Serialize simple structure to file.
* `03_enums` - Enums in the data structure.

All samples can be executed by `cargo run --bin EXAMPLE_NAME`.
