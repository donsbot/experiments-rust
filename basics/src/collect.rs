// references let you access a value without changing its ownership
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

// this is like an ownership destructor. the function param takes ownership of the table, and then
// destorys the ks and values.
//
// so instead we pass a ref T , to access without taking ownership of the lifetime
fn show(t: &Table) { // ref to Table
    for (k,vs) in t { // n.b. iterating over a shared ref produces shared refs to components
        println!("key : {}:", k);
        for v in vs {
            println!(" {}", v);
        }
    }
}

fn sort_works(t: &mut Table) {
    for (_,vs) in t {
        vs.sort();
    }
}

fn main() {
    let mut t = Table::new();

    t.insert("a".to_string(),
            vec!["x".to_string() , "y".to_string(), "z".to_string()]);
    t.insert("b".to_string(),
            vec!["2".to_string() , "3".to_string(), "1".to_string()]);

    show(&t); // create a shared ref T (read-only is fine)
    sort_works(&mut t); // mutate the table
    show(&t); // create a shared ref T (read-only is fine)

    ref_1();
    ref_2();
}

fn ref_1() {
    let x:i64 = 10;
    let r:&i64 = &x;
    assert!(r == &10);
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y; //mut ref to y
    *m += 32;
    assert!(*m == 64);
}

// since references are so widely used  the '.' operator on methods implicitly dereferences its
// left operand
fn ref_2() {
    struct A { name: &'static str, _bechdel_pass: bool }
    let aria = A { name: "Aria: The Animation", _bechdel_pass: true };
    let a_ref = &aria;
    assert_eq!(a_ref.name, "Aria: The Animation");
    assert_eq!((*a_ref).name, "Aria: The Animation");
}
