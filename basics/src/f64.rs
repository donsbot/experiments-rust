fn main() {
    let m = 10.0_f64;
    let x = 4.0_f64;
    let b = 60.0_f64;

    // 100.0
    let abs_difference = (m.mul_add(x, b) - ((m * x) + b)).abs();

    assert!(abs_difference < 1e-10);
}
