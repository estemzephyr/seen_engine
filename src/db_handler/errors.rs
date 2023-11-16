use std::convert::Infallible;
use std::num::ParseIntError;
use mongodb::error::{Error, ErrorKind};

#[derive(Debug)]
pub enum IError {
    ParseInt(ParseIntError),
    MongoDbError(Error),
    StringParseError(String),
}

impl From<ParseIntError> for IError {
    fn from(parse_error: ParseIntError) -> Self {
        IError::ParseInt(parse_error)
    }
}

impl From<Error> for IError {
    fn from(mongo_error: Error) -> Self {
        IError::MongoDbError(mongo_error)
    }
}

fn parse_string(value: &str) -> Result<i16, IError> {
    value.parse().map_err(|_parse_error| IError::StringParseError(value.to_string()))
}