use anyhow::{bail, Result};

pub fn ensure_supported(schema_version: u32) -> Result<()> {
    if schema_version == 1 {
        Ok(())
    } else {
        bail!("unsupported schema version: {schema_version}")
    }
}
