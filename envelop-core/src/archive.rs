use std::path::{Path, PathBuf};

use tar::{Archive, Builder};

pub fn pack_files(paths: &[PathBuf]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut builder = Builder::new(Vec::new());

    for path in paths {
        builder.append_path(path)?;
    }

    Ok(builder.into_inner()?)
}

pub fn unpack_files(archive: &[u8], output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut archive = Archive::new(archive);
    archive.unpack(output_dir)?;
    Ok(())
}
