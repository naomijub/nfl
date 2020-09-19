use juniper::{FieldError, IntoFieldError, graphql_value};

#[derive(Debug)]
pub enum Error {
    JsonReaderError,
    LoadDataError(String),
    AttributeParseError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsonReaderError => write!(f, "Could not read Json data"),
            Error::LoadDataError(err) => write!(f, "Data could not be loaded due to {}", err),
            Error::AttributeParseError(att) => write!(f, "Could not parse Json attribute {}", att),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::JsonReaderError => "Could not read Json data",
            Error::LoadDataError(_) => "Data could not be loaded",
            Error::AttributeParseError(_) => "Could not parse Json attribute",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}

impl IntoFieldError for Error {
    fn into_field_error(self) -> FieldError {
        match self {
            Error::JsonReaderError => FieldError::new(
                "Could not read Json data",
                graphql_value!({
                    "type": "JSON READER ERROR"
                }),
            ),
            Error::LoadDataError(err) => FieldError::new(
                &format!("Data could not be loaded due to {}", err),
                graphql_value!({
                    "type": "LOAD DATA ERROR"
                }),
            ),
            Error::AttributeParseError(att) => FieldError::new(
                &format!("Could not parse Json attribute {}", att),
                graphql_value!({
                    "type": "ATTRIBUTE PARSE ERROR"
                }),
            ),
        }
        
    }
}