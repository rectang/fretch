// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use libflate::zlib;
use sha1;

trait Object {

    /// Represent the object as the content needed for content-addressable
    /// store, pre-Zlib-encoding.
    fn serialize(&self) -> &[u8];

    /// Put the Object into the content-addressable store.
    fn store<P: Into<PathBuf>>(&self, objects_dir: P) -> io::Result<()> {
        let objects_dir = objects_dir.into();

        // Calculate the SHA1 checksum from the serialized object content.
        let mut bytes = self.serialize();
        let mut accum = sha1::Sha1::new();
        accum.update(bytes);
        let digest = accum.digest().to_string();

        // Prep directory and filepath.
        let dir = &digest[0..2];
        let filename = &digest[2..];
        let fulldir = objects_dir.join(dir);
        let _ = fs::create_dir(&fulldir); // Catch error on file write.
        let fullpath = fulldir.join(filename);

        // Compress content using Zlib and write out.
        let mut encoder = zlib::Encoder::new(Vec::new())?;
        io::copy(&mut bytes, &mut encoder)?;
        let encoded_data = encoder.finish().into_result()?;
        fs::write(fullpath, encoded_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Hello();
    impl Object for Hello {
        fn serialize(&self) -> &[u8] {
            b"blob 12\0hello world\n"
        }
    }

    #[test]
    fn test_store() {
        let test_dir = "_fretch_object_test"; // careful will rm -rf
        if let Ok(_) = fs::metadata(test_dir) {
            fs::remove_dir_all(test_dir).unwrap(); // clean up if needed
        }
        fs::create_dir(test_dir).unwrap();

        let hello = Hello {};
        hello.store(test_dir).unwrap();

        let expected: PathBuf = [test_dir, "3b", "18e512dba79e4c8300dd08aeb37f8e728b8dad"]
            .iter()
            .collect();
        match fs::read(expected) {
            Err(e) => panic!("{}", e),
            Ok(_) => {}
        }

        fs::remove_dir_all(test_dir).unwrap(); // clean up
    }
}
