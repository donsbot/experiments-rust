use std::str::FromStr;

fn main() {
    let mut ns: Vec<u64> = Vec::new();

    for arg in std::env::args().skip(1) {
        let i = u64::from_str(&arg);
        let r = i.expect("Parser error on numeric argument");
        ns.push(r);
    }

    if ns.is_empty() {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    // foldl1 over args
    let mut r = ns[0]; // there's at least one argument
    for p in &ns[1..] {
        r = gcd(r, *p);
    }

    println!("The GCD of {:?} is {}", ns, r);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 77 * 11 * 13 * 19), 3 * 11);
}

#[allow(dead_code)]
// allocate a new vector and poke some values in
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10i16);
    v.push(20);
    v
}

// fn str_games() -> &str {
//     let t = "I see the eigenvalue in thine eye";
//     let (x, _xs) = str::split_at(t,21);
//     x
// }
