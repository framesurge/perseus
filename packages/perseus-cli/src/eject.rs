use crate::errors::*;
use std::fs;
use std::path::PathBuf;

/// Ejects the user from the Perseus CLi harness by exposing the internal subcrates to them. All this does is remove `.perseus/` from
/// the user's `.gitignore` and add a file `.ejected` to `.perseus/`.
pub fn eject(dir: PathBuf) -> Result<(), EjectionError> {
    // Create a file declaring ejection so `clean` throws errors (we don't want the user to accidentally delete everything)
    let ejected = dir.join(".perseus/.ejected");
    fs::write(
        &ejected,
        "This file signals to Perseus that you've ejected. Do NOT delete it!",
    )
    .map_err(|err| EjectionError::GitignoreUpdateFailed { source: err })?;
    // Now remove `.perseus/` from the user's `.gitignore`
    let gitignore = dir.join(".gitignore");
    if gitignore.exists() {
        let content = fs::read_to_string(&gitignore)
            .map_err(|err| EjectionError::GitignoreUpdateFailed { source: err })?;
        let mut new_content_vec = Vec::new();
        // Remove the line pertaining to Perseus
        // We only target the one that's exactly the same as what's automatically injected, anything else can be done manually
        let mut have_changed = false;
        for line in content.lines() {
            if line != ".perseus/" {
                new_content_vec.push(line);
            } else {
                have_changed = true;
            }
        }
        let new_content = new_content_vec.join("\n");
        // Make sure we've actually changed something
        if !have_changed {
            return Err(EjectionError::GitignoreLineNotPresent);
        }
        fs::write(&gitignore, new_content)
            .map_err(|err| EjectionError::GitignoreUpdateFailed { source: err })?;

        Ok(())
    } else {
        // The file wasn't found
        Err(EjectionError::GitignoreUpdateFailed {
            source: std::io::Error::from(std::io::ErrorKind::NotFound),
        })
    }
}

/// Checks if the user has ejected or not. If they have, commands like `clean` should fail unless `--force` is provided.
pub fn has_ejected(dir: PathBuf) -> bool {
    let ejected = dir.join(".perseus/.ejected");
    ejected.exists()
}
