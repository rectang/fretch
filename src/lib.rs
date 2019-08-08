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
//! With Fretch, an application is modeled as a directory full of files, and
//! the commands in the application are modeled as changesets.

use std::env;
use std::io;
use std::path::PathBuf;

use crate::object::Object;

pub mod blob;
mod initialize;
pub mod mutex;
pub mod object;

pub struct Engine {
    repo_path: PathBuf,
    objects_dir: PathBuf,
}

impl Engine {
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

    pub fn init(&self) -> io::Result<()> {
        initialize::init_repo(&self.repo_path)
    }

    /// Store the object in the repository's content-addressable store.
    pub fn store<O: Object>(&mut self, obj: &mut O) -> io::Result<()> {
        obj.store(&self.objects_dir)
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
