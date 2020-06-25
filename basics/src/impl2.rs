pub struct Q<T> {
    left: Vec<T>,
    right: Vec<T>
}

// OO style
impl<T> Q<T> {
    pub fn new() -> Q<T> {
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

}
