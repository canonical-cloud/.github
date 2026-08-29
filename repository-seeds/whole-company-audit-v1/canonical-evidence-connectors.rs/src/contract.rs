use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Enterprise connector families supported by the extension boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorFamily {
    /// Source-control and software-delivery posture.
    Github,
    /// Amazon Web Services posture.
    Aws,
    /// Google Cloud Platform posture.
    Gcp,
    /// Microsoft Azure posture.
    Azure,
    /// Identity-provider posture.
    IdentityProvider,
    /// Google Workspace posture.
    GoogleWorkspace,
    /// Microsoft 365 posture.
    Microsoft365,
    /// Kubernetes cluster and workload posture.
    Kubernetes,
    /// `PostgreSQL` and managed database posture.
    Database,
    /// Endpoint and mobile-device-management posture.
    Endpoint,
    /// Vulnerability-management posture.
    Vulnerability,
    /// Ticketing-system posture.
    Ticketing,
    /// Vendor-management posture.
    VendorManagement,
    /// Policy-document posture.
    PolicyDocument,
}

/// Declared pagination limits for a connector.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PaginationPolicy {
    /// Maximum records requested per page.
    pub per_page: u16,
    /// Maximum pages collected in one run.
    pub max_pages: u16,
    /// Maximum normalized records emitted in one run.
    pub max_records: usize,
}

/// Declared rate-limit behavior.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitPolicy {
    /// Stop and return a typed retry-after error without sleeping or silently truncating.
    FailClosedWithRetryAfter,
}

/// Reviewable connector capability and data-handling declaration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConnectorDescriptor {
    /// Stable connector identifier.
    pub connector_id: String,
    /// Adapter version emitted in provenance.
    pub adapter_version: String,
    /// Connector family.
    pub family: ConnectorFamily,
    /// True for every connector in this repository.
    pub read_only: bool,
    /// Minimum upstream permission names.
    pub permission_scopes: Vec<String>,
    /// Framework-neutral evidence types emitted by the adapter.
    pub evidence_types: Vec<String>,
    /// Maximum evidence age in seconds.
    pub freshness_seconds: i64,
    /// Bounded pagination policy.
    pub pagination: PaginationPolicy,
    /// Explicit rate-limit behavior.
    pub rate_limit: RateLimitPolicy,
    /// Exact normalized field allow-list; all other fields are dropped.
    pub redaction_allowlist: Vec<String>,
}

/// Automated source shape from the auditor-owned observation input contract.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EvidenceSource {
    /// A connector-generated observation.
    Automated {
        /// Stable connector identifier.
        connector: String,
        /// Adapter version.
        adapter_version: String,
    },
}

/// Serialized observation input whose schema is owned by `canonical-company-auditor.rs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvidenceObservationInputV1 {
    /// Auditor-owned schema version.
    pub schema_version: String,
    /// Explicit customer tenant.
    pub tenant_id: String,
    /// Explicit tenant-local scope.
    pub scope_id: String,
    /// Framework-neutral evidence type.
    pub evidence_type: String,
    /// Automated connector source.
    pub source: EvidenceSource,
    /// Stable collector identity.
    pub collector_identity: String,
    /// Collection time as Unix seconds.
    pub collected_at: i64,
    /// Freshness boundary as Unix seconds.
    pub valid_until: i64,
    /// Bounded, allow-listed normalized evidence.
    pub normalized: Value,
    /// Canonical normalized JSON SHA-256.
    pub content_sha256: String,
}

/// Batch envelope consumed by the auditor CLI and import boundary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationBatchV1 {
    /// Auditor-owned batch schema version.
    pub schema_version: String,
    /// Deterministically ordered observation inputs.
    pub observations: Vec<EvidenceObservationInputV1>,
}
