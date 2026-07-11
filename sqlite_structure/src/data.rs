use std::env::var;
use std::fs::{DirBuilder, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::Context;

pub fn data_file(file_name: &str) -> Result<PathBuf, anyhow::Error> {
    let data_dir =
        PathBuf::from(var("DATA_DIR_PATH").context("Failed getting data dir path from env!")?);
    if !data_dir.exists() {
        DirBuilder::new()
            .create(&data_dir)
            .context("Failed creating data dir!")?;
    }
    Ok(data_dir.join(file_name))
}

pub fn get_data(file_path: &Path) -> anyhow::Result<String> {
    let mut file =
        File::open(file_path).with_context(|| format!("Failed opening {file_path:?}!"))?;
    let mut content = String::new();

    file.read_to_string(&mut content)
        .with_context(|| format!("Failed reading {file:?}!"))?;
    Ok(content)
}
