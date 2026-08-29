//! Hermetic fixture CLI and fail-closed live-authority check.

use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};

use canonical_evidence_connectors::{FixtureGithubTransport, GithubCollector, LiveGithubAuthority};

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args().skip(1);
    let command = arguments.next().ok_or(
        "usage: connectorctl collect-fixture <path> <tenant> <organization> <collected-at> | live-check",
    )?;
    match command.as_str() {
        "collect-fixture" => {
            let collect_arguments = arguments.collect::<Vec<_>>();
            collect_fixture(&collect_arguments)
        }
        "live-check" => {
            let authority = LiveGithubAuthority::from_environment()?;
            authority.require_production_transport()?;
            Ok(())
        }
        _ => Err(format!("unknown command: {command}").into()),
    }
}

fn collect_fixture(arguments: &[String]) -> Result<(), Box<dyn Error>> {
    if arguments.len() != 4 {
        return Err("collect-fixture requires path, tenant, organization, and collected-at".into());
    }
    let fixture = fs::read_to_string(&arguments[0])?;
    let collected_at: i64 = arguments[3].parse()?;
    let transport = FixtureGithubTransport::from_json(&fixture)?;
    let collector = GithubCollector::new(transport);
    let batch = collector.collect(&arguments[1], &arguments[2], collected_at)?;
    let mut output = io::BufWriter::new(io::stdout().lock());
    serde_json::to_writer_pretty(&mut output, &batch)?;
    output.write_all(b"\n")?;
    output.flush()?;
    Ok(())
}
