//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------

/*
pub struct Error {
    pub messages: std::vec::Vec<std::string::String>,
}
*/

#[derive(Debug)]
pub enum Error {
    UnableToRemovePrim,
    MessageOnly(std::string::String),
    NullError(std::ffi::NulError),
    Utf8(std::str::Utf8Error),
}

impl std::convert::From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Self {
        Error::NullError(error)
    }
}

impl std::convert::From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::MessageOnly(error.to_string())
    }
}

impl std::convert::From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Utf8(error)
    }
}
