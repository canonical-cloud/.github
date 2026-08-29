//! Domain, concurrency, persistence, and boundary conformance tests.

use std::collections::BTreeSet;
use std::error::Error;
use std::sync::mpsc;

use canonical_company_auditor::{
    AssessmentPeriod, AuditError, AuditPackage, Capability, CapabilityGrant, EvidenceObservation,
    EvidenceObservationInputV1, EvidenceSource, OscalBackMatterV1, ScopeId, TenantId,
};

fn fixture() -> Result<EvidenceObservationInputV1, Box<dyn Error>> {
    Ok(serde_json::from_str(include_str!(
        "fixtures/manual-observation.json"
    ))?)
}

fn grant(
    capabilities: impl IntoIterator<Item = Capability>,
) -> Result<CapabilityGrant, AuditError> {
    Ok(CapabilityGrant {
        tenant_id: TenantId::parse("tenant-a")?,
        scope_id: ScopeId::parse("organization/acme")?,
        capabilities: capabilities.into_iter().collect::<BTreeSet<_>>(),
    })
}

#[test]
fn manual_and_automated_sources_remain_distinct() -> Result<(), Box<dyn Error>> {
    let grant = grant([Capability::ImportEvidence])?;
    let manual = EvidenceObservation::import(fixture()?, &grant)?;
    assert!(matches!(manual.source, EvidenceSource::Manual { .. }));

    let mut automated_input = fixture()?;
    automated_input.evidence_type = "policy.repository_posture".to_owned();
    automated_input.source = EvidenceSource::Automated {
        connector: "github-read-only".to_owned(),
        adapter_version: "v1".to_owned(),
    };
    automated_input.collector_identity = "connector:fixture".to_owned();
    let automated = EvidenceObservation::import(automated_input, &grant)?;
    assert!(matches!(automated.source, EvidenceSource::Automated { .. }));
    assert_ne!(manual.observation_id, automated.observation_id);
    Ok(())
}

#[test]
fn import_rejects_tampering_and_cross_tenant_access() -> Result<(), Box<dyn Error>> {
    let grant = grant([Capability::ImportEvidence])?;
    let mut tampered = fixture()?;
    tampered.normalized["status"] = serde_json::json!("revoked");
    assert!(matches!(
        EvidenceObservation::import(tampered, &grant),
        Err(AuditError::DigestMismatch { .. })
    ));

    let mut other_tenant = fixture()?;
    other_tenant.tenant_id = TenantId::parse("tenant-b")?;
    assert!(matches!(
        EvidenceObservation::import(other_tenant, &grant),
        Err(AuditError::CapabilityDenied)
    ));
    Ok(())
}

#[test]
fn package_identity_is_order_independent_under_concurrency() -> Result<(), Box<dyn Error>> {
    let grant = grant([Capability::ImportEvidence, Capability::AssembleAuditPackage])?;
    let first = EvidenceObservation::import(fixture()?, &grant)?;
    let mut second_input = fixture()?;
    second_input.collected_at += 1;
    second_input.valid_until += 1;
    let second = EvidenceObservation::import(second_input, &grant)?;
    let tenant = TenantId::parse("tenant-a")?;
    let scope = ScopeId::parse("organization/acme")?;
    let period = AssessmentPeriod::new(1_767_225_600, 1_775_001_600)?;
    let (sender, receiver) = mpsc::channel();

    std::thread::scope(|threads| {
        for observations in [vec![first.clone(), second.clone()], vec![second, first]] {
            let sender = sender.clone();
            let grant = grant.clone();
            let tenant = tenant.clone();
            let scope = scope.clone();
            let period = period.clone();
            threads.spawn(move || {
                let result = AuditPackage::build(tenant, scope, period, observations, &grant)
                    .map(|package| package.manifest.package_id);
                let _ = sender.send(result);
            });
        }
    });
    drop(sender);

    let first_id = receiver.recv()??;
    let second_id = receiver.recv()??;
    assert_eq!(first_id, second_id);
    Ok(())
}

#[test]
fn persisted_package_is_create_only_and_verified_on_load() -> Result<(), Box<dyn Error>> {
    let grant = grant([Capability::ImportEvidence, Capability::AssembleAuditPackage])?;
    let observation = EvidenceObservation::import(fixture()?, &grant)?;
    let package = AuditPackage::build(
        TenantId::parse("tenant-a")?,
        ScopeId::parse("organization/acme")?,
        AssessmentPeriod::new(1_767_225_600, 1_775_001_600)?,
        vec![observation],
        &grant,
    )?;
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("package.json");
    package.persist_new(&path)?;
    assert!(matches!(
        package.persist_new(&path),
        Err(AuditError::Io(error)) if error.kind() == std::io::ErrorKind::AlreadyExists
    ));
    assert_eq!(AuditPackage::load_verified(path)?, package);
    Ok(())
}

#[test]
fn package_verification_rejects_provenance_tampering() -> Result<(), Box<dyn Error>> {
    let grant = grant([Capability::ImportEvidence, Capability::AssembleAuditPackage])?;
    let observation = EvidenceObservation::import(fixture()?, &grant)?;
    let mut package = AuditPackage::build(
        TenantId::parse("tenant-a")?,
        ScopeId::parse("organization/acme")?,
        AssessmentPeriod::new(1_767_225_600, 1_775_001_600)?,
        vec![observation],
        &grant,
    )?;
    if let Some(observation) = package.observations.first_mut() {
        observation.collector_identity = "manual-portal:tampered".to_owned();
    }
    assert!(matches!(
        package.verify(),
        Err(AuditError::ObservationIdentityMismatch { .. })
    ));
    Ok(())
}

#[test]
fn oscal_projection_is_a_boundary_not_the_internal_model() -> Result<(), Box<dyn Error>> {
    let grant = grant([Capability::ImportEvidence, Capability::AssembleAuditPackage])?;
    let observation = EvidenceObservation::import(fixture()?, &grant)?;
    let package = AuditPackage::build(
        TenantId::parse("tenant-a")?,
        ScopeId::parse("organization/acme")?,
        AssessmentPeriod::new(1_767_225_600, 1_775_001_600)?,
        vec![observation],
        &grant,
    )?;
    let oscal = OscalBackMatterV1::from(&package);
    assert_eq!(oscal.resources.len(), 1);
    assert_eq!(oscal.resources[0].props.len(), 3);
    Ok(())
}
