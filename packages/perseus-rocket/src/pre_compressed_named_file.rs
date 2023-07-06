use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::ops::{Deref, DerefMut};

use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket::http::ContentType;

/// A file with an associated name; responds with the Content-Type based on the
/// file extension.
#[derive(Debug)]
pub struct PreCompressedBrNamedFile(PathBuf, File, bool);

impl PreCompressedBrNamedFile {
    /// Attempts to open a file in read-only mode.
    /// Looking for a pre compressed brotli file at the same path with a .br ending.
    /// If the pre compressed file does not exist default back to the uncompressed file.
    ///
    /// # Errors
    ///
    /// This function will return an error if path does not already exist. Other
    /// errors may also be returned according to
    /// [`OpenOptions::open()`](std::fs::OpenOptions::open()).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket::response::NamedFile;
    ///
    /// # #[allow(unused_variables)]
    /// let file = NamedFile::open("foo.txt");
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<PreCompressedBrNamedFile> {
        let compressed_path = format!("{}.br", path.as_ref().to_string_lossy());
        match File::open(compressed_path){
            Ok(file) => Ok(PreCompressedBrNamedFile(path.as_ref().to_path_buf(), file, true)),
            Err(_) => {
                let file = File::open(path.as_ref())?;
                Ok(PreCompressedBrNamedFile(path.as_ref().to_path_buf(), file, false))
            }
        }
    }

    /// Retrieve the underlying `File`.
    #[inline(always)]
    pub fn file(&self) -> &File {
        &self.1
    }

    /// Take the underlying `File`.
    #[inline(always)]
    pub fn take_file(self) -> File {
        self.1
    }

    /// Retrieve a mutable borrow to the underlying `File`.
    #[inline(always)]
    pub fn file_mut(&mut self) -> &mut File {
        &mut self.1
    }

    /// Retrieve the path of this file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::io;
    /// use rocket::response::NamedFile;
    ///
    /// # #[allow(dead_code)]
    /// # fn demo_path() -> io::Result<()> {
    /// let file = NamedFile::open("foo.txt")?;
    /// assert_eq!(file.path().as_os_str(), "foo.txt");
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }
}

/// Streams the named file to the client. Sets or overrides the Content-Type in
/// the response according to the file's extension if the extension is
/// recognized. See [`ContentType::from_extension()`] for more information. If
/// you would like to stream a file with a different Content-Type than that
/// implied by its extension, use a [`File`] directly.
/// If the file is flagged as pre compressed brotli, add the Content-Encoding: br header
impl<'r> Responder<'r, 'r> for PreCompressedBrNamedFile {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let mut response = self.1.respond_to(req)?;
        if let Some(ext) = self.0.extension() {
            if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy()) {
                response.set_header(ct);
            }
        }

        if self.2 {
            response.set_raw_header("Content-Encoding", "br");
        }

        Ok(response)
    }
}

impl Deref for PreCompressedBrNamedFile {
    type Target = File;

    fn deref(&self) -> &File {
        &self.1
    }
}

impl DerefMut for PreCompressedBrNamedFile {
    fn deref_mut(&mut self) -> &mut File {
        &mut self.1
    }
}

impl io::Read for PreCompressedBrNamedFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file().read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.file().read_to_end(buf)
    }
}

impl io::Write for PreCompressedBrNamedFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file().flush()
    }
}

impl io::Seek for PreCompressedBrNamedFile {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.file().seek(pos)
    }
}

impl<'a> io::Read for &'a PreCompressedBrNamedFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file().read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.file().read_to_end(buf)
    }
}

impl<'a> io::Write for &'a PreCompressedBrNamedFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file().flush()
    }
}

impl<'a> io::Seek for &'a PreCompressedBrNamedFile {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.file().seek(pos)
    }
}
