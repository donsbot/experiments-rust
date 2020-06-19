fn main() {
    print_padovan();
}

fn print_padovan() {
    let a = vec![1,1,1];  // local var on stack, ptr to heap 
    let mut b = a; // drops a
    for i in 3..10 {
        let next = b[i-3] + b[i-2];
        b.push(next);
    }
    println!("P(1..10) = {:?}", b);
}
