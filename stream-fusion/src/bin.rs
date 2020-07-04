mod closure;

extern crate num_traits;

fn main() {
    println!("{}", closure::basic_bench(1_000_000));
}
