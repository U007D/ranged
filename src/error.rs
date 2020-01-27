use crate::consts::msg;
use std::fmt::{Display, Error as StdFmtError, Formatter};
use thiserror::Error;

/// `Error` is a newtype (single-field tuple struct as opposed to an `enum`) with a private inner
/// type.  This prevents leaking internal dependencies to external consumers.
///
/// In this example, "external" is defined as outside of the current crate via `pub(crate)` on `.0`.
/// "External" can be defined as per-module or even per-file, depending on the need, but in my
/// experience, per-crate is reasonable and has been sufficient.
#[derive(Debug, Error)]
pub struct Error(pub(crate) InnerError);

impl Error {
    /// The crate developer implements `kind()` to specify the mapping between an `InnerError`
    /// variant and `ErrorKind`.
    #[cfg(feature = "const_fn")]
    pub const fn kind(&self) -> ErrorKind {
        #[allow(clippy::unneeded_field_pattern)]
        match self.0 {
            InnerError::InvalidIntRepr(_) => ErrorKind::InvalidStringToValueConversion,
        }
    }
    #[cfg(not(feature = "const_fn"))]
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        #[allow(clippy::unneeded_field_pattern)]
        match self.0 {
            InnerError::InvalidIntRepr(_) => ErrorKind::InvalidStringToValueConversion,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), StdFmtError> {
        write!(f, "{}", self)
    }
}

/// The set of errors returnable by this crate.  Note that functions which can truly only fail
/// for one reason should generally return `Option<T>`, not `Result<T>`.  Because `InnerError` is
/// private, variant payloads (typically external error types when representing external errors) are
/// hidden from external consumers of `Error`.
#[derive(Debug, Error)]
pub(crate) enum InnerError {
    #[error("{}: {}", msg::ERR_INVALID_INT_REPR, 0)]
    InvalidIntRepr(#[from] std::num::ParseIntError),
}

/// `ErrorKind` is idiomatic Rust for a static dependency-free exportable, comparable `Error` API.
/// The crate developer decides the level of granularity and context to export.  Context data (e.g.
/// the name of the resource that was not found) may be returned in an `ErrorKind` variant payload
/// (e.g. `ErrorKind::InvalidDestination(std::net::IpAddr)`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::pedantic)]
pub enum ErrorKind {
    InvalidStringToValueConversion,
}
