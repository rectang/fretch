// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

use std::fs;
use std::io;
use std::path::PathBuf;

use libflate::zlib;
use sha1;

pub struct Accumulator {
    encoder: zlib::Encoder<std::vec::Vec<u8>>,
    digester: sha1::Sha1,
}

impl Accumulator {
    /// Process the next in an ordered sequence of byte slices which represent
    /// the serialized Object.
    pub fn ingest(&mut self, mut slice: &[u8]) -> io::Result<()> {
        self.digester.update(&slice);
        io::copy(&mut slice, &mut self.encoder)?;
        Ok(())
    }
}

pub trait Object {
    /// Collect a sequence of byte slices into the supplied `sink` which
    /// when placed end-to-end represent the serialized form of the object.
    fn accumulate(&mut self, sink: &mut Accumulator) -> io::Result<()>;

    /// Put the Object into the content-addressable store.
    fn put<P: Into<PathBuf>>(&mut self, objects_dir: P) -> io::Result<()> {
        let objects_dir = objects_dir.into();

        // Accumulate the Object as a sequence of serialized byte slices.
        let mut sink = Accumulator {
            digester: sha1::Sha1::new(),
            encoder: zlib::Encoder::new(Vec::new())?
        };
        self.accumulate(&mut sink)?;

        // Calculate the SHA1 checksum from the serialized object content.
        // Prep directory and filepath.
        let digest = sink.digester.digest().to_string();
        let dir = &digest[0..2];
        let filename = &digest[2..];
        let fulldir = objects_dir.join(dir);
        let _ = fs::create_dir(&fulldir); // Catch error on file write.
        let fullpath = fulldir.join(filename);

        // Compress content using Zlib and write out.
        let encoded_data = sink.encoder.finish().into_result()?;
        fs::write(fullpath, encoded_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Hello();
    impl Object for Hello {
        fn accumulate(&mut self, sink: &mut Accumulator) -> io::Result<()> {
            sink.ingest(b"blob 12\0hello world\n")
        }
    }

    #[test]
    fn test_put() {
        let test_dir = "_fretch_object_test"; // careful will rm -rf
        if let Ok(_) = fs::metadata(test_dir) {
            fs::remove_dir_all(test_dir).unwrap(); // clean up if needed
        }
        fs::create_dir(test_dir).unwrap();

        let mut hello = Hello {};
        hello.put(test_dir).unwrap();

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
