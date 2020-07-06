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

    // generating iterators by references and values
    #[allow(clippy::into_iter_on_ref)]
    {
        use std::collections::BTreeSet;
        let mut f = BTreeSet::new();
        f.insert("foos".to_string());
        f.insert("bars".to_string());

        // some kind of traversal
        let r = &f;
        let mut i = r.into_iter(); // shared mut ref
        assert_eq!(i.next(), Some(&"bars".to_string()));
        assert_eq!(i.next(), Some(&"foos".to_string()));
        assert_eq!(i.next(), None);

        let mut i = f.into_iter();
        assert_eq!(i.next(), Some("bars".to_string()));

        // by value
    }

    // generics
    {
        use std::fmt::Debug;

        fn dump<T, U>(t: T)
            where T: IntoIterator<Item=U>,
                  U: Debug
        {
            for u in t {
                println!("{:?}", u);
            }
        }
        let v = vec!["abc","abcd","foo","bar"];
        dump(v);

    }

    // drain: passes ownership , drops the rest
    {
        use std::iter::FromIterator;

        // take a slice
        let mut outer = "Earth".to_string();
        let inner = String::from_iter(outer.drain(1..4));
        assert_eq!(outer, "Eh");
        assert_eq!(inner, "art");


    }

}
