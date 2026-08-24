//! Hermetic GitHub connector, authority, policy, and concurrency tests.

use std::error::Error;
use std::sync::{Arc, mpsc};

use canonical_evidence_connectors::{
    ConnectorError, CredentialReference, FixtureGithubTransport, GithubCollector, GithubGetRequest,
    GithubMetadataTransport, GithubPage, LiveGithubAuthority, github_descriptor,
};

const COLLECTED_AT: i64 = 1_767_225_600;

fn fixture_transport() -> Result<FixtureGithubTransport, ConnectorError> {
    FixtureGithubTransport::from_json(include_str!("fixtures/github-pages.json"))
}

#[test]
fn descriptor_declares_read_only_bounded_behavior() {
    let descriptor = github_descriptor();
    assert!(descriptor.read_only);
    assert_eq!(descriptor.pagination.per_page, 100);
    assert_eq!(descriptor.pagination.max_pages, 10);
    assert_eq!(descriptor.pagination.max_records, 1_000);
    assert!(
        descriptor
            .permission_scopes
            .iter()
            .all(|scope| scope.ends_with(":read"))
    );
    assert_eq!(descriptor.evidence_types, ["github.repository_posture"]);
}

#[test]
fn fixture_collection_is_paginated_redacted_hashed_and_sorted() -> Result<(), Box<dyn Error>> {
    let transport = fixture_transport()?;
    let collector = GithubCollector::new(transport.clone());
    let batch = collector.collect("tenant-a", "acme", COLLECTED_AT)?;
    assert_eq!(batch.observations.len(), 3);
    assert_eq!(
        batch
            .observations
            .iter()
            .map(|observation| observation.scope_id.as_str())
            .collect::<Vec<_>>(),
        [
            "organization/acme/repository/api",
            "organization/acme/repository/docs",
            "organization/acme/repository/web"
        ]
    );
    for observation in &batch.observations {
        assert_eq!(observation.content_sha256.len(), 64);
        assert_eq!(observation.valid_until, COLLECTED_AT + 86_400);
        assert!(observation.normalized.get("upstream_noise").is_none());
        assert_eq!(
            observation.normalized.as_object().map(serde_json::Map::len),
            Some(7)
        );
    }
    assert_eq!(
        transport.requests()?,
        [
            GithubGetRequest {
                organization: "acme".to_owned(),
                page: 1,
                per_page: 100,
            },
            GithubGetRequest {
                organization: "acme".to_owned(),
                page: 2,
                per_page: 100,
            }
        ]
    );
    Ok(())
}

#[test]
fn raw_credentials_are_rejected_and_live_transport_fails_closed() -> Result<(), Box<dyn Error>> {
    assert!(matches!(
        CredentialReference::parse("not-a-reference"),
        Err(ConnectorError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        CredentialReference::parse("env:lowercase"),
        Err(ConnectorError::InvalidConfiguration(_))
    ));
    let authority = LiveGithubAuthority::new(
        "tenant-a",
        "acme",
        CredentialReference::parse("env:CANONICAL_GITHUB_READ_TOKEN")?,
    )?;
    assert!(matches!(
        authority.require_production_transport(),
        Err(ConnectorError::ProductionTransportUnavailable)
    ));
    Ok(())
}

#[derive(Debug)]
struct RateLimitedTransport;

impl GithubMetadataTransport for RateLimitedTransport {
    fn get_page(&self, _request: &GithubGetRequest) -> Result<GithubPage, ConnectorError> {
        Err(ConnectorError::RateLimited {
            retry_after_seconds: 30,
        })
    }
}

#[test]
fn rate_limits_propagate_without_partial_evidence() {
    let collector = GithubCollector::new(RateLimitedTransport);
    assert!(matches!(
        collector.collect("tenant-a", "acme", COLLECTED_AT),
        Err(ConnectorError::RateLimited {
            retry_after_seconds: 30
        })
    ));
}

#[test]
fn concurrent_collection_is_deterministic() -> Result<(), Box<dyn Error>> {
    let collector = Arc::new(GithubCollector::new(fixture_transport()?));
    let (sender, receiver) = mpsc::channel();
    std::thread::scope(|threads| {
        for _ in 0..4 {
            let collector = Arc::clone(&collector);
            let sender = sender.clone();
            threads.spawn(move || {
                let result = collector.collect("tenant-a", "acme", COLLECTED_AT);
                let _ = sender.send(result);
            });
        }
    });
    drop(sender);
    let batches = (0..4)
        .map(|_| receiver.recv())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    assert!(batches.windows(2).all(|pair| pair[0] == pair[1]));
    Ok(())
}
