mod dlist {

// A difference list is a function that, given a list, returns the original contents of the
// difference list prepended to the given list.
//
// This structure supports /O(1)/ append and snoc operations on lists, making it
// very useful for append-heavy uses (esp. left-nested uses of append operations), such as logging
// and pretty printing.
//
    // pub struct DList<A> { unDL: &(dyn Fn(&[A]) -> &[A]) }

    pub fn id<A>(a: A) -> A {
        a
    }

    // pub fn empty<A>() -> DList<A> {
    //     DList { unDL: &id }
   //  }

}

#[cfg(test)]
mod tests {
    #[test]
    assert_eq!(super::id(1), 1);
}
