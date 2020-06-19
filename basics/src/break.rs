fn main() {
    // 1. can't borrow a reference to a local variable and take it out of scope
    {
        let r: &i64 = &1; // empty ref binding
                          // assert_eq!(*r, 1); // uniitialized *r.
        {
            let _x = 2;
            // NO: r = &x; // borrowed value doesn't live long enough
            // ref to x can't outlive x
        } // dropped here
        assert_eq!(*r, 1);
    }

    // 2. a valid example
    {
        let x = 1;
        {
            let r = &x; // inner life time is subset of lifetime of x
            assert_eq!(*r, 1);
        }
    }

    // 3. borrwing a ref to a data structure
    {
        let v = vec![1, 2, 3];
        {
            let r = &v[1];
            assert_eq!(*r, 2);
        }
    }

    // 4. references in parameters
    {
        // threadsafety. this is a top-level IORef.
        // need to treat like an MVar
        // mutable static is pretty unsafe
        static mut STASH: &i32 = &128; // needs to be initialized
        fn f(p: &'static i32) {
            // tick A
            // ignore the threadsafety explicitly
            unsafe {
                STASH = p; // but lifetime of p
            }
        }
        f(&17);
        // lifetime genericity , most specific lifetime etc
    }
}
