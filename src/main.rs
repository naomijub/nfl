mod error;
mod model;
mod reader;

use serde_json::to_string;
use reader::read_json;
fn main() {
    let json = read_json("rushing.json");
    println!("{}", to_string(&json.unwrap()).unwrap())
}
