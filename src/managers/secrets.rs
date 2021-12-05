use crate::services::env::{EnvironmentService, StandardEnvironmentService};

const DISCORD_TOKEN_KEY: &str = "DISCORD_TOKEN";

pub struct SecretManager<E: EnvironmentService = StandardEnvironmentService> {
	environment_service: E,
}

impl Default for SecretManager {
	fn default() -> Self {
		Self::new(StandardEnvironmentService)
	}
}

impl<E: EnvironmentService> SecretManager<E> {
	/// Create a new environment manager
	pub const fn new(environment_service: E) -> Self {
		Self {
			environment_service,
		}
	}

	/// Get the secret discord token from the environment variable: "DISCORD_TOKEN"
	///
	/// # Errors
	///
	/// Returns an error if the "DISCORD_TOKEN" environment variable doesn't exist
	pub fn discord_token(&self) -> Result<String, E::GetVarError> {
		self.environment_service.var(DISCORD_TOKEN_KEY)
	}
}

#[cfg(test)]
mod tests {
	use crate::services::env::TestEnvironmentService;

	use super::*;

	#[test]
	fn discord_token() {
		// arrange
		let mut environment_service = TestEnvironmentService::new();
		let token = "fhsagfbl";
		environment_service.add_var(DISCORD_TOKEN_KEY, token);

		let manager = SecretManager::new(environment_service);

		// act
		let result = manager.discord_token();

		// assert
		assert!(result.is_ok());
		assert_eq!(token, result.unwrap());
	}

	#[test]
	fn discord_token_fail() {
		// arrange
		let service = TestEnvironmentService::new();
		let manager = SecretManager::new(service);

		// act
		let result = manager.discord_token();

		// assert
		assert!(result.is_err());
	}
}
