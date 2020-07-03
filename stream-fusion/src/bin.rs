mod r#trait;

extern crate num_traits;

fn main() {
    println!("{}", r#trait::basic_bench(100_000_000));
}
