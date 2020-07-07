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
    #[allow(clippy::unnecessary_fold)]
    {
        let i = 1i32..100; // .into_iter();
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

    // adaptors ("transformers")
    // more parsing stuff
    {
        let text = "     ponies \n giraffes \n monkeys\nsquid".to_string();
        let v: Vec<&str> =
            text.lines()
                .map(str::trim)
                .filter(|s| *s != "monkeys")
                .collect();
        assert_eq!(v, ["ponies","giraffes","squid"]);

    }

    // laziness
    #[allow(unused_must_use)]
    {
        // this does nothing
        ["launch","the","missles"].iter().map(|e|println!("fire: {}",e));
    }

    // more functions
    {
        let j = (0i64..100) // twice to avoid sharing
                 .filter_map(|n| if n % 2 == 0 { Some (n*2) } else { None })
                 .take(10);
        for i in j {
            println!("{}",i);
        }
        let k = (0i64..100)
                 .filter(|n| n % 2 == 0)
                 .map(|n| n * 2)
                 .take(10);
        for i in k {
            println!("{}",i);
        }

        use std::str::FromStr;

        let text ="1\nfrond .25 289\n3.1415 estuary\n";
        for n in text.split_whitespace()
                     .filter_map(|w| f64::from_str(w).ok()) {
                         println!("{:4.2}", n.sqrt());
        }
    }

    // flat_map / concat_map
    {
        let j = 0i64 .. 20;
        let k: Vec<Vec<i64>> = j.map(|i| { (0 .. i).collect() }).collect();
        for x in k {
            let l = x.len();
            for _ in x {
                print!("#");
            }
            println!("{}",l);
        }

        use std::collections::HashMap;

        let mut ms = HashMap::new();
        ms.insert("Japan", vec!["Tokyo", "Kyoto"]);
        ms.insert("Australia", vec!["Sydney", "Darwin"]);

        let countries = ["Australia", "Japan"];

        // flatten a set of keys. index into a hashtable
        for &city in countries.iter().flat_map(|c| &ms[c]) {
            println!("{}",city);
        }
    }

    // scan
    {
        let iter = (0..20)
            .scan(0, |sum, i| { // short circuiting scanl
                *sum += i;
                if *sum > 1000 {
                    None
                } else {
                    Some(i * i)
                }
            });

        for x in iter.collect::<Vec<i32>>() {
            print!("{}-",x);
        }
        println!();

    }

    // take , take_while
    {
        let iter = (0..20)
            .take_while(|n| *n <= 10)
            .take(3);
        for x in iter.collect::<Vec<i32>>() {
            print!("{}-",x);
        }
        println!();

    }

    // skips (can't call it drop!)
    {
        let iter = (0..20)
            .skip_while(|n| *n <= 10)
            .skip(3);
        for x in iter.collect::<Vec<i32>>() {
            print!("{}-",x);
        }
        println!();
    }

    // peekable... look at the next item in the iterator but dont' consume it. lookahead?
    {
        use std::iter::Peekable;

        // pull first set of digits off a string
        fn parse_number<I> (toks: &mut Peekable<I>) -> u32
            where I: Iterator<Item=char>
        {
            let mut n = 0;
            loop {
                match toks.peek() {
                    Some(r) if r.is_digit(10) => {
                        n = n*10 + r.to_digit(10).unwrap();
                    }
                    _ => return n
                }
                toks.next();
            }
        }
        // iterator of characters
        let mut chars = "226153290,112312321".chars().peekable();

        let x = parse_number(&mut chars);
        println!("{:?}",x);
        let x = parse_number(&mut chars);
        println!("{:?}",x);
        chars.next(); // bump state. 
        let x = parse_number(&mut chars);
        println!("{:?}",x);
    }

    // fuse : Iterators are stateful, you can observe if they have been evaluated
    {
        // fuse ensures an adaptor that returns None idempotently continues to do so
        struct Flaky(bool);

        impl Iterator for Flaky {
            type Item = &'static str;

            // sort of a boolean state machine
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 {
                    self.0 = false;
                    Some("the last item")
                } else {
                    self.0 = true; // flip flop
                    None
                }
            }
        }

        let mut f = Flaky(true);
        assert_eq!(f.next(), Some("the last item"));
        assert_eq!(f.next(), None);
        assert_eq!(f.next(), Some("the last item"));

        let mut f1 = Flaky(true).fuse();
        assert_eq!(f1.next(), Some("the last item"));
        assert_eq!(f1.next(), None);
        assert_ne!(f1.next(), Some("the last item"));
    }

    // reversible/ double ended iterators. a next_back() method
    // useful for arrays/slices
    {
        let bee = ["head", "thorax", "abdomen"];

        let mut iter = bee.iter();

        assert_eq!(iter.next() , Some(&"head"));
        assert_eq!(iter.next_back() , Some(&"abdomen"));
        assert_eq!(iter.next() , Some(&"thorax"));
        assert_eq!(iter.next_back() , None);

        let ns: std::ops::Range<i64> = 1..100;
        let mut i = ns.rev();
        assert_eq!(i.next() , Some(99));

    }

    // inspect: debugging the composition of iterators
    {
        let uc: String = "groSsse".chars()
            .inspect(|c| println!("before: {:?}", c))
            .flat_map(|c| c.to_uppercase())
            .inspect(|c| println!("after:        {:?}", c))
            .collect();
        assert_eq!(uc, "GROSSSE");
    }

    // chaining - appending.
    {
        let a = 1..10;
        let b = -20..-10;
        let vs: Vec<i64> = a.chain(b).collect();
        println!("{:?}", vs);
    }

    // enumerate. zipwith count
    {
        let a = -20 .. -10;
        let vs: Vec<(usize,i64)> = a.enumerate().collect();
        println!("{:?}", vs);
    }

    // zip. length of the shortest iterator
    {
        let a = -20 .. -10;
        let b = -2 .. 10;
        let vs: Vec<(i64,i64)> = a.zip(b).collect();
        println!("{:?}", vs);
    }

    // using by ref than handing it back
    {
        let ms = "To: foo\r\n\
                  From: id\r\n\
                  \r\n
                  Ooooh\r\n";
        let mut lines = ms.lines();
        println!("Headers:");
        for h in lines.by_ref().take_while(|l| !l.is_empty()) {
            println!("{}", h);
        }
        println!("Body:");
        for b in lines {
            println!("{}", b);
        }
            
    }

    // a cloning iterator
    {
        let a = ['1', '2', '3'];
        assert_eq!(a.iter().next(), Some(&'1'));
        assert_eq!(a.iter().cloned().next(), Some('1'));
    }

    // cycle!
    {
        let a = 0i64..4;
        let v = a.cycle();
        println!("{:?}", v.take(20).collect::<Vec<_>>());

    }

    // fizz buzz in a lazy list style
    {
        use std::iter::{once, repeat};

        let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
        let buzzes = repeat("").take(4).chain(once("buzz")).cycle();

        let fizz_buzz = (1..10).zip(fizzes.zip(buzzes))
            .map(|i|
                 match i {
                     (i, ("","")) => i.to_string(),
                     (_, (f,b)) => format!("{}{}", f, b)
                 });
        for l in fizz_buzz { 
            println!("{}", l);
        }

    }

    // consuming
    {
        println!("{}", (1..30).by_ref().product::<i128>());
        println!("{}", (1..30).sum::<i16>());
        println!("{:?}", (1..30).max());
        println!("{:?}", (1..30).min());

        println!("{}", [0., 1. , 3.].iter().sum::<f64>());

        use std::cmp::Ordering;

        fn cmp(lhs: &&f64, rhs: &&f64) -> Ordering {
            match lhs.partial_cmp(rhs) {
                Some(x) => x,
                _ => Ordering::Less
            }
        }
        let ns = [1., 4., std::f64::NAN, 2.];
        assert_eq!(ns.iter().max_by(cmp), Some(&2.));
    }

}
