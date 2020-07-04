mod r#trait;

extern crate num_traits;

fn main() {
    println!("{}", r#trait::basic_bench(1_000_000));
}
