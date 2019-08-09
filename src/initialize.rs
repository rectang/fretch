// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Initalize a repository at `path`.
pub fn init_repo<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    init_repo_dirs(path)?;
    init_head(path)?;
    init_config(path)?;
    Ok(())
}

/// Create all the required directories for a minimal repository.
fn init_repo_dirs<P: AsRef<Path>>(repo_path: P) -> io::Result<()> {
    static DIRS: [&'static str; 4] = [
        "info",
        "objects",
        "refs",
        "refs/heads",
    ];
    for dir in DIRS.iter() {
        let mut buf = PathBuf::from(repo_path.as_ref());
        for component in dir.split("/") {
            buf.push(component);
        }
        let dirpath = buf.as_path();
        // TODO: More robust permissions and sanity checks.
        if !dirpath.exists() {
            fs::create_dir(dirpath)?;
        }
    }
    Ok(())
}

/// Create the HEAD file.
fn init_head<P: AsRef<Path>>(repo_path: P) -> io::Result<()> {
    static HEAD_CONTENT: &str = "ref: refs/heads/master\n";
    let head_path = repo_path.as_ref().join("HEAD");
    fs::write(head_path, HEAD_CONTENT)
}

/// Create a default repository config file.
fn init_config<P: AsRef<Path>>(repo_path: P) -> io::Result<()> {
    // TODO: Implement this as a Config type.
    // TODO: These values are system specific and will cause repository
    // corruption on systems where they are inaccurate.  The proper thing to
    // do is probe for them.
    static CONFIG_CONTENT: &str = "
[core]
	repositoryformatversion = 0
	filemode = true
	bare = true
	logallrefupdates = true
	ignorecase = true
	precomposeunicode = true
";
    let config_path = repo_path.as_ref().join("config");
    fs::write(config_path, CONFIG_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_repo() {
        let repo_path = "_fretch_test"; // Careful, will rm -rf!

        // Clean up if needed.
        if let Ok(_) = fs::metadata(repo_path) {
            fs::remove_dir_all(repo_path).unwrap();
        }

        init_repo(repo_path).unwrap();
        let repo_path_stat = fs::metadata(repo_path).unwrap();
        assert!(repo_path_stat.is_dir());

        // Clean up.
        fs::remove_dir_all(repo_path).unwrap();
    }
}
