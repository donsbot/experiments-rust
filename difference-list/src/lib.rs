///
/// This structure supports /O(1)/ append and snoc operations on lists, making it
/// very useful for append-heavy uses (esp. left-nested uses of append operations), such as logging
/// and pretty printing.
///
pub mod dlist {

    use std::marker::PhantomData;

    /// The type of difference lists of A
    pub struct DList<A> {
        phantom: PhantomData<A>,
    }

// pub struct DList<A> { unDL: &(dyn Fn(&[A]) -> &[A]) }

    pub fn empty<A>() -> DList<A> {
        DList{phantom: PhantomData}
    }

    #[allow(dead_code)]
    fn id<A>(a: A) -> A {
        a
    }

}
