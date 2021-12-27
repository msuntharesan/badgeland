use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid Color")]
pub struct ColorError;

#[derive(Error, Debug)]
#[error("Invalid Icon Name {}", name)]
pub struct IconError<'a> {
    pub name: &'a str,
}

#[derive(Error, Debug)]
#[error("Invalid Size")]
pub struct SizeError;

#[derive(Error, Debug)]
#[error("Invalid Size")]
pub struct StyleError;
