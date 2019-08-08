// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain. See <http://creativecommons.org/publicdomain/zero/1.0/>

use fretch::blob::Blob;
use fretch::Engine;

fn main() {
    let repo = "fretch_demo_hello/.git";
    let mut engine = Engine::new(repo).unwrap();
    engine.init().unwrap();
    let mut content = b"hello world\n".to_owned();
    let mut hello = Blob::new(&mut content);
    engine.store(&mut hello).unwrap();
}
