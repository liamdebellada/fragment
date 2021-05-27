use serde_json::{Value};
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("File name missing.");
    let document_data: String = fs::read_to_string(file_name)
        .expect("File not found.")
        .parse()
        .expect("Failed to parse file.");

    let data: &str = r#"
    {
        "name" : "Liam"
    }
    "#;

    let parsed: HashMap<String, Value> = serde_json::from_str(data).expect("Invalid JSON format");

    for key in parsed.keys() {
        println!("{}", key)
    }
}
