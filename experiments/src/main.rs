fn main() {
    eprintln!("{}", f(4));
}

fn f(mut n : u64) -> u64 {
    { n += 3;
      n*2 }
}
