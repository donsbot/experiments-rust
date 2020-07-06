mod r#trait;

extern crate num_traits;

fn main() {
    println!("{}", r#trait::basic_bench(1_000_000));
    // println!("{}", iter_bench(1_000_000));
}

/*
 * Same code generated as for Stream w/ Skip
 */

/*
fn iter_bench(n: i64) -> i64 {
    (0 .. n)
        .filter(|n| n % 2 == 1)
        .map(|n| n * 3)
        .fold(0, |n, x| n + x)
}
*/
