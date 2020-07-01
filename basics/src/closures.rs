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

fn sort_s_pure(c: &[S]) -> Vec<S> {
    let mut d = c.to_owned(); // own a new d from borrowed c
    let x = 2; // to be captured
    d.sort_by_key(|c| {-c.b + x} );
    d
}
