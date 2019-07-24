// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Initalize a repository at `path`.
pub fn init_repo(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    init_repo_dirs(path)?;
    Ok(())
}

fn init_repo_dirs(repo_path: &Path) -> io::Result<()> {
    static DIRS: [&'static str; 9] = [
        "branches",
        "hooks",
        "info",
        "objects",
        "objects/info",
        "objects/pack",
        "refs",
        "refs/heads",
        "refs/tags",
    ];
    for dir in DIRS.iter() {
        let mut buf = PathBuf::from(repo_path);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_repo() {
        let repo_path = Path::new("_fretch_test"); // Careful, will rm -rf!

        // Clean up if needed.
        if repo_path.exists() {
            fs::remove_dir_all(repo_path).unwrap();
        }

        init_repo(repo_path).unwrap();
        let repo_path_stat = fs::metadata(repo_path).unwrap();
        assert!(repo_path_stat.is_dir());

        // Clean up.
        fs::remove_dir_all(repo_path).unwrap();
    }
}
