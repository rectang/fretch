// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

pub trait Lock {
    /// Attempt to lock the resource.  Returns true of the attempt to lock
    /// succeeds without problems, false if the lock was already successfully
    /// held by this lock, false if the attempt failed because the resource is
    /// locked by another lock, or an Error if an unexpected condition
    /// occurs.
    fn lock(&mut self) -> io::Result<bool>;

    /// Attempt to unlock the resource.  Returns true if the attempt to unlock
    /// succeeds with no problems, false if the resource was already unlocked,
    /// or an Error if a problem occurs.
    fn unlock(&mut self) -> io::Result<bool>;

    /// Indicate whether the mutex is currently held.
    fn is_locked(&self) -> bool;
}

/// Portable, multi-process-aware mutex.
pub struct LockFile {
    path: PathBuf,
    locked: bool,
}

impl LockFile {
    /// `path` must be an absolute path.
    pub fn new<P: Into<PathBuf>>(path: P) -> io::Result<LockFile> {
        let lockfile = LockFile {
            path: path.into(),
            locked: false,
        };
        if !lockfile.path.is_absolute() {
            return Err(io::Error::from(ErrorKind::InvalidInput));
        }
        Ok(lockfile)
    }
}

impl Lock for LockFile {
    /// Attempt to acquire a mutex lock on the resource.
    fn lock(&mut self) -> io::Result<bool> {
        if self.locked {
            return Ok(false);
        }

        // Rely on atomic `open` for securing lock.  On success, let `f` go
        // out of scope at the end of the function and close itself.
        let f = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.path);
        match f {
            Err(e) => return Err(e),
            Ok(_) => {
                self.locked = true;
                return Ok(true);
            }
        }
    }

    fn unlock(&mut self) -> io::Result<bool> {
        if !self.locked {
            return Ok(false);
        }
        fs::remove_file(&self.path)?;
        self.locked = false;
        Ok(true)
    }

    fn is_locked(&self) -> bool {
        self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_lockfile() {
        match LockFile::new("./relative") {
            Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidInput),
            Ok(_) => panic!("Relative dir should not succeed"),
        }
        let lock_path = env::current_dir().unwrap().join("_fretch_lock_test");
        if lock_path.exists() {
            // Clean up after previous crashed test runs.
            fs::remove_file(&lock_path).unwrap();
        }
        let mut lockfile = LockFile::new(&lock_path).unwrap();
        assert!(!lockfile.is_locked(), "LockFile unlocked by default");
        assert!(lockfile.lock().unwrap(), "Acquiring a lock returns true");
        assert!(lockfile.is_locked(), "is_locked true for locked resource");
        assert!(!lockfile.lock().unwrap(), "Locking twice returns false");
        assert!(
            lockfile.unlock().unwrap(),
            "Unlocking successfully returns true"
        );
        assert!(!lockfile.is_locked(), "is_locked false after unlocking");
        assert!(
            !lockfile.unlock().unwrap(),
            "unlocking false when already unlocked"
        );
    }
}
