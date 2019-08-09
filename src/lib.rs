// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

//! # Fretch
//!
//! Persistent undo engine using Git-compatible format
//!
//! ```rust
//! use fretch::Engine;
//! use fretch::blob::Blob;
//!
//! let mut engine = Engine::new("fretch_demo_hello/.git").unwrap();
//! engine.init().unwrap();
//! let mut content = b"hello world\n".to_owned();
//! let mut hello = Blob::new(&mut content);
//! engine.store(&mut hello).unwrap();
//!
//! ```
//!
//! With Fretch, an application is modeled as a directory full of files.  It
//! is intended for use in applications where each command can be modeled as a
//! changeset.
//!
//! This approach offers several advantages:
//!
//! * Undo/redo capabilities persist can beyond the life of a process.
//! * Consolidation and truncation of history using familiar techniques.
//! * Offline introspection using tools from the Git ecosystem.
//! * An archive-friendly public file format which will still be readable for
//!   into the far future.

use std::env;
use std::io;
use std::path::PathBuf;

use crate::object::Object;

pub mod blob;
mod initialize;
pub mod mutex;
pub mod object;

/// Engine which represents the state of the application and which
/// encapsulates the repository.
pub struct Engine {
    repo_path: PathBuf,
    objects_dir: PathBuf,
}

impl Engine {
    /// Create a new Engine with a repository located at `path`.
    ///
    /// In this context, "repository" means a bare repository, not a working
    /// tree.
    pub fn new<P: Into<PathBuf>>(path: P) -> io::Result<Engine> {
        let mut path = path.into();
        if !path.is_absolute() {
            let cwd = match env::current_dir() {
                Ok(dir) => dir,
                Err(e) => return Err(e),
            };
            path = cwd.join(path)
        }
        let objects_dir = PathBuf::new().join(&path).join("objects");
        let engine = Engine {
            repo_path: path,
            objects_dir: objects_dir,
        };
        Ok(engine)
    }

    /// Initialize the repository.
    pub fn init(&self) -> io::Result<()> {
        initialize::init_repo(&self.repo_path)
    }

    /// Write the `Object` to the repository's content-addressable store.
    pub fn store<O: Object>(&mut self, obj: &mut O) -> io::Result<()> {
        obj.put(&self.objects_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_init() {
        let repo_path = "_fretch_core_test"; // Careful, will rm -rf!

        // Clean up if needed.
        let metadata = fs::metadata(repo_path);
        if let Ok(_) = metadata {
            fs::remove_dir_all(repo_path).unwrap();
        }

        let engine = Engine::new(repo_path).unwrap();
        engine.init().unwrap();

        // Clean up.
        fs::remove_dir_all(repo_path).unwrap();
    }
}
