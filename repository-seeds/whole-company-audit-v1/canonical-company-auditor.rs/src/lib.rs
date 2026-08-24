//! Framework-neutral whole-company audit domain and orchestration primitives.
//!
//! Framework catalogs, connector transports, and remediation mutations are deliberately outside
//! this crate. Every import and package operation requires an explicit tenant-scoped capability.

mod boundary;
mod canonical;
mod domain;
mod package;

use thiserror::Error;

pub use boundary::{ObservationBatchV1, OscalBackMatterV1, OscalPropertyV1, OscalResourceV1};
pub use domain::{
    AssessmentPeriod, AssessmentSubject, Capability, CapabilityGrant, EvidenceObservation,
    EvidenceObservationInputV1, EvidenceSource, ScopeId, SubjectKind, TenantId,
};
pub use package::{AuditPackage, AuditPackageManifest};

/// Fail-closed domain, boundary, and persistence errors.
#[derive(Debug, Error)]
pub enum AuditError {
    /// A required field did not satisfy its contract.
    #[error("invalid {field}: {reason}")]
    InvalidField {
        /// Field name.
        field: &'static str,
        /// Human-readable safe validation reason.
        reason: String,
    },
    /// A caller attempted an operation outside its explicit grant.
    #[error("capability denied for tenant or scope")]
    CapabilityDenied,
    /// An observation crossed the package tenant or scope boundary.
    #[error("observation {observation_id} is outside the package tenant or scope")]
    TenantOrScopeMismatch {
        /// Rejected deterministic observation identifier.
        observation_id: String,
    },
    /// A content or manifest digest did not match recomputed bytes.
    #[error("digest mismatch: expected {expected}, got {actual}")]
    DigestMismatch {
        /// Supplied digest.
        expected: String,
        /// Recomputed digest.
        actual: String,
    },
    /// Normalized evidence exceeded the bounded exchange limit.
    #[error("normalized evidence is {actual} bytes; maximum is {maximum}")]
    EvidenceTooLarge {
        /// Actual canonical JSON size.
        actual: usize,
        /// Maximum canonical JSON size.
        maximum: usize,
    },
    /// A package contained the same deterministic observation more than once.
    #[error("duplicate deterministic observation")]
    DuplicateObservation,
    /// A deserialized package did not reproduce its claimed immutable manifest.
    #[error("package does not match its immutable manifest")]
    PackageManifestMismatch,
    /// A boundary version is not supported.
    #[error("unsupported schema version: {0}")]
    UnsupportedSchema(String),
    /// JSON serialization or parsing failed.
    #[error("JSON boundary failure: {0}")]
    Json(#[from] serde_json::Error),
    /// Persistence failed.
    #[error("persistence failure: {0}")]
    Io(#[from] std::io::Error),
}
