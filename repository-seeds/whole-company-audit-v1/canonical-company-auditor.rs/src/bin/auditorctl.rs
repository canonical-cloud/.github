//! Narrow JSON CLI harness for seed conformance and package verification.

use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::io::{self, Read, Write};

use canonical_company_auditor::{
    AssessmentPeriod, AuditPackage, Capability, CapabilityGrant, EvidenceObservation,
    ObservationBatchV1, OscalBackMatterV1, ScopeId, TenantId,
};
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args().skip(1);
    let command = arguments.next().ok_or(
        "usage: auditorctl package <tenant> <scope> <starts-at> <ends-at> | verify | oscal",
    )?;
    match command.as_str() {
        "package" => {
            let package_arguments = arguments.collect::<Vec<_>>();
            package(&package_arguments)
        }
        "verify" => verify(),
        "oscal" => oscal(),
        _ => Err(format!("unknown command: {command}").into()),
    }
}

fn package(arguments: &[String]) -> Result<(), Box<dyn Error>> {
    if arguments.len() != 4 {
        return Err("package requires tenant, scope, starts-at, and ends-at".into());
    }
    let tenant = TenantId::parse(&arguments[0])?;
    let scope = ScopeId::parse(&arguments[1])?;
    let starts_at: i64 = arguments[2].parse()?;
    let ends_at: i64 = arguments[3].parse()?;
    let period = AssessmentPeriod::new(starts_at, ends_at)?;
    let batch: ObservationBatchV1 = read_stdin_json()?;
    batch.validate_version()?;

    let grant = CapabilityGrant {
        tenant_id: tenant.clone(),
        scope_id: scope.clone(),
        capabilities: BTreeSet::from([
            Capability::ImportEvidence,
            Capability::AssembleAuditPackage,
        ]),
    };
    let observations = batch
        .observations
        .into_iter()
        .map(|input| EvidenceObservation::import(input, &grant))
        .collect::<Result<Vec<_>, _>>()?;
    let package = AuditPackage::build(tenant, scope, period, observations, &grant)?;
    write_stdout_json(&package)
}

fn verify() -> Result<(), Box<dyn Error>> {
    let package: AuditPackage = read_stdin_json()?;
    package.verify()?;
    write_stdout_json(&json!({
        "package_id": package.manifest.package_id,
        "manifest_sha256": package.manifest.manifest_sha256,
        "observation_count": package.observations.len(),
        "verified": true
    }))
}

fn oscal() -> Result<(), Box<dyn Error>> {
    let package: AuditPackage = read_stdin_json()?;
    package.verify()?;
    write_stdout_json(&OscalBackMatterV1::from(&package))
}

fn read_stdin_json<T: serde::de::DeserializeOwned>() -> Result<T, Box<dyn Error>> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    Ok(serde_json::from_slice(&input)?)
}

fn write_stdout_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn Error>> {
    let mut output = io::BufWriter::new(io::stdout().lock());
    serde_json::to_writer_pretty(&mut output, value)?;
    output.write_all(b"\n")?;
    output.flush()?;
    Ok(())
}
