//! Property tests for scope boundaries and evidence tamper detection.

use std::collections::BTreeSet;

use canonical_company_auditor::{
    AuditError, Capability, CapabilityGrant, EvidenceObservation, EvidenceObservationInputV1,
    ScopeId, TenantId,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn scope_containment_respects_segment_boundaries(segment in "[a-z][a-z0-9_-]{0,20}") {
        let root_result = ScopeId::parse("organization/acme");
        let child_result = ScopeId::parse(format!("organization/acme/{segment}"));
        let lookalike_result = ScopeId::parse(format!("organization/acme-{segment}"));
        prop_assert!(root_result.is_ok());
        prop_assert!(child_result.is_ok());
        prop_assert!(lookalike_result.is_ok());
        if let (Ok(root), Ok(child), Ok(lookalike)) = (root_result, child_result, lookalike_result) {
            prop_assert!(root.contains(&child));
            prop_assert!(!root.contains(&lookalike));
        }
    }

    #[test]
    fn any_normalized_content_change_invalidates_source_digest(status in "[a-z]{1,24}") {
        prop_assume!(status != "approved");
        let parsed = serde_json::from_str::<EvidenceObservationInputV1>(include_str!(
            "fixtures/manual-observation.json"
        ));
        prop_assert!(parsed.is_ok());
        if let Ok(mut input) = parsed {
            input.normalized["status"] = serde_json::json!(status);
            let tenant = TenantId::parse("tenant-a");
            let scope = ScopeId::parse("organization/acme");
            prop_assert!(tenant.is_ok());
            prop_assert!(scope.is_ok());
            if let (Ok(tenant_id), Ok(scope_id)) = (tenant, scope) {
                let grant = CapabilityGrant {
                    tenant_id,
                    scope_id,
                    capabilities: BTreeSet::from([Capability::ImportEvidence]),
                };
                let digest_mismatch = matches!(
                    EvidenceObservation::import(input, &grant),
                    Err(AuditError::DigestMismatch { .. })
                );
                prop_assert!(digest_mismatch);
            }
        }
    }
}
