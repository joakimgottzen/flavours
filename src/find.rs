use anyhow::{anyhow, Result};
use glob::glob;
use path::{Path, PathBuf};
use std::path;

/// Find function
///
/// * `pattern` - Which pattern to use
/// * `schemes_dir` - Schemes directory
pub fn find(pattern: &str, schemes_dir: &Path) -> Result<Vec<PathBuf>> {
    let dir = schemes_dir
        .to_str()
        .ok_or_else(|| anyhow!("Unable to convert path"))?;

    let pattern = format!("{}/*/{}.y*ml", dir, pattern);
    let matches = glob(&pattern)?;

    let mut found = Vec::new();
    for element in matches {
        found.push(element?);
    }

    Ok(found)
}

/// Get schemes function
///
/// * `patterns`: Which pattern to use
/// * `base_dir`: flavours base data dir
pub fn get_schemes(patterns: Vec<&str>, base_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find(pattern, &base_dir.join("base16").join("schemes"))?;

        for found_scheme in found_schemes {
            schemes.push(found_scheme);
        }
    }

    //Sort and remove duplicates
    schemes.sort();
    schemes.dedup();
    Ok(schemes)
}
