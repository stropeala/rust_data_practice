use std::env::var;
use std::fs::DirBuilder;
use std::path::PathBuf;

use anyhow::Context;

pub fn data_file(name: &str) -> Result<PathBuf, anyhow::Error> {
    let data_dir =
        PathBuf::from(var("DATA_DIR_PATH").context("Could not get Data directory path from env!")?);
    if !data_dir.exists() {
        DirBuilder::new()
            .create(&data_dir)
            .context("Could not create Data directory!")?;
    }
    Ok(data_dir.join(name))
}
