use std::env;

use serde::{Deserialize, Serialize};

use crate::ConnectorError;

const TENANT_VARIABLE: &str = "CANONICAL_TENANT_ID";
const ORGANIZATION_VARIABLE: &str = "CANONICAL_GITHUB_ORGANIZATION";
const CREDENTIAL_REFERENCE_VARIABLE: &str = "CANONICAL_GITHUB_CREDENTIAL_REF";

/// A reference to a credential, never credential material itself.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum CredentialReference {
    /// Name of an environment variable holding credential material.
    Environment {
        /// Safe variable name, not its value.
        variable: String,
    },
    /// Provider-qualified secret-store locator.
    SecretStore {
        /// Safe secret locator, not resolved content.
        locator: String,
    },
}

impl CredentialReference {
    /// Parses `env:VARIABLE_NAME` or `secret-store:provider/path` without accepting raw material.
    ///
    /// # Errors
    ///
    /// Returns [`ConnectorError::InvalidConfiguration`] without echoing the supplied value when the
    /// string is not a valid reference.
    pub fn parse(value: &str) -> Result<Self, ConnectorError> {
        if let Some(variable) = value.strip_prefix("env:")
            && !variable.is_empty()
            && variable.len() <= 128
            && variable
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
        {
            return Ok(Self::Environment {
                variable: variable.to_owned(),
            });
        }
        if let Some(locator) = value.strip_prefix("secret-store:")
            && !locator.is_empty()
            && locator.len() <= 240
            && locator.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_' | b'.' | b':')
            })
            && !locator.contains("..")
        {
            return Ok(Self::SecretStore {
                locator: locator.to_owned(),
            });
        }
        Err(ConnectorError::InvalidConfiguration(
            "credential must be an env: or secret-store: reference; raw values are forbidden",
        ))
    }
}

/// Explicit live GitHub tenant authority loaded only from named environment configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveGithubAuthority {
    tenant_id: String,
    organization: String,
    credential_reference: CredentialReference,
}

impl LiveGithubAuthority {
    /// Loads tenant, organization, and credential reference from named environment variables.
    ///
    /// # Errors
    ///
    /// Returns a typed missing/invalid configuration error. Values are never included in errors.
    pub fn from_environment() -> Result<Self, ConnectorError> {
        let tenant_id = required_environment(TENANT_VARIABLE)?;
        let organization = required_environment(ORGANIZATION_VARIABLE)?;
        let credential_reference_value = required_environment(CREDENTIAL_REFERENCE_VARIABLE)?;
        Self::new(
            &tenant_id,
            &organization,
            CredentialReference::parse(&credential_reference_value)?,
        )
    }

    /// Creates an authority from already isolated configuration values.
    ///
    /// # Errors
    ///
    /// Returns [`ConnectorError::InvalidConfiguration`] for an unsafe tenant or organization name.
    pub fn new(
        tenant_id: &str,
        organization: &str,
        credential_reference: CredentialReference,
    ) -> Result<Self, ConnectorError> {
        validate_safe_name("tenant", tenant_id)?;
        validate_safe_name("organization", organization)?;
        Ok(Self {
            tenant_id: tenant_id.to_owned(),
            organization: organization.to_owned(),
            credential_reference,
        })
    }

    /// Returns the configured tenant identifier.
    #[must_use]
    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    /// Returns the configured GitHub organization.
    #[must_use]
    pub fn organization(&self) -> &str {
        &self.organization
    }

    /// Returns the safe credential reference without resolving credential material.
    #[must_use]
    pub fn credential_reference(&self) -> &CredentialReference {
        &self.credential_reference
    }

    /// Fails closed because this seed intentionally contains no production HTTP authority.
    ///
    /// # Errors
    ///
    /// Always returns [`ConnectorError::ProductionTransportUnavailable`] until a separately
    /// reviewed, read-only production transport is installed after repository publication.
    pub fn require_production_transport(&self) -> Result<(), ConnectorError> {
        Err(ConnectorError::ProductionTransportUnavailable)
    }
}

fn required_environment(name: &'static str) -> Result<String, ConnectorError> {
    env::var(name).map_err(|_| ConnectorError::MissingEnvironment(name))
}

fn validate_safe_name(field: &'static str, value: &str) -> Result<(), ConnectorError> {
    if value.is_empty()
        || value.len() > 120
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
    {
        return Err(ConnectorError::InvalidConfiguration(field));
    }
    Ok(())
}
