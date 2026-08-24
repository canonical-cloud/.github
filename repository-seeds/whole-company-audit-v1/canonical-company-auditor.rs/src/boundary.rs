use serde::{Deserialize, Serialize};

use crate::{AuditError, AuditPackage, EvidenceObservationInputV1};

const BATCH_SCHEMA_VERSION: &str = "canonical.evidence-observation-batch/v1";

/// Versioned batch imported from manual or automated evidence producers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationBatchV1 {
    /// Must equal `canonical.evidence-observation-batch/v1`.
    pub schema_version: String,
    /// Ordered or unordered observation inputs; package creation canonicalizes order.
    pub observations: Vec<EvidenceObservationInputV1>,
}

impl ObservationBatchV1 {
    /// Rejects unknown exchange versions before domain import.
    ///
    /// # Errors
    ///
    /// Returns [`AuditError::UnsupportedSchema`] when the version is not supported.
    pub fn validate_version(&self) -> Result<(), AuditError> {
        if self.schema_version != BATCH_SCHEMA_VERSION {
            return Err(AuditError::UnsupportedSchema(self.schema_version.clone()));
        }
        Ok(())
    }
}

/// OSCAL-compatible back-matter projection kept outside the internal domain model.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OscalBackMatterV1 {
    /// OSCAL resource projections for package observations.
    pub resources: Vec<OscalResourceV1>,
}

/// Minimal OSCAL resource projection for an evidence observation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OscalResourceV1 {
    /// Stable UUID-shaped deterministic identifier.
    pub uuid: String,
    /// Framework-neutral resource title.
    pub title: String,
    /// Portable resource properties.
    pub props: Vec<OscalPropertyV1>,
}

/// OSCAL-compatible name/value property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OscalPropertyV1 {
    /// Property name.
    pub name: String,
    /// Property value.
    pub value: String,
}

impl From<&AuditPackage> for OscalBackMatterV1 {
    fn from(package: &AuditPackage) -> Self {
        let resources = package
            .observations
            .iter()
            .map(|observation| OscalResourceV1 {
                uuid: digest_as_uuid(&observation.observation_id),
                title: observation.evidence_type.clone(),
                props: vec![
                    OscalPropertyV1 {
                        name: "tenant-id".to_owned(),
                        value: observation.tenant_id.as_str().to_owned(),
                    },
                    OscalPropertyV1 {
                        name: "scope-id".to_owned(),
                        value: observation.scope_id.as_str().to_owned(),
                    },
                    OscalPropertyV1 {
                        name: "content-sha256".to_owned(),
                        value: observation.content_sha256.clone(),
                    },
                ],
            })
            .collect();
        Self { resources }
    }
}

fn digest_as_uuid(digest: &str) -> String {
    let Some(prefix) = digest.get(0..32) else {
        return "00000000-0000-0000-0000-000000000000".to_owned();
    };
    if !prefix.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return "00000000-0000-0000-0000-000000000000".to_owned();
    }
    format!(
        "{}-{}-{}-{}-{}",
        &prefix[0..8],
        &prefix[8..12],
        &prefix[12..16],
        &prefix[16..20],
        &prefix[20..32]
    )
}
