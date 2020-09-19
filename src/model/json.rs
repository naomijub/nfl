use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Player {
    #[serde(rename(serialize = "Name"))]
    name: String,
    #[serde(rename(serialize = "Team"))]
    team: String,
    #[serde(rename(serialize = "Position"))]
    position: String,
    #[serde(rename(serialize = "Rushing Attempts Per Game Average"))]
    avg_attempts_per_game: f64,
    #[serde(rename(serialize = "Rushing Attempts"))]
    attemps: f64,
    #[serde(rename(serialize = "Total Rushing Yards"))]
    total_rushing_yards: f64,
    #[serde(rename(serialize = "Rushing Average Yards Per Attempt"))]
    average_rushing_yards_per_attemp: f64,
    #[serde(rename(serialize = "Rushing Yards Per Game"))]
    rushing_yards_per_game: f64,
    #[serde(rename(serialize = "Total Rushing Touchdowns"))]
    total_rushing_touchdowns: f64,
    #[serde(rename(serialize = "Longest Rush"))]
    largest_rust: String,
    #[serde(rename(serialize = "Rushing First Downs"))]
    rushing_first_downs: f64,
    #[serde(rename(serialize = "Rushing First Down Percentage"))]
    rushing_first_downs_percentage: f64,
    #[serde(rename(serialize = "Rushing 20+ Yards Each"))]
    rushing_20_yards: f64,
    #[serde(rename(serialize = "Rushing 40+ Yards Each"))]
    rushing_40_yards: f64,
    #[serde(rename(serialize = "Rushing Fumbles"))]
    rushing_fumbles: f64,
}

impl Player {
    pub fn from_value(object: &Value) -> Result<Self, String> {
        if let Value::Object(obj) = object {
            Ok(Player {
                name: obj["Player"].to_string(),
                team: obj["Team"].to_string(),
                position: obj["Pos"].to_string(),
                avg_attempts_per_game: match &obj["Att/G"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                attemps: match &obj["Att"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                total_rushing_yards: match &obj["Yds"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                average_rushing_yards_per_attemp: match &obj["Avg"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                rushing_yards_per_game: match &obj["Yds/G"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                total_rushing_touchdowns: match &obj["TD"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                largest_rust: obj["Lng"].to_string(),
                rushing_first_downs: match &obj["1st"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                rushing_first_downs_percentage: match &obj["1st%"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                rushing_20_yards: match &obj["20+"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                rushing_40_yards: match &obj["40+"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
                rushing_fumbles: match &obj["FUM"] {
                    Value::Number(n) if n.is_f64() => n.as_f64().unwrap(),
                    Value::Number(n) => n.to_string().parse::<f64>().unwrap(),
                    Value::String(n) => n.replace(",", "").parse::<f64>().unwrap(),
                    _ => return Err("Cagou".to_string()),
                },
            })
        } else {
            Err("deu pau".to_string())
        }
    }
}
