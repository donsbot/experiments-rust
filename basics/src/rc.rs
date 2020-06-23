// Rc and Arc
//
// Rc: Refefence Count
// Arc: Atomic Reference Count
//
use std::rc::Rc;

pub fn main() {
    // construct a reference counted pointer
    let s: Rc<String> = Rc::new("foo".to_string());
    let _t: Rc<String> = s.clone();
    let _u: Rc<String> = s.clone();

    // 3 stack allocated pointers to a heap allocated ref-count box + string ref (unboxed) 
    // all refer to the same block
    // can't be shared and mutable
    // s.push_str(" noodles");




}
