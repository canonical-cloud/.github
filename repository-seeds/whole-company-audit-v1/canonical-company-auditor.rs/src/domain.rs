use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AuditError;
use crate::canonical::{canonical_json_bytes, digest_serializable, sha256_hex};

const INPUT_SCHEMA_VERSION: &str = "canonical.evidence-observation-input/v1";
const MAX_NORMALIZED_EVIDENCE_BYTES: usize = 64 * 1024;

/// A validated tenant boundary. Tenant identifiers never default implicitly.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TenantId(String);

impl TenantId {
    /// Parses a tenant identifier.
    ///
    /// # Errors
    ///
    /// Returns [`AuditError::InvalidField`] for an empty, oversized, or unsafe identifier.
    pub fn parse(value: impl Into<String>) -> Result<Self, AuditError> {
        validate_identifier("tenant_id", value.into()).map(Self)
    }

    /// Returns the canonical string value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for TenantId {
    type Error = AuditError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<TenantId> for String {
    fn from(value: TenantId) -> Self {
        value.0
    }
}

/// A hierarchical, tenant-local scope such as `organization/acme/repository/api`.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ScopeId(String);

impl ScopeId {
    /// Parses a slash-delimited scope with no empty or traversal segments.
    ///
    /// # Errors
    ///
    /// Returns [`AuditError::InvalidField`] for absolute, oversized, empty, traversal, or unsafe
    /// scope segments.
    pub fn parse(value: impl Into<String>) -> Result<Self, AuditError> {
        let value = value.into();
        if value.len() > 240 || value.starts_with('/') || value.ends_with('/') {
            return Err(AuditError::InvalidField {
                field: "scope_id",
                reason: "must be a relative path no longer than 240 bytes".to_owned(),
            });
        }
        let segments: Vec<_> = value.split('/').collect();
        if segments.is_empty()
            || segments.iter().any(|segment| {
                segment.is_empty()
                    || *segment == "."
                    || *segment == ".."
                    || !segment.bytes().all(|byte| {
                        byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')
                    })
            })
        {
            return Err(AuditError::InvalidField {
                field: "scope_id",
                reason: "contains an invalid path segment".to_owned(),
            });
        }
        Ok(Self(value))
    }

    /// Returns the canonical string value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns true when this scope contains the candidate at a segment boundary.
    #[must_use]
    pub fn contains(&self, candidate: &Self) -> bool {
        candidate == self
            || candidate
                .0
                .strip_prefix(&self.0)
                .is_some_and(|suffix| suffix.starts_with('/'))
    }
}

impl TryFrom<String> for ScopeId {
    type Error = AuditError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ScopeId> for String {
    fn from(value: ScopeId) -> Self {
        value.0
    }
}

/// The entity classes that can participate in an assessment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectKind {
    /// The assessed organization.
    Organization,
    /// A business unit within an organization.
    BusinessUnit,
    /// A technical or operational system.
    System,
    /// A classified data set.
    DataClass,
    /// A workforce or machine identity.
    Identity,
    /// An external vendor.
    Vendor,
    /// An internal policy.
    Policy,
    /// A risk record.
    Risk,
    /// A control statement independent of any framework.
    Control,
    /// An approved exception.
    Exception,
    /// An audit finding.
    Finding,
    /// A remediation owner.
    RemediationOwner,
}

/// A framework-neutral assessment subject.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssessmentSubject {
    /// Deterministic subject identifier.
    pub subject_id: String,
    /// Owning tenant.
    pub tenant_id: TenantId,
    /// Tenant-local authorization scope.
    pub scope_id: ScopeId,
    /// Subject class.
    pub kind: SubjectKind,
    /// Human-readable label that does not affect authorization.
    pub display_name: String,
}

/// A closed assessment time range expressed as Unix seconds.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssessmentPeriod {
    /// Inclusive range start.
    pub starts_at: i64,
    /// Inclusive range end.
    pub ends_at: i64,
}

impl AssessmentPeriod {
    /// Validates and creates an assessment period.
    ///
    /// # Errors
    ///
    /// Returns [`AuditError::InvalidField`] for negative or reversed time ranges.
    pub fn new(starts_at: i64, ends_at: i64) -> Result<Self, AuditError> {
        if starts_at < 0 || ends_at < starts_at {
            return Err(AuditError::InvalidField {
                field: "assessment_period",
                reason: "requires 0 <= starts_at <= ends_at".to_owned(),
            });
        }
        Ok(Self { starts_at, ends_at })
    }
}

/// Least-privilege operations granted to an audit actor.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    /// Import an observation into the domain.
    ImportEvidence,
    /// Assemble observations into an immutable package.
    AssembleAuditPackage,
    /// Export a verified package through a versioned boundary.
    ExportAuditPackage,
}

/// An explicit tenant- and scope-bound capability grant.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapabilityGrant {
    /// Grant tenant.
    pub tenant_id: TenantId,
    /// Maximum accessible scope.
    pub scope_id: ScopeId,
    /// Allowed operations.
    pub capabilities: BTreeSet<Capability>,
}

impl CapabilityGrant {
    /// Checks tenant, hierarchical scope, and operation in one decision.
    #[must_use]
    pub fn allows(&self, tenant: &TenantId, scope: &ScopeId, capability: &Capability) -> bool {
        &self.tenant_id == tenant
            && self.scope_id.contains(scope)
            && self.capabilities.contains(capability)
    }
}

/// Provenance source, preserving manual versus automated collection.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EvidenceSource {
    /// A human-submitted observation.
    Manual {
        /// Stable submitter identity, not a display name.
        submitted_by: String,
    },
    /// An observation emitted by a connector.
    Automated {
        /// Connector identifier.
        connector: String,
        /// Connector build or adapter version.
        adapter_version: String,
    },
}

/// Versioned connector-to-auditor exchange value owned by this crate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvidenceObservationInputV1 {
    /// Must equal `canonical.evidence-observation-input/v1`.
    pub schema_version: String,
    /// Explicit tenant boundary.
    pub tenant_id: TenantId,
    /// Explicit collection scope.
    pub scope_id: ScopeId,
    /// Framework-neutral evidence type.
    pub evidence_type: String,
    /// Manual or automated source with source-specific identity.
    pub source: EvidenceSource,
    /// Stable collector identity.
    pub collector_identity: String,
    /// Collection time as Unix seconds.
    pub collected_at: i64,
    /// Freshness boundary as Unix seconds.
    pub valid_until: i64,
    /// Bounded normalized evidence content.
    pub normalized: Value,
    /// SHA-256 of canonical normalized JSON, calculated at the source.
    pub content_sha256: String,
}

/// A validated, tamper-evident observation in the internal domain.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvidenceObservation {
    /// Deterministic observation ID.
    pub observation_id: String,
    /// Explicit tenant boundary.
    pub tenant_id: TenantId,
    /// Explicit collection scope.
    pub scope_id: ScopeId,
    /// Framework-neutral evidence type.
    pub evidence_type: String,
    /// Manual or automated provenance.
    pub source: EvidenceSource,
    /// Stable collector identity.
    pub collector_identity: String,
    /// Collection time as Unix seconds.
    pub collected_at: i64,
    /// Freshness boundary as Unix seconds.
    pub valid_until: i64,
    /// Bounded normalized evidence content.
    pub normalized: Value,
    /// Verified canonical content digest.
    pub content_sha256: String,
}

impl EvidenceObservation {
    /// Imports a connector or manual exchange value under an explicit capability grant.
    ///
    /// # Errors
    ///
    /// Returns an [`AuditError`] when schema, capability, tenant, scope, freshness, size,
    /// provenance, or digest validation fails.
    pub fn import(
        input: EvidenceObservationInputV1,
        grant: &CapabilityGrant,
    ) -> Result<Self, AuditError> {
        if input.schema_version != INPUT_SCHEMA_VERSION {
            return Err(AuditError::UnsupportedSchema(input.schema_version));
        }
        if !grant.allows(
            &input.tenant_id,
            &input.scope_id,
            &Capability::ImportEvidence,
        ) {
            return Err(AuditError::CapabilityDenied);
        }
        validate_identifier("evidence_type", input.evidence_type.clone())?;
        validate_identifier("collector_identity", input.collector_identity.clone())?;
        validate_source(&input.source)?;
        if input.collected_at < 0 || input.valid_until < input.collected_at {
            return Err(AuditError::InvalidField {
                field: "freshness",
                reason: "requires 0 <= collected_at <= valid_until".to_owned(),
            });
        }
        if !input.normalized.is_object() {
            return Err(AuditError::InvalidField {
                field: "normalized",
                reason: "must be a JSON object".to_owned(),
            });
        }
        let normalized_bytes = canonical_json_bytes(&input.normalized)?;
        if normalized_bytes.len() > MAX_NORMALIZED_EVIDENCE_BYTES {
            return Err(AuditError::EvidenceTooLarge {
                actual: normalized_bytes.len(),
                maximum: MAX_NORMALIZED_EVIDENCE_BYTES,
            });
        }
        let actual_digest = sha256_hex(&normalized_bytes);
        if input.content_sha256 != actual_digest {
            return Err(AuditError::DigestMismatch {
                expected: input.content_sha256,
                actual: actual_digest,
            });
        }

        let observation_id = digest_serializable(&(
            "canonical.observation/v1",
            &input.tenant_id,
            &input.scope_id,
            &input.evidence_type,
            &input.source,
            &input.collector_identity,
            input.collected_at,
            input.valid_until,
            &input.content_sha256,
        ))?;

        Ok(Self {
            observation_id,
            tenant_id: input.tenant_id,
            scope_id: input.scope_id,
            evidence_type: input.evidence_type,
            source: input.source,
            collector_identity: input.collector_identity,
            collected_at: input.collected_at,
            valid_until: input.valid_until,
            normalized: input.normalized,
            content_sha256: input.content_sha256,
        })
    }

    /// Recomputes the normalized content digest.
    ///
    /// # Errors
    ///
    /// Returns [`AuditError::DigestMismatch`] when content has changed, or a JSON error when the
    /// value cannot be canonically serialized.
    pub fn verify_content_digest(&self) -> Result<(), AuditError> {
        let actual = digest_serializable(&self.normalized)?;
        if actual != self.content_sha256 {
            return Err(AuditError::DigestMismatch {
                expected: self.content_sha256.clone(),
                actual,
            });
        }
        Ok(())
    }
}

fn validate_identifier(field: &'static str, value: String) -> Result<String, AuditError> {
    if value.is_empty()
        || value.len() > 160
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'@')
        })
    {
        return Err(AuditError::InvalidField {
            field,
            reason: "must be 1..=160 safe identifier bytes".to_owned(),
        });
    }
    Ok(value)
}

fn validate_source(source: &EvidenceSource) -> Result<(), AuditError> {
    match source {
        EvidenceSource::Manual { submitted_by } => {
            validate_identifier("submitted_by", submitted_by.clone())?;
        }
        EvidenceSource::Automated {
            connector,
            adapter_version,
        } => {
            validate_identifier("connector", connector.clone())?;
            validate_identifier("adapter_version", adapter_version.clone())?;
        }
    }
    Ok(())
}
