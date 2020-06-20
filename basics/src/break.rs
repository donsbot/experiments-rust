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
                                       // f can only applied to things with static lifetime
        fn f(p: &'static i32) {
            // tick A
            // ignore the threadsafety explicitly
            unsafe {
                STASH = p; // but lifetime of p
            }
        }
        f(&17);
        // lifetime genericity , most specific lifetime etc
        // not ok: let WORTH_POINTING_AT: i32 = 1000;
        static WORTH_POINTING_AT: i32 = 1000;
        // also ok: const WORTH_POINTING_AT: i32 = 1000;
        f(&WORTH_POINTING_AT);
    }

    // 5. passing references as arguments
    {
        // polymorphic in lifetime
        fn g<'a>(_p: &'a i32) {}
        fn h<'a>(_p: &'static i32) {}

        let x = 10;
        g(&x);
        const UY: i32 = 10;
        h(&UY);
    }

    // 6. slices
    {
        // with a single ref in and return a single ref, assume same lifetime
        // fn smallest(v: &[i32]) -> &i32 {
        fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
            let mut s = &v[0];
            for r in &v[1..] {
                // iterate by ref
                if *r < *s {
                    s = r;
                }
            }
            s
        }

        {
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            let s = smallest(&parabola);
            assert_eq!(*s, 0);
        }
    }

    // 7. structs with references
    // you must pass a lifetime param through the struct to any references contained
    {
        struct S<'a> {
            r: &'a i32,
        }

        let x = 10;
        let s = S { r: &x }; // some lifetime 'a, which must be within x's lifetime
        assert_eq!(*s.r, 10);
    }
}
