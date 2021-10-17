use std::fmt;

#[derive(Debug)]
pub enum Error {
    Parameter(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Parameter(ref err) => write!(f, "{}", err.as_str()),
        }
    }
}
