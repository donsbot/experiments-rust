fn main() {
    let x: &str = match Some(("foo", 7)) {
        None => {"none"}
        Some((y@"bar", _)) => {y}
        Some(("foo", 7)) => {"foo_7"}
        _ => {"anything else"}
    };
    println!("{}",x);

    let lim = 100;
    let y = f2(lim);
    println!("{}", y);
    let y = f1(lim);
    println!("{}", y);

    let lim = 1000000000;
    let y = f2(lim);
    println!("{}", y);
    let y = f1(lim);
    println!("{}", y);

}

// interesting. no tail recursion
// the obvious way:
fn f1(n: i64) -> i64 {
    go(n,0)
}

fn go(n:i64, acc:i64) -> i64 {
    if n <= 0 {
        acc
    } else {
        go(n-1, acc + n)
    }
}

// explicit loop with mutable params
fn f2(n: i64) -> i64 {
    go2(n)
}

fn go2(n:i64) -> i64 {
    let mut acc = 0;
    let mut m   = n;
    loop {
        if m <= 0 {
            return acc;
        } else {
            acc += m;
            m -= 1;
        }
    }

}
