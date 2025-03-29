use std::fs::File;
use std::io::BufReader;

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct User {
  name: String,
  emails: Vec<String>,
}

fn example_with_str() -> serde_json::Result<()> {
  let json_data = r#"{
    "name": "Joe Doe",
    "emails": [
      "doe.joe@personal.com",
      "jdoe@company.com"
    ]
  }"#;

  let data: User = serde_json::from_str(json_data)?;

  println!("{}", serde_json::to_string(&data)?);

  Ok(())
}

fn example_with_file() -> Result <(), Box<dyn std::error::Error>>{
  let file = File::open("data.json")?;
  let buffer = BufReader::new(file);

  let data: Vec<User> = serde_json::from_reader(buffer)?;

  println!("{}", serde_json::to_string(&data).unwrap());

  Ok(())
}

fn main() {
  example_with_str().unwrap();
  example_with_file().unwrap();
}
