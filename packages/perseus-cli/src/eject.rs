use crate::errors::*;
use std::fs;
use std::path::PathBuf;

/// Ejects the user from the Perseus CLi harness by exposing the internal subcrates to them. All this does is remove `.perseus/` from
/// the user's `.gitignore` and add a file `.ejected` to `.perseus/`.
pub fn eject(dir: PathBuf) -> Result<()> {
    // Create a file declaring ejection so `clean` throws errors (we don't want the user to accidentally delete everything)
    let ejected = dir.join(".perseus/.ejected");
    fs::write(
        &ejected,
        "This file signals to Perseus that you've ejected. Do NOT delete it!",
    )
    .map_err(|err| ErrorKind::GitignoreEjectUpdateFailed(err.to_string()))?;
    // Now remove `.perseus/` from the user's `.gitignore`
    let gitignore = dir.join(".gitignore");
    if gitignore.exists() {
        let content = fs::read_to_string(&gitignore)
            .map_err(|err| ErrorKind::GitignoreEjectUpdateFailed(err.to_string()))?;
        let mut new_content_vec = Vec::new();
        // Remove the line pertaining to Perseus
        // We only target the one that's exactly the same as what's automatically injected, anything else can be done manually
        for line in content.lines() {
            if line != ".perseus/" {
                new_content_vec.push(line);
            }
        }
        let new_content = new_content_vec.join("\n");
        // Make sure we've actually changed something
        if content == new_content {
            bail!(ErrorKind::GitignoreEjectUpdateFailed(
                "line `.perseus/` to remove not found".to_string()
            ))
        }
        fs::write(&gitignore, new_content)
            .map_err(|err| ErrorKind::GitignoreEjectUpdateFailed(err.to_string()))?;

        Ok(())
    } else {
        bail!(ErrorKind::GitignoreEjectUpdateFailed(
            "file not found".to_string()
        ))
    }
}

/// Checks if the user has ejected or not. If they have, commands like `clean` should fail unless `--force` is provided.
pub fn has_ejected(dir: PathBuf) -> bool {
    let ejected = dir.join(".perseus/.ejected");
    ejected.exists()
}
