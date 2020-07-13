// ch18: IO . page 432.
//
use std::fs::File;

pub fn main() {
    let h = File::open("io.rs").unwrap();
    let m = h.metadata().unwrap();
    println!("{:?}", m);

    assert_eq!(false, m.is_dir());
}
