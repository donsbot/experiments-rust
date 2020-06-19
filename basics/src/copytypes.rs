fn main() {

    #[derive(Copy,Clone)] // make it a copy type
    struct Label { 
            ns: u32
    }

    fn print(l: Label) { println!("STAMP: {}", l.ns); }

    let l = Label { ns: 3 };
    print(l); // move
    println!("My label number is: {}", l.ns)

}
