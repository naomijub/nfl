use rayon::prelude::*;
use serde_json::{Result as JsonResult, Value};
use std::fs::File;
use std::io::{self, BufReader};

use crate::model::{error::Error, json::Player};

pub fn read_json(path: &str) -> Result<Vec<Player>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json_value: JsonResult<Value> = serde_json::from_reader(reader);
    if let Ok(Value::Array(v)) = json_value {
        Ok(v.par_iter()
            .map(|e| Player::from_value(e).unwrap())
            .collect::<Vec<Player>>())
    } else {
        Err(Error::JsonReaderError)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::LoadDataError(error.to_string())
    }
}
