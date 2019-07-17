extern crate crc32fast;

use {
    crc32fast::Hasher,
    std::io::{self, BufRead},
};

fn main() {
    let mut hasher = Hasher::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    while let Ok(buf) = stdin.fill_buf() {
        let len = buf.len();
        if len == 0 {
            break;
        }

        hasher.update(buf);

        stdin.consume(len);
    }
    let checksum = hasher.finalize();
    println!("{:x}", checksum);
}
