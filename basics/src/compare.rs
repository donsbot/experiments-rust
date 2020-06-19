// structural equality. 
fn main() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);
    assert!(*rrx <= *rry);
    assert!(*rrx == *rry);
    assert!(*rx <= *ry);
    assert!(*rx == *ry);
    assert!(rx <= ry);
    assert!(rx == ry);

    assert!(!std::ptr::eq(rx, ry));

    let a = factorial(10);
    println!("{}", a);
    let a = factorial(33);
    println!("{}", a);

    let r = &factorial;
    println!("{}",(*r)(6));
    println!("{}",r(6));

    // arith sees through one level of indirection
    assert_eq!(&r(6) + &1009, 1729); // fun
}

fn factorial(n: i128) -> i128 {
    (1..n+1).fold(1, |a, b| a * b)
}
