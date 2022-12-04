// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::fmt::Display;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    FileNotFound { path: PathBuf },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileNotFound { path } => write!(f, "file not found: {}", path.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_file_not_found() {
        let err = Error::FileNotFound {
            path: PathBuf::from("invalid"),
        };
        assert_eq!("file not found: invalid", format!("{}", err));
    }
}
