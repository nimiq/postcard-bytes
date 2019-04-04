#![allow(unused_variables)]

use core::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        unimplemented!()
    }
}

pub type Result<T> = ::core::result::Result<T, Error>;

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display
    {
        unimplemented!()
    }
}
