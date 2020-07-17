use crate::consts::msg;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, PartialEq)]

pub enum Error {
    #[error("{}: {}", msg::ERR_INVALID_INT_REPR, 0)]
    InvalidIntRepr(std::num::ParseIntError),
}
