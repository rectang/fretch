// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

use std::io;

use crate::object::{Accumulator, Object};

pub struct Blob<'a> {
    content: &'a mut [u8],
}

impl<'a> Blob<'a> {
    pub fn new(content: &'a mut [u8]) -> Blob<'a> {
        Blob { content: content }
    }
}

impl<'a> Object for Blob<'a> {
    fn accumulate(&mut self, sink: &mut Accumulator) -> io::Result<()> {
        let len_str = format!("{}", self.content.len());
        sink.ingest(b"blob ")?;
        sink.ingest(len_str.as_bytes())?;
        sink.ingest(b"\0")?;
        sink.ingest(&mut self.content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_put() {
        let test_dir = "_fretch_blob_test"; // careful will rm -rf
        if let Ok(_) = fs::metadata(test_dir) {
            fs::remove_dir_all(test_dir).unwrap(); // clean up if needed
        }
        fs::create_dir(test_dir).unwrap();

        let mut content = b"hello world\n".to_owned();
        let mut hello = Blob::new(&mut content);
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
