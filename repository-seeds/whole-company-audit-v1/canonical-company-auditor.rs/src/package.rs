use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::canonical::digest_serializable;
use crate::{
    AssessmentPeriod, AuditError, Capability, CapabilityGrant, EvidenceObservation, ScopeId,
    TenantId,
};

const PACKAGE_SCHEMA_VERSION: &str = "canonical.audit-package/v1";

/// Immutable manifest for a framework-neutral audit evidence package.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuditPackageManifest {
    /// Package schema version.
    pub schema_version: String,
    /// Deterministic package identifier.
    pub package_id: String,
    /// Explicit tenant boundary.
    pub tenant_id: TenantId,
    /// Scope authorized for package assembly.
    pub scope_id: ScopeId,
    /// Closed assessment period.
    pub assessment_period: AssessmentPeriod,
    /// Sorted observation identifiers.
    pub observation_ids: Vec<String>,
    /// Digest over the unsigned manifest fields.
    pub manifest_sha256: String,
}

/// Verified package containing a manifest and its deterministically ordered observations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditPackage {
    /// Immutable package manifest.
    pub manifest: AuditPackageManifest,
    /// Observations ordered by deterministic identifier.
    pub observations: Vec<EvidenceObservation>,
}

#[derive(Serialize)]
struct UnsignedManifest<'a> {
    schema_version: &'a str,
    tenant_id: &'a TenantId,
    scope_id: &'a ScopeId,
    assessment_period: &'a AssessmentPeriod,
    observation_ids: &'a [String],
}

impl AuditPackage {
    /// Builds a deterministic package while enforcing tenant, scope, capability, and digest boundaries.
    ///
    /// # Errors
    ///
    /// Returns an [`AuditError`] for denied capability, cross-boundary evidence, duplicate
    /// observations, tampered content, or canonical serialization failure.
    pub fn build(
        tenant_id: TenantId,
        scope_id: ScopeId,
        assessment_period: AssessmentPeriod,
        mut observations: Vec<EvidenceObservation>,
        grant: &CapabilityGrant,
    ) -> Result<Self, AuditError> {
        if !grant.allows(&tenant_id, &scope_id, &Capability::AssembleAuditPackage) {
            return Err(AuditError::CapabilityDenied);
        }

        for observation in &observations {
            if observation.tenant_id != tenant_id || !scope_id.contains(&observation.scope_id) {
                return Err(AuditError::TenantOrScopeMismatch {
                    observation_id: observation.observation_id.clone(),
                });
            }
            observation.verify_identity()?;
        }

        observations.sort_by(|left, right| left.observation_id.cmp(&right.observation_id));
        let observation_ids: Vec<_> = observations
            .iter()
            .map(|observation| observation.observation_id.clone())
            .collect();
        let unique: BTreeSet<_> = observation_ids.iter().collect();
        if unique.len() != observation_ids.len() {
            return Err(AuditError::DuplicateObservation);
        }

        let unsigned = UnsignedManifest {
            schema_version: PACKAGE_SCHEMA_VERSION,
            tenant_id: &tenant_id,
            scope_id: &scope_id,
            assessment_period: &assessment_period,
            observation_ids: &observation_ids,
        };
        let manifest_sha256 = digest_serializable(&unsigned)?;
        let package_id = digest_serializable(&(
            "canonical.audit-package-id/v1",
            &manifest_sha256,
            &observation_ids,
        ))?;

        Ok(Self {
            manifest: AuditPackageManifest {
                schema_version: PACKAGE_SCHEMA_VERSION.to_owned(),
                package_id,
                tenant_id,
                scope_id,
                assessment_period,
                observation_ids,
                manifest_sha256,
            },
            observations,
        })
    }

    /// Revalidates all package invariants and digests after deserialization or persistence.
    ///
    /// # Errors
    ///
    /// Returns an [`AuditError`] when the package cannot be reproduced exactly from its content.
    pub fn verify(&self) -> Result<(), AuditError> {
        if self.manifest.schema_version != PACKAGE_SCHEMA_VERSION {
            return Err(AuditError::UnsupportedSchema(
                self.manifest.schema_version.clone(),
            ));
        }
        let rebuilt = Self::build(
            self.manifest.tenant_id.clone(),
            self.manifest.scope_id.clone(),
            self.manifest.assessment_period.clone(),
            self.observations.clone(),
            &CapabilityGrant {
                tenant_id: self.manifest.tenant_id.clone(),
                scope_id: self.manifest.scope_id.clone(),
                capabilities: BTreeSet::from([Capability::AssembleAuditPackage]),
            },
        )?;
        if rebuilt.manifest != self.manifest || rebuilt.observations != self.observations {
            return Err(AuditError::PackageManifestMismatch);
        }
        Ok(())
    }

    /// Persists a verified package to a new file and refuses to overwrite any existing path.
    ///
    /// # Errors
    ///
    /// Returns an [`AuditError`] when verification, serialization, creation, flush, or durable sync
    /// fails. An existing path is always an error.
    pub fn persist_new(&self, path: impl AsRef<Path>) -> Result<(), AuditError> {
        self.verify()?;
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path.as_ref())?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, self)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        writer.get_ref().sync_all()?;
        Ok(())
    }

    /// Loads and verifies a persisted package before returning it.
    ///
    /// # Errors
    ///
    /// Returns an [`AuditError`] when the file cannot be read, parsed, or verified.
    pub fn load_verified(path: impl AsRef<Path>) -> Result<Self, AuditError> {
        let file = File::open(path.as_ref())?;
        let package: Self = serde_json::from_reader(BufReader::new(file))?;
        package.verify()?;
        Ok(package)
    }
}
