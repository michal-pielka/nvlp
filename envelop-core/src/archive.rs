use std::path::PathBuf;

use tar::Builder;

pub fn pack_files(paths: &[PathBuf]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut builder = Builder::new(Vec::new());

    for path in paths {
        builder.append_path(path)?;
    }

    Ok(builder.into_inner()?)
}
