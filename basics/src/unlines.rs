use std::io;
use std::io::prelude::*;

pub fn main() {
    let f = io::stdin();
    for l in f.lock().lines() {
        println!("{:?}", l);
    }
}
