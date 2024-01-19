use std::{
    error,
    fmt::{Display, Formatter, Result},
};

#[derive(Clone, Debug)]
pub enum Error {}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("BRUH")
    }
}
impl error::Error for Error {}
