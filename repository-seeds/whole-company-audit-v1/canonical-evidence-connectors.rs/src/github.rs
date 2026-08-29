use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::canonical::digest_serializable;
use crate::{
    ConnectorDescriptor, ConnectorError, ConnectorFamily, EvidenceObservationInputV1,
    EvidenceSource, ObservationBatchV1, PaginationPolicy, RateLimitPolicy,
};

const OBSERVATION_SCHEMA_VERSION: &str = "canonical.evidence-observation-input/v1";
const BATCH_SCHEMA_VERSION: &str = "canonical.evidence-observation-batch/v1";

/// Returns the reviewable capability and handling declaration for the GitHub adapter.
#[must_use]
pub fn github_descriptor() -> ConnectorDescriptor {
    ConnectorDescriptor {
        connector_id: "github-read-only".to_owned(),
        adapter_version: "v1".to_owned(),
        family: ConnectorFamily::Github,
        read_only: true,
        permission_scopes: vec![
            "metadata:read".to_owned(),
            "administration:read".to_owned(),
            "organization_members:read".to_owned(),
            "audit_log:read".to_owned(),
        ],
        evidence_types: vec!["github.repository_posture".to_owned()],
        freshness_seconds: 86_400,
        pagination: PaginationPolicy {
            per_page: 100,
            max_pages: 10,
            max_records: 1_000,
        },
        rate_limit: RateLimitPolicy::FailClosedWithRetryAfter,
        redaction_allowlist: vec![
            "name".to_owned(),
            "default_branch".to_owned(),
            "visibility".to_owned(),
            "branch_protection_required".to_owned(),
            "actions_enabled".to_owned(),
            "team_count".to_owned(),
            "audit_log_reference".to_owned(),
        ],
    }
}

/// A GET-only page request. No method field exists, so callers cannot request mutation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GithubGetRequest {
    /// GitHub organization slug.
    pub organization: String,
    /// One-based page number.
    pub page: u16,
    /// Bounded requested record count.
    pub per_page: u16,
}

/// A bounded upstream response page.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubPage {
    /// Untrusted upstream repository records.
    pub records: Vec<Value>,
    /// Next page number, if present.
    pub next_page: Option<u16>,
}

/// Read-only transport extension point for GitHub metadata collection.
pub trait GithubMetadataTransport: Send + Sync {
    /// Performs one GET-only metadata page request.
    ///
    /// # Errors
    ///
    /// Returns a typed transport or rate-limit error. Implementations must not mutate GitHub state.
    fn get_page(&self, request: &GithubGetRequest) -> Result<GithubPage, ConnectorError>;
}

#[derive(Deserialize)]
struct FixtureDocument {
    pages: Vec<FixturePage>,
}

#[derive(Deserialize)]
struct FixturePage {
    page: u16,
    records: Vec<Value>,
    next_page: Option<u16>,
}

/// Hermetic fake transport used by fixtures and tests.
#[derive(Clone, Debug)]
pub struct FixtureGithubTransport {
    pages: Arc<BTreeMap<u16, GithubPage>>,
    requests: Arc<Mutex<Vec<GithubGetRequest>>>,
}

impl FixtureGithubTransport {
    /// Parses a fixture document without network or credential access.
    ///
    /// # Errors
    ///
    /// Returns a JSON or configuration error for malformed, duplicate, or zero-numbered pages.
    pub fn from_json(input: &str) -> Result<Self, ConnectorError> {
        let document: FixtureDocument = serde_json::from_str(input)?;
        let mut pages = BTreeMap::new();
        for page in document.pages {
            if page.page == 0
                || pages
                    .insert(
                        page.page,
                        GithubPage {
                            records: page.records,
                            next_page: page.next_page,
                        },
                    )
                    .is_some()
            {
                return Err(ConnectorError::InvalidConfiguration(
                    "fixture pages must be unique and one-based",
                ));
            }
        }
        if pages.is_empty() {
            return Err(ConnectorError::InvalidConfiguration(
                "fixture must contain at least one page",
            ));
        }
        Ok(Self {
            pages: Arc::new(pages),
            requests: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Returns an isolated copy of recorded GET requests.
    ///
    /// # Errors
    ///
    /// Returns a lock error if a test thread poisoned the request ledger.
    pub fn requests(&self) -> Result<Vec<GithubGetRequest>, ConnectorError> {
        self.requests
            .lock()
            .map(|requests| requests.clone())
            .map_err(|_| ConnectorError::LockPoisoned)
    }
}

impl GithubMetadataTransport for FixtureGithubTransport {
    fn get_page(&self, request: &GithubGetRequest) -> Result<GithubPage, ConnectorError> {
        self.requests
            .lock()
            .map_err(|_| ConnectorError::LockPoisoned)?
            .push(request.clone());
        self.pages
            .get(&request.page)
            .cloned()
            .ok_or(ConnectorError::UnexpectedPage(request.page))
    }
}

/// Deterministic, bounded GitHub evidence collector over a read-only transport.
#[derive(Debug)]
pub struct GithubCollector<T> {
    transport: T,
    descriptor: ConnectorDescriptor,
}

impl<T: GithubMetadataTransport> GithubCollector<T> {
    /// Creates a collector using the immutable GitHub descriptor.
    #[must_use]
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            descriptor: github_descriptor(),
        }
    }

    /// Returns the connector capability and handling declaration.
    #[must_use]
    pub fn descriptor(&self) -> &ConnectorDescriptor {
        &self.descriptor
    }

    /// Collects and normalizes repository posture for one explicit tenant and organization.
    ///
    /// # Errors
    ///
    /// Fails closed for invalid configuration, transport failure, rate limiting, pagination drift,
    /// record-limit overflow, malformed records, time overflow, or serialization failure.
    pub fn collect(
        &self,
        tenant_id: &str,
        organization: &str,
        collected_at: i64,
    ) -> Result<ObservationBatchV1, ConnectorError> {
        validate_safe_name("tenant", tenant_id)?;
        validate_safe_name("organization", organization)?;
        if collected_at < 0 {
            return Err(ConnectorError::InvalidConfiguration(
                "collected_at must be non-negative",
            ));
        }
        let valid_until = collected_at
            .checked_add(self.descriptor.freshness_seconds)
            .ok_or(ConnectorError::TimeOverflow)?;
        let mut current_page = 1_u16;
        let mut records = Vec::new();

        for page_index in 0..self.descriptor.pagination.max_pages {
            let page = self.transport.get_page(&GithubGetRequest {
                organization: organization.to_owned(),
                page: current_page,
                per_page: self.descriptor.pagination.per_page,
            })?;
            records.extend(page.records);
            if records.len() > self.descriptor.pagination.max_records {
                return Err(ConnectorError::RecordLimitExceeded);
            }
            match page.next_page {
                None => break,
                Some(next_page)
                    if next_page > current_page
                        && page_index + 1 < self.descriptor.pagination.max_pages =>
                {
                    current_page = next_page;
                }
                Some(_) => return Err(ConnectorError::InvalidPagination),
            }
        }

        let mut observations = records
            .iter()
            .map(|record| {
                normalize_repository(
                    record,
                    tenant_id,
                    organization,
                    collected_at,
                    valid_until,
                    &self.descriptor,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        observations.sort_by(|left, right| left.scope_id.cmp(&right.scope_id));
        Ok(ObservationBatchV1 {
            schema_version: BATCH_SCHEMA_VERSION.to_owned(),
            observations,
        })
    }
}

fn normalize_repository(
    record: &Value,
    tenant_id: &str,
    organization: &str,
    collected_at: i64,
    valid_until: i64,
    descriptor: &ConnectorDescriptor,
) -> Result<EvidenceObservationInputV1, ConnectorError> {
    let object = record
        .as_object()
        .ok_or(ConnectorError::MalformedUpstreamRecord(
            "repository record must be an object",
        ))?;
    let name = required_string(object, "name")?;
    validate_safe_name("repository name", name)?;
    let default_branch = required_string(object, "default_branch")?;
    validate_safe_name("default branch", default_branch)?;
    let visibility = required_string(object, "visibility")?;
    if !matches!(visibility, "public" | "private" | "internal") {
        return Err(ConnectorError::MalformedUpstreamRecord(
            "visibility must be public, private, or internal",
        ));
    }
    let branch_protection_required = required_bool(object, "branch_protection_required")?;
    let actions_enabled = required_bool(object, "actions_enabled")?;
    let team_count = required_u64(object, "team_count")?;
    let audit_log_reference = optional_bounded_string(object, "audit_log_reference", 200)?;

    let mut normalized = Map::new();
    normalized.insert("actions_enabled".to_owned(), actions_enabled.into());
    normalized.insert(
        "audit_log_reference".to_owned(),
        audit_log_reference.map_or(Value::Null, |value| Value::String(value.to_owned())),
    );
    normalized.insert(
        "branch_protection_required".to_owned(),
        branch_protection_required.into(),
    );
    normalized.insert("default_branch".to_owned(), default_branch.into());
    normalized.insert("name".to_owned(), name.into());
    normalized.insert("team_count".to_owned(), team_count.into());
    normalized.insert("visibility".to_owned(), visibility.into());
    let normalized = Value::Object(normalized);
    let content_sha256 = digest_serializable(&normalized)?;

    Ok(EvidenceObservationInputV1 {
        schema_version: OBSERVATION_SCHEMA_VERSION.to_owned(),
        tenant_id: tenant_id.to_owned(),
        scope_id: format!("organization/{organization}/repository/{name}"),
        evidence_type: descriptor.evidence_types[0].clone(),
        source: EvidenceSource::Automated {
            connector: descriptor.connector_id.clone(),
            adapter_version: descriptor.adapter_version.clone(),
        },
        collector_identity: format!("github:{organization}"),
        collected_at,
        valid_until,
        normalized,
        content_sha256,
    })
}

fn required_string<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
) -> Result<&'a str, ConnectorError> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or(ConnectorError::MalformedUpstreamRecord(field))
}

fn required_bool(object: &Map<String, Value>, field: &'static str) -> Result<bool, ConnectorError> {
    object
        .get(field)
        .and_then(Value::as_bool)
        .ok_or(ConnectorError::MalformedUpstreamRecord(field))
}

fn required_u64(object: &Map<String, Value>, field: &'static str) -> Result<u64, ConnectorError> {
    object
        .get(field)
        .and_then(Value::as_u64)
        .ok_or(ConnectorError::MalformedUpstreamRecord(field))
}

fn optional_bounded_string<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
    maximum: usize,
) -> Result<Option<&'a str>, ConnectorError> {
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) if value.len() <= maximum => Ok(Some(value)),
        Some(_) => Err(ConnectorError::MalformedUpstreamRecord(field)),
    }
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
