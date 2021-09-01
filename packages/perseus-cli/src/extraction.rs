// This file contains a temporary fix for the issues with recursive extraction in `include_dir`
// Tracking issue is https://github.com/Michael-F-Bryan/include_dir/issues/59

use std::path::Path;
use include_dir::Dir;
use std::io::Write;

/// Extracts a directory included with `include_dir!` until issue #59 is fixed on that module (recursive extraction support).
pub fn extract_dir<S: AsRef<Path>>(dir: Dir, path: S) -> std::io::Result<()> {
    let path = path.as_ref();

    // Create all the subdirectories in here (but not their files yet)
    for dir in dir.dirs() {
        std::fs::create_dir_all(path.join(dir.path()))?;
        // Recurse for this directory
        extract_dir(*dir, path)?;
    }

    // Write all the files at the root of this directory
    for file in dir.files() {
        let mut fsf = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.join(file.path()))?;
        fsf.write_all(file.contents())?;
        fsf.sync_all()?;
    }

    Ok(())
}
