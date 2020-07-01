// pg 303 / ch14 closures

pub fn main() {
    let a = vec![S{a:"foo".to_string(),b:7,c:"c".to_string()}
                ,S{a:"foo".to_string(),b:6,c:"x".to_string()}
                ,S{a:"bar".to_string(),b:700,c:"y".to_string()}
    ];
    let b = sort_s_pure(&a);
    println!("{:?}", a);
    println!("{:?}", b);
}

#[derive(Clone,PartialEq,Debug,Eq,Ord,PartialOrd)]
struct S { 
    a: String,
    b: i64,
    c: String
}

fn b_descending(c: &S) -> i64 {
    -c.b
}

fn sort_s_pure(c: &Vec<S>) -> Vec<S> {
    let mut d = c.clone();
    d.sort_by_key(b_descending);
    d
}
