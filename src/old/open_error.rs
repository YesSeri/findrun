use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct OpenError;
impl OpenError {
	fn message(&self) -> &str {
		"Program did not execute correctly. OpenError has been encountered."
	}
}
impl Error for OpenError {}

impl Display for OpenError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}
