use serde::Serialize;
use serde_json::Value;
use juniper::GraphQLObject;

use crate::model::error::Error;

#[derive(Debug, PartialEq, Serialize, GraphQLObject)]
pub struct Player {
    #[serde(rename(serialize = "Name"))]
    pub name: String,
    #[serde(rename(serialize = "Team"))]
    pub team: String,
    #[serde(rename(serialize = "Position"))]
    pub position: String,
    #[serde(rename(serialize = "Rushing Attempts Per Game Average"))]
    pub avg_attempts_per_game: f64,
    #[serde(rename(serialize = "Rushing Attempts"))]
    pub attemps: i32,
    #[serde(rename(serialize = "Total Rushing Yards"))]
    pub total_rushing_yards: i32,
    #[serde(rename(serialize = "Rushing Average Yards Per Attempt"))]
    pub average_rushing_yards_per_attemp: f64,
    #[serde(rename(serialize = "Rushing Yards Per Game"))]
    pub rushing_yards_per_game: f64,
    #[serde(rename(serialize = "Total Rushing Touchdowns"))]
    pub total_rushing_touchdowns: i32,
    #[serde(rename(serialize = "Longest Rush"))]
    pub largest_rust: String,
    #[serde(rename(serialize = "Rushing First Downs"))]
    pub rushing_first_downs: f64,
    #[serde(rename(serialize = "Rushing First Down Percentage"))]
    pub rushing_first_downs_percentage: f64,
    #[serde(rename(serialize = "Rushing 20+ Yards Each"))]
    pub rushing_20_yards: f64,
    #[serde(rename(serialize = "Rushing 40+ Yards Each"))]
    pub rushing_40_yards: f64,
    #[serde(rename(serialize = "Rushing Fumbles"))]
    pub rushing_fumbles: i32,
}

impl Player {
    pub fn from_value(object: &Value) -> Result<Self, Error> {
        if let Value::Object(obj) = object {
            Ok(Player {
                name: obj["Player"].as_str().unwrap().to_string(),
                team: obj["Team"].as_str().unwrap().to_string(),
                position: obj["Pos"].as_str().unwrap().to_string(),
                avg_attempts_per_game: match &obj["Att/G"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err(Error::AttributeParseError("Att/G".to_string())),
                },
                attemps: match &obj["Att"] {
                    Value::Number(n) if n.is_i64() => n.as_i64().unwrap() as i32,
                    Value::Number(n) => n.to_string().parse::<i32>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<i32>().unwrap(),
                    _ => return Err(Error::AttributeParseError("Att".to_string())),
                },
                total_rushing_yards: match &obj["Yds"] {
                    Value::Number(n) if n.is_i64() => n.as_i64().unwrap() as i32,
                    Value::Number(n) => n.to_string().parse::<i32>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<i32>().unwrap(),
                    _ => return Err(Error::AttributeParseError("Yds".to_string())),
                },
                average_rushing_yards_per_attemp: match &obj["Avg"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err(Error::AttributeParseError("Avg".to_string())),
                },
                rushing_yards_per_game: match &obj["Yds/G"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err(Error::AttributeParseError("Yds/G".to_string())),
                },
                total_rushing_touchdowns: match &obj["TD"] {
                    Value::Number(n) if n.is_u64() => n.as_i64().unwrap() as i32,
                    Value::Number(n) => n.to_string().parse::<i32>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<i32>().unwrap(),
                    _ => return Err(Error::AttributeParseError("TD".to_string())),
                },
                largest_rust: obj["Lng"].as_str().unwrap().to_string(),
                rushing_first_downs: match &obj["1st"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err(Error::AttributeParseError("1st".to_string())),
                },
                rushing_first_downs_percentage: match &obj["1st%"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err(Error::AttributeParseError("1st%".to_string())),
                },
                rushing_20_yards: match &obj["20+"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err(Error::AttributeParseError("20+".to_string())),
                },
                rushing_40_yards: match &obj["40+"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err(Error::AttributeParseError("40+".to_string())),
                },
                rushing_fumbles: match &obj["FUM"] {
                    Value::Number(n) if n.is_u64() => n.as_i64().unwrap() as i32,
                    Value::Number(n) => n.to_string().parse::<i32>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<i32>().unwrap(),
                    _ => return Err(Error::AttributeParseError("FUM".to_string())),
                },
            })
        } else {
            Err(Error::JsonReaderError)
        }
    }
}
