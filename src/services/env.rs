use std::{
	collections::HashMap,
	env::{self, VarError},
	error::Error,
	ffi::OsString,
};
use thiserror::Error;

pub trait EnvironmentService {
	type GetVarError: Error;

	/// Gets a string from the environment
	///
	/// # Arguments
	///
	/// * `key` - The name of the environment variable
	///
	/// # Error
	///
	/// This returns a `GetVarError` if a variable of the given name does not exist
	fn var<K: AsRef<str>>(&self, key: K) -> Result<String, Self::GetVarError>;
}

/// Uses the standard library's env implementation
pub struct StandardEnvironmentService;

impl EnvironmentService for StandardEnvironmentService {
	type GetVarError = VarError;

	fn var<K: AsRef<str>>(&self, key: K) -> Result<String, Self::GetVarError> {
		let Ok(key) = <OsString as std::str::FromStr>::from_str(key.as_ref());
		env::var(key)
	}
}

/// Used for testing
pub struct TestEnvironmentService {
	variables: HashMap<String, String>,
}

#[derive(Error, Debug, Clone)]
#[error("No environment variable with the name, {key}")]
pub struct TestVarError {
	key: String,
}

#[cfg(test)]
impl TestEnvironmentService {
	pub fn new() -> Self {
		Self {
			variables: HashMap::new(),
		}
	}

	pub fn add_var<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> Option<String> {
		self.variables
			.insert(key.as_ref().into(), value.as_ref().into())
	}
}

impl EnvironmentService for TestEnvironmentService {
	type GetVarError = TestVarError;

	fn var<K: AsRef<str>>(&self, key: K) -> Result<String, Self::GetVarError> {
		match self.variables.get(key.as_ref()) {
			Some(value) => Ok(value.clone()),
			None => Err(TestVarError {
				key: key.as_ref().into(),
			}),
		}
	}
}
