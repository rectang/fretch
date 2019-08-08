// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

//! # Fretch
//!
//! Persistent undo engine using Git-compatible format
//!
//! ```rust
//! use fretch::Engine;
//!
//! let engine = Engine::new("/path/to/repo").unwrap();
//! engine.init();
//! // ...
//!
//! ```
//!
//! With Fretch, an application is modeled as a directory full of files, and
//! the commands in the application are modeled as changesets.

use std::env;
use std::io;
use std::path::PathBuf;

mod initialize;
pub mod mutex;
pub mod object;

pub struct Engine {
    repo_path: PathBuf,
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
        Ok(Engine { repo_path: path })
    }

    pub fn init(&self) -> io::Result<()> {
        initialize::init_repo(&self.repo_path)
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
