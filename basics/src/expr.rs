fn main() {
    let x: &str = match Some(("foo", 7)) {
        None => {"none"}
        Some((y@"bar", _)) => {y}
        Some(("foo", 7)) => {"foo_7"}
        _ => {"anything else"}
    };
    println!("{}",x);
}
