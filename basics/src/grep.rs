// grep like thing pg 432

use std::io;
use std::io::prelude::*;

fn grep(target: &str) -> io::Result<()> {
    let stdin = io::stdin();

    for res in stdin.lock().lines() {
        let l = res?;
        if l.contains(target) {
            println!("{}", l);
        }
    }
    Ok(())
}

pub fn main() {
    grep("for").unwrap();
}
