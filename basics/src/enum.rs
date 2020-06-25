// Chapter 10: Enums. page 212

use std::cmp::Ordering;
use std::mem::size_of;

// n.b. #[repr]!!

#[derive(Debug,PartialEq)]
enum O {
    LT_ = -1,
    EQ_ = 0,
    GT_ = 1
}

// import the constructors
use self::O::*;

impl O {

    fn i32_to_maybe_o(n: i32) -> Option<O> {
        match n {
            -1 => Some(LT_),
            0 => Some(EQ_),
            1 => Some(GT_),
            _ => None
        }
    }

    fn compare_(n: i32, m: i32) -> O {
        match n.cmp(&m) /*urgh*/ {
            Ordering::Greater => GT_,
            Ordering::Less => LT_,
            Ordering::Equal => EQ_,
        }

        /*
        if n < m {
            LT_
        } else if n > m {
            GT_
        } else {
            EQ_
        }
        */
        
    }

}

#[derive(Debug,PartialEq)]
enum A {
    Z,
    X,
    F(Option<String>),
    G {
        left: u32,
        right: u32
    }
}

pub fn main() {
    println!("{:?}", O::compare_(7,32));

    // assertions about size of objects
    assert_eq!(size_of::<O>(), 1);
    assert_eq!(LT_ as i32, -1);

    assert_eq!(O::i32_to_maybe_o(42), None);
    assert_eq!(O::i32_to_maybe_o(1), Some(GT_));

    assert_ne!(A::Z,A::X);
    assert_ne!(A::F(None),A::G { left: 0, right: 1})

}
