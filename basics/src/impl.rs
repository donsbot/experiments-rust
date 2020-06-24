// page 198: impls
#[derive(Debug)]
pub struct Q {
    old: Vec<char>,
    new: Vec<char>,
}

impl Q {
    pub fn push(&mut self, c: char) {
        self.new.push(c);
    }

    pub fn is_empty(&self) -> bool {
        self.old.is_empty() && self.new.is_empty()
    }

    pub fn pop(&mut self) -> Option<char> {
        use std::mem::swap;

        match (self.old.is_empty(), self.new.is_empty()) {
            (true, true) => None,

            (true, _) => {
                swap(&mut self.old, &mut self.new);
                self.old.reverse();
                self.old.pop()
            }

            (false, _) => self.old.pop(),
        }
    }
}

pub fn main() {
    let mut q = Q {
        old: Vec::new(),
        new: Vec::new(),
    };
    q.push('a');
    q.push('b');
    println!("{:?}", q);
    let c = q.pop();
    println!("{:?}", c);
    let c = q.pop();
    println!("{:?}", c);
    let c = q.pop();
    println!("{:?}", c);
}
