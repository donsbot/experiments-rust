// ch 16: pg 374: vecdeques
//
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::BTreeSet;

pub fn main() {
    // introduction
    {
        let mut x: VecDeque<i64> = VecDeque::new();
        x.push_front(4);
        x.push_back(3);
        x.push_back(2);
        for i in x {
            println!("{}",i );
        }
    }

    // binary heaps
    {
        let mut x: BinaryHeap<i64> = BinaryHeap::new();
        x.push(7);
        x.push(3);
        x.push(1);
        println!("{}", x.pop().unwrap());
    }

    // hashmaps and btrees
    {
        let mut h : HashMap<String,i64> = vec![("foo".to_string(), 7i64)
                                      ,("bar".to_string(), 8)
                                      ].into_iter().collect();
        let mut b : BTreeMap<String,i64> = vec![("foo".to_string(), 7i64)
                                      ,("bar".to_string(), 8)
                                      ].into_iter().collect();

        assert_eq!(h.is_empty(), false);
        assert_eq!(b.is_empty(), false);

        assert_eq!(h.contains_key("foo"), true );
        assert_eq!(b.contains_key("foo"), true );

        println!("{}", b.get("foo").unwrap()
                     + h.get("foo").unwrap());

        assert_eq!(b.insert("foo".to_string(),7), Some(7));
        assert_eq!(h.insert("foo".to_string(),7), Some(7));

        /*
        let mut i = HashMap::new();
        i.append(&h);
        assert_eq!(h.is_empty(), true);
        assert_eq!(i.is_empty(), false);
        */

    }

    // hashsets / btreesets
    {
        // membership
        let h : HashSet<String> = vec!["foo".to_string() ,"bar".to_string()].into_iter().collect();
        let b : BTreeSet<String> = vec!["foo".to_string() ,"bar".to_string()].into_iter().collect();
        let b2 : BTreeSet<String> = vec!["baz".to_string() ,"bar".to_string()].into_iter().collect();

        assert_eq!(h.contains("foo"), true );
        assert_eq!(b.contains("foo"), true );

        // "bar" is the intersection. shared ref to string in set.  fast(!)
        let v : Vec<&String> = b.intersection(&b2).collect();
        println!("{:?}", v);

    }

    // hash types : derivation
    {
        #[derive(PartialEq,Hash,Eq)]
        struct A {
            a: String,
            b: i64
        }
        let h : HashSet<A> = vec![A{ a : "foo".to_string(), b: 32 }].into_iter().collect();
        assert_eq!(h.contains(&A { a : "foo".to_string(), b: 33 } ), false );
        assert_eq!(h.contains(&A { a : "foo".to_string(), b: 32 } ), true );

    }
}
