pub mod error;
pub mod expr;
pub mod rlox;
pub mod scanner;
pub mod token;

use error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
