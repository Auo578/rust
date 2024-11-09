use std::error::Error as StdError;


pub type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug)]
pub struct Error{
    inner: BoxError,
}

impl  Error {
    pub fn new(error:impl Into<BoxError> ) -> Self{
        Self{inner:error.into()}
    }
}