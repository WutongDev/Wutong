use md5::{Digest, Md5};
use std::io::Write;

pub fn md5_text(text: String) -> String {
    let mut hasher = Md5::new();
    hasher.write_all(text.as_bytes()).unwrap();
    let result = hex::encode(hasher.finalize());
    result
}
