// ch 15: pg 322

pub fn main() {
    // introduction

    #[allow(clippy::while_let_on_iterator)]
    #[allow(clippy::into_iter_on_ref)]
    {
        let v = vec!["abc", "abcd", "foo", "bar"];
        for e in &v {
            println!("{}", e);
        }

        // vec ref to iterator
        let mut i = (&v).into_iter();
        while let Some(e) = i.next() {
            println!("{}", e);
        }

        // vec ref to iterator
        let mut i = 1..10; // directly construct an iterator object
        while let Some(e) = i.next() {
            println!("{}", e);
        }
        println!("{:?}", i.next());
        println!("{:?}", i.next());
    }

    // generation

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
        where
            T: IntoIterator<Item = U>,
            U: Debug,
        {
            for u in t {
                println!("{:?}", u);
            }
        }
        let v = vec!["abc", "abcd", "foo", "bar"];
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

    // instances
    {
        let i = (1i32..100).into_iter();
        println!("{}", i.fold(0, |n, i| n + i));

        // loops 0 or 1 times
        for i in Some(1).iter() {
            println!("{}", i);
        }

        // interesting slices on vectors
        let v = vec![1i64; 1024];
        println!("{}", v.windows(4).len()); // 4-element long  prefix scans?
        println!("{}", v.chunks(16).len()); // 64 * 16 chunks of v

        // tokenize
        let o = &[
            "asdbaf".to_string(),
            "df".to_string(),
            "ASDfdsa".to_string(),
            "TOK".to_string(),
            "Dsdfa".to_string(),
        ];
        for i in o.split(|b| *b == "TOK") {
            for l in i {
                print!("{}-", l);
            }
            println!();
        }
    }
}
