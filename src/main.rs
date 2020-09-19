mod error;
mod model;
mod reader;

use reader::read_json;
use serde_json::to_string;
fn main() {
    let json = read_json("rushing.json");
    println!("{}", to_string(&json.unwrap()).unwrap())
}
