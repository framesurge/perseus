use std::thread::{self, JoinHandle};

/// Spawns a new thread with the given code, or executes it directly if the
/// environment variable `PERSEUS_CLI_SEQUENTIAL` is set to any valid (Unicode)
/// value. Multithreading is the default.
pub fn spawn_thread<F, T>(f: F, sequential: bool) -> ThreadHandle<F, T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    if sequential {
        ThreadHandle {
            join_handle: None,
            f: Some(f),
        }
    } else {
        let join_handle = thread::spawn(f);
        ThreadHandle {
            join_handle: Some(join_handle),
            f: None,
        }
    }
}

/// An abstraction over a `JoinHandle` in a multithreaded case, or just a
/// similar interface that will immediately return if otherwise. This allows the
/// interfaces for multithreading and single-threading to be basically
/// identical.
pub struct ThreadHandle<F, T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    /// If multithreaded, this is the join handle.
    join_handle: Option<JoinHandle<T>>,
    // If single-threaded, this is the output (it's already been executed).
    f: Option<F>,
}
impl<F, T> ThreadHandle<F, T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    /// Waits for the 'thread' to complete, properly if it's multithreaded, or
    /// by direct execution if it's single-threaded.
    pub fn join(
        self,
    ) -> Result<T, std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>> {
        if let Some(join_handle) = self.join_handle {
            join_handle.join()
        } else if let Some(f) = self.f {
            let output = f();
            Ok(output)
        } else {
            unreachable!();
        }
    }
}
