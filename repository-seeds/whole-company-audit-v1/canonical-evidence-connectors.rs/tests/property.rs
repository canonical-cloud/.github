//! Property tests for allow-list redaction of arbitrary upstream fields.

use canonical_evidence_connectors::{FixtureGithubTransport, GithubCollector};
use proptest::prelude::*;

proptest! {
    #[test]
    fn arbitrary_unknown_upstream_fields_never_escape(
        unknown_key in "extra_[a-z]{1,20}",
        unknown_value in "[ -~]{0,80}"
    ) {
        let mut record = serde_json::json!({
            "name": "api",
            "default_branch": "main",
            "visibility": "private",
            "branch_protection_required": true,
            "actions_enabled": true,
            "team_count": 2,
            "audit_log_reference": null
        });
        record[&unknown_key] = serde_json::Value::String(unknown_value);
        let fixture = serde_json::json!({
            "pages": [{"page": 1, "records": [record], "next_page": null}]
        });
        let transport = FixtureGithubTransport::from_json(&fixture.to_string());
        prop_assert!(transport.is_ok());
        if let Ok(transport) = transport {
            let result = GithubCollector::new(transport).collect("tenant-a", "acme", 1_767_225_600);
            prop_assert!(result.is_ok());
            if let Ok(batch) = result {
                prop_assert_eq!(batch.observations.len(), 1);
                prop_assert!(batch.observations[0].normalized.get(&unknown_key).is_none());
            }
        }
    }
}
