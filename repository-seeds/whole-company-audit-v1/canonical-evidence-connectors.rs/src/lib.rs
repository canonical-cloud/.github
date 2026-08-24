//! Least-privilege, read-only enterprise evidence connector primitives.
//!
//! The seed implements a hermetic GitHub repository-posture vertical slice and exposes explicit
//! extension metadata for other enterprise connector families. It contains no production HTTP
//! client, mutation method, resolved credential, framework mapping, or remediation behavior.

mod authority;
mod canonical;
mod contract;
mod github;

use thiserror::Error;

pub use authority::{CredentialReference, LiveGithubAuthority};
pub use contract::{
    ConnectorDescriptor, ConnectorFamily, EvidenceObservationInputV1, EvidenceSource,
    ObservationBatchV1, PaginationPolicy, RateLimitPolicy,
};
pub use github::{
    FixtureGithubTransport, GithubCollector, GithubGetRequest, GithubMetadataTransport, GithubPage,
    github_descriptor,
};

/// Fail-closed connector configuration, transport, normalization, and policy errors.
#[derive(Debug, Error)]
pub enum ConnectorError {
    /// Required environment configuration was absent.
    #[error("required environment configuration is missing: {0}")]
    MissingEnvironment(&'static str),
    /// Configuration was invalid; the rejected value is deliberately omitted.
    #[error("invalid connector configuration: {0}")]
    InvalidConfiguration(&'static str),
    /// The seed deliberately has no live GitHub transport.
    #[error("production GitHub transport is unavailable in the repository seed")]
    ProductionTransportUnavailable,
    /// An upstream service requested a bounded retry.
    #[error("upstream rate limited collection; retry after {retry_after_seconds} seconds")]
    RateLimited {
        /// Upstream-provided bounded retry interval.
        retry_after_seconds: u64,
    },
    /// A transport failed without exposing secret material.
    #[error("read-only transport failed: {0}")]
    Transport(&'static str),
    /// A fixture or upstream transport omitted a requested page.
    #[error("transport did not return requested page {0}")]
    UnexpectedPage(u16),
    /// Pagination was cyclic, backward, or exceeded the declared maximum.
    #[error("invalid or over-limit pagination")]
    InvalidPagination,
    /// The connector exceeded its declared normalized record bound.
    #[error("connector record limit exceeded")]
    RecordLimitExceeded,
    /// An upstream value failed the normalized allow-list contract.
    #[error("malformed upstream record field: {0}")]
    MalformedUpstreamRecord(&'static str),
    /// Freshness arithmetic overflowed.
    #[error("freshness time overflow")]
    TimeOverflow,
    /// A hermetic request ledger was poisoned by a failed thread.
    #[error("fixture request ledger lock poisoned")]
    LockPoisoned,
    /// JSON parsing or serialization failed.
    #[error("JSON boundary failure: {0}")]
    Json(#[from] serde_json::Error),
    /// Fixture file input failed.
    #[error("fixture input failure: {0}")]
    Io(#[from] std::io::Error),
}
