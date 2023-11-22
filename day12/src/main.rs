use std::{path::Path, fs::File, io};
use std::io::BufRead;

use serde_json::Value;

fn main() -> io::Result<()> {
    let path = Path::new("input12.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match serde_json::from_str::<serde_json::Value>(&line.unwrap_or_default()) {
            Ok(json) =>{
                println!("Result: {}",get_numbers(&json));
            }         
            Err(e) => panic!("Unable to read json {e}"),
        }
    }
    Ok(())
}

fn get_numbers (value: &Value) -> i32 {
    match value {
        Value::Array(array) => {
            array.iter().map(get_numbers).sum()
        },
        Value::Number(num) => {num.as_i64().unwrap() as i32},
        Value::Object(obj) if ! obj.values().any(|item| item=="red") => {
                obj.values().map(get_numbers).sum()
        },
        _ => {0}
    }
}
