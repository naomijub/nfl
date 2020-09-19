mod error;
mod model;
mod reader;

use reader::read_json;
fn main() {
    let json = read_json("rushing.json");
    println!("{:?}", json)
}
