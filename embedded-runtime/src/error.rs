//! A runtime related error type

use core::fmt::{self, Display, Formatter};

/// Creates a new error
#[macro_export]
macro_rules! err {
    ($message:expr) => {{
        $crate::error::Error { message: $message, location: (file!(), line!()) }
    }};
}

/// An error
///
/// # Note
/// This type should be constructed using the `error`-macro
#[derive(Debug, Clone, Copy)]
pub struct Error {
    /// The error message
    #[doc(hidden)]
    pub message: &'static str,
    /// The error location (file, line)
    #[doc(hidden)]
    pub location: (&'static str, u32),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (file, line) = self.location;
        write!(f, "{} at {}:{}", self.message, file, line)
    }
}
