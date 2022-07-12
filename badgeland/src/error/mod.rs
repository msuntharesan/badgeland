use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid Color")]
pub struct ColorError;

#[cfg(feature = "static_icons")]
#[derive(Error, Debug)]
#[error("Invalid Icon")]
pub struct IconError;

#[derive(Error, Debug)]
#[error("Invalid Size")]
pub struct SizeError;

#[derive(Error, Debug)]
#[error("Invalid Style")]
pub struct StyleError;
