use std::path::PathBuf;
use std::process::Command;

/// Gets the contents of the given filename at the given Git version by executing `git show`. This given filename MUST be relative to
/// the root directory of `git_dir`.
pub fn get_file_at_version(
    filename: &str,
    version: &str,
    git_dir: PathBuf,
) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(["show", &format!("{}:{}", version, filename)])
        .current_dir(git_dir)
        .output()?;
    let contents = String::from_utf8(output.stdout)
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))?;
    Ok(contents)
}
