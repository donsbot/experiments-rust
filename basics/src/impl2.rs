pub struct Q<T> {
    left: Vec<T>,
    right: Vec<T>
}

// holds references with the lifetime 'e
#[derive(Debug)]
struct E<'e> {
    biggest: &'e i32,
    smallest: &'e i32
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct P<T> {
    x: T,
    y: T 
}

fn find_e<'s>(slice: &'s [i32]) -> E/* lifetime omitted */ {
    let mut biggest: &'s i32 = &slice[0];
    let mut smallest: &'s i32 = &slice[0];
    for i in 1 .. slice.len() {
        let x = slice[i];
        if x < *smallest { smallest = &slice[i]; }
        if x > *biggest  { biggest  = &slice[i]; }
    }
    E { biggest, smallest } 
}

// OO style
impl<T> Q<T> {
    pub fn new() -> Self /* interesting */ {
        Q { left: Vec::new(), right: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.right.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.right.is_empty() && self.left.is_empty()
    }
}

// free functions style
pub fn new_q<T>() -> Q<T> {
    Q::new()
}

pub fn push_q<T>(q: &mut Q<T>, t: T) {
    Q::push(q, t);
}

pub fn is_empty_q<T>(q: &Q<T>) -> bool {
    Q::is_empty(q)
}

// interesting bias towards mutable state
pub fn main() {
    let mut q: Q<u64> = Q::new();
    let r: Q<u64> = new_q();
    assert_eq!(is_empty_q(&r), true);
    assert_eq!(is_empty_q(&q), true);
    assert_eq!(q.is_empty(), true);
    push_q(&mut q,42);
    q.push(42);
    assert_eq!(!q.is_empty(), true);

    let a = [0, -3, 2, 1, 15, 100, 98];
    let e = find_e(&a);
    println!("{:?}", e);

    let a = P { x: 0., y: 0.};
    let b = a.clone();
    println!("{:?}", a);
    if a == b {
        println!("equal");
    }

}
