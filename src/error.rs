use std::fmt::Display;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AppError {
    NotFound,
    URLAParseError
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            AppError::URLAParseError => write!(f, "URL parse error")
        }
    }
}
