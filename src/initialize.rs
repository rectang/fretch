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
    init_head(path)?;
    init_config(path)?;
    Ok(())
}

fn init_repo_dirs(repo_path: &Path) -> io::Result<()> {
    static DIRS: [&'static str; 4] = [
        "info",
        "objects",
        "refs",
        "refs/heads",
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

fn init_head(repo_path: &Path) -> io::Result<()> {
    static HEAD_CONTENT: &str = "ref: refs/heads/master\n";
    let head_path = PathBuf::from(repo_path).join("HEAD");
    fs::write(head_path, HEAD_CONTENT)
}

fn init_config(repo_path: &Path) -> io::Result<()> {
    // TODO: Implement this as a Config type.
    static CONFIG_CONTENT: &str = "
[core]
	repositoryformatversion = 0
	filemode = true
	bare = true
	logallrefupdates = true
	ignorecase = true
	precomposeunicode = true
";
    let config_path = PathBuf::from(repo_path).join("config");
    fs::write(config_path, CONFIG_CONTENT)
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
