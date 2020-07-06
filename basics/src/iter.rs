// ch 15: pg 322

pub fn main() {
    #[allow(clippy::while_let_on_iterator)]
    #[allow(clippy::into_iter_on_ref)]
    {
        let v = vec!["abc","abcd","foo","bar"];
        for e in &v {
            println!("{}", e);
        }

        // vec ref to iterator
        let mut i = (&v).into_iter();
        while let Some(e) = i.next() {
            println!("{}", e);
        }

        // vec ref to iterator
        let mut i = 1 .. 10; // directly construct an iterator object
        while let Some(e) = i.next() {
            println!("{}", e);
        }
        println!("{:?}", i.next());
        println!("{:?}", i.next());

    }

}
