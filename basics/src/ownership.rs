fn main() {
    print_padovan();
    let s = boxing();
    println!("{}", s);
}

fn print_padovan() {
    let a = vec![1, 1, 1]; // local var on stack, ptr to heap
    let mut b = a; // drops a
    for i in 3..10 {
        let next = b[i - 3] + b[i - 2];
        b.push(next);
    }
    println!("P(1..10) = {:?}", b); // drop b and reclaim the heap
}

fn boxing() -> String {
    let point = Box::new((0.625, 0.5)); // ptr to heap alloc
    let label = format!("{:?}", point); // ptr to heap alloc str
    label
}
