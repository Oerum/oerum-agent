use std::path::Path;

use anyhow::Result;
use brain_core::{run_diagnostics, DiagnosticStatus};

pub fn run(cwd: &Path) -> Result<()> {
    for item in run_diagnostics(cwd) {
        let state = match item.status {
            DiagnosticStatus::Ok => "OK",
            DiagnosticStatus::Warn => "WARN",
            DiagnosticStatus::Fail => "FAIL",
        };
        println!("[{state}] {} - {}", item.check, item.detail);
        if let Some(help) = item.remediation {
            println!("  remediation: {help}");
        }
    }
    Ok(())
}
