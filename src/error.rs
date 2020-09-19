#[derive(Debug)]
pub enum Error {
    JsonReaderError,
    LoadDataError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsonReaderError => write!(f, "Could not read Json data"),
            Error::LoadDataError(err) => write!(f, "Data could not be loaded due to {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::JsonReaderError => "Could not read Json data",
            Error::LoadDataError(_) => "Data could not be loaded",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}
