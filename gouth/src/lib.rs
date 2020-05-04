//! This library provides auto-renewed tokens for GCP service authentication.
//!
//! # Example
//! ```no_run
//! use gouth::Token;
//!
//! let token = Token::new().unwrap();
//! println!("authorization: {}", token.header_value().unwrap());
//! ```

mod error;
mod source;
mod token;

pub use error::{Error, ErrorKind, Result};
pub use token::{Builder, Token};
