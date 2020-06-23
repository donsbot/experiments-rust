// rebinding a path
// extern crate std as prelude;

// extern crate std::prelude::v1;

// items
pub mod m {

    pub fn mk_e_v() -> E<u64> {
        E::EV(V { v: 7 })
    }

    #[derive(Debug)]
    pub struct V<T> {
        v: T,
    }
    #[derive(Debug)]
    pub struct U<T> {
        u: T,
    }

    #[derive(Debug)]
    pub enum E<T> {
        EV(V<T>),
        _EU(U<T>),
    }

    #[derive(Debug)]
    pub enum Tree<'a, T> {
        Node {
            left: &'a Tree<'a, T>,
            right: &'a Tree<'a, T>,
            value: T, // unboxed
        },
        Empty,
    }

    // constructors
    pub fn mk_one_node<'a, T>(v: T) -> Tree<'a, T> {
        Tree::Node {
            left: &Tree::Empty,
            right: &Tree::Empty,
            value: v,
        }
    }
}

pub fn main() {
    let x: m::E<u64> = m::mk_e_v();
    println!("{:?}", x);

    use m::Tree;

    // Empty
    let zero: Tree<u64> = Tree::Empty;

    // Node { left: Empty, right: Empty, value: 1 }
    let one: Tree<u64> = m::mk_one_node(1);

    let two: Tree<u64> = Tree::Node {
        left: &one,
        right: &one,
        value: 1,
    };

    println!("{:?}", zero);
    println!("{:?}", one);
    println!("{:?}", two);
}
