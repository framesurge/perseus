mod build;
mod clean;
mod export;
mod export_error_page;
mod help;
mod new;
mod serve;
mod snoop_build;
mod snoop_serve;
mod snoop_wasm_build;
mod tools;

mod utils {
    use std::process::Command;

    /// Tests a running app by executing the given command. This will safely
    /// terminate any child processes in the event of an error.
    pub fn test_serve(cmd: &mut Command, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use command_group::CommandGroup;

        // We use a group process spawn because the child will spawn the server process,
        // which can't be cleaned up if SIGKILL is sent
        let mut child = cmd.group_spawn()?;

        std::thread::sleep(std::time::Duration::from_millis(5000));

        // Check if the child process has failed (this is the only way to catch
        // things like binding errors); this will not block trying to wait
        let exit_status = child.try_wait()?;
        if let Some(status) = exit_status {
            panic!("server process returned non-zero exit code '{}'", status);
        }

        // We don't extensively test things here, since that's what Perseus' testing
        // system is for, and that tests *all* the core examples extensively in a
        // headless browser, which is more realistic than simple HTTP requests
        // anyway
        let body = ureq::get(path)
            .call()
            .map_err(|err| {
                let _ = child.kill();
                err
            })?
            .into_string()
            .map_err(|err| {
                let _ = child.kill();
                err
            })?;
        assert!(body.contains("Welcome to Perseus!"));

        let _ = child.kill();

        Ok(())
    }
}
