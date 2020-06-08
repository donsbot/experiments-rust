extern crate num;
use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    square_add_loop(1.25);
}

#[allow(dead_code)]
fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T,T)> {
    match s.find(sep) {
        None => None,
        Some(idx) => {
            match (T::from_str(&s[..idx]), T::from_str(&s[idx + 1 ..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    let sep = ',';
    assert_eq!(parse_pair::<i32>("",        sep), None);
    assert_eq!(parse_pair::<i32>("10",      sep), None);
    assert_eq!(parse_pair::<i32>(",10",     sep), None); // no missing
    assert_eq!(parse_pair::<i32>("10,20",   sep), Some((10,20)));
    assert_ne!(parse_pair::<i32>("10,20",   sep), Some((10,10))); // equality
    assert_eq!(parse_pair::<i32>("10,20xy", sep), None); // no trailing
}

#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0 .. limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

#[allow(dead_code)]
fn complex_square_root(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

#[allow(dead_code)]
fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}
