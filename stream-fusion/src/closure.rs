//
// Example of how to do stream fusion iterators in Rust
// using pure functions/closure passing only
//
extern crate either;

// Result of taking a single step in a stream
pub enum Step<S, A> {
    Yield(A, S),
    Skip(S),
    Done,
}

// data Stream a = forall s. Stream (s -> (Step s a)) s
pub struct Stream<'s,  S: Seed, A: Copy> {
    next: Box<dyn Fn(S) -> Step<S, A> + 's>,
    seed: S
}

pub trait Seed: Copy {}
impl Seed for () {}
impl Seed for bool {}
impl Seed for usize {}
impl Seed for i64 {}
impl<S: Seed, T:Seed> Seed for either::Either<S,T> {}
impl<S: Seed, T:Seed> Seed for (S, T) {}

// Check if a 'Stream' is empty
pub fn is_empty_f<'s, A : Copy>(s: &Stream<'s, impl Seed, A>) -> bool {
    let mut st1 = s.seed;
    loop {
        let r = (s.next)(st1);
        match r {
            Step::Yield(..) => return false,
            Step::Skip(st2) => st1 = st2,
            Step::Done => return true,
        }
    }
}

// The empty stream
pub fn empty_f<'s, A: Copy>() -> Stream<'s, impl Seed, A> {
    Stream {
        next: Box::new(|_| Step::Done),
        seed: (),
    }
}

// A stream with a single element
pub fn singleton<'s, A: 's + Copy>(a: A) -> Stream<'s, impl Seed, A> {
    let step = move |b: bool| {
        if b {
            Step::Yield(a, false)
        } else {
            Step::Done
        }
    };
    Stream {
        next: Box::new(step),
        seed: true,
    }
}

// Concatenate two streams
pub fn append<'s, A: Copy + 's, S: Seed + 's, T: Seed + 's>(
    l: Stream<'s, S, A>,
    r: Stream<'s, T, A>,
) -> Stream<'s, impl Seed, A>
{
    let x = l.seed;
    let step = move |a: either::Either<S, T>| match a {
        either::Either::Left(sa) => {
            let v = (l.next)(sa);
            match v {
                Step::Yield(x, sa1) => Step::Yield(x, either::Either::Left(sa1)),
                Step::Skip(sa1) => Step::Skip(either::Either::Left(sa1)),
                Step::Done => Step::Skip(either::Either::Right(r.seed)),
            }
        }
        either::Either::Right(sb) => {
            let v = (r.next)(sb);
            match v {
                Step::Yield(x, sb1) => Step::Yield(x, either::Either::Right(sb1)),
                Step::Skip(sb1) => Step::Skip(either::Either::Right(sb1)),
                Step::Done => Step::Done,
            }
        }
    };

    Stream {
        next: Box::new(step),
        seed: either::Either::Left(x),
    }
}

// Yield a 'Stream' of values obtained by running the generator a given number of times
pub fn replicate<'s, A: 's + Copy>(n: usize, a: A) -> Stream<'s, impl Seed, A> {
    let step = move |i: usize| {
        if i == 0 {
            Step::Done
        } else {
            Step::Yield(a, i - 1)
        }
    };
    Stream {
        next: Box::new(step),
        seed: n,
    }
}

use num_traits::int::PrimInt;

// Yield a 'Stream' of values from A to B-1
pub fn range<'s, A>(a: A, b : A) -> Stream<'s, impl Seed, A>
    where A: 's + Seed + Copy + std::ops::Add<Output = A> + PrimInt
{
    let step = move |i: A| {
        if i >= b {
            Step::Done
        } else {
            Step::Yield(i, i + A::one())
        }
    };
    Stream {
        next: Box::new(step),
        seed: a,
    }
}

// Left fold with a accumulator and an operator
pub fn foldl<'s, A: 's + Copy, B: 's + Copy>(
    f: fn(B, A) -> B,
    w: B,
    s: &Stream<'s, impl Seed, A>,
) -> B {
    let mut st = s.seed;
    let mut z = w;
    loop {
        let r = (s.next)(st);
        match r {
            Step::Yield(x, s1) => {
                z = f(z, x);
                st = s1
            }
            Step::Skip(s1) => st = s1,
            Step::Done => return z,
        }
    }
}

// Length of a stream
pub fn length<A: Copy>(s: &Stream<impl Seed, A>) -> usize {
    foldl(|n, _| n + 1, 0, s)
}

// Map a function over a 'Stream'
pub fn map<'s, A: 's + Copy, B: 's + Copy>(
    f: fn(A) -> B,
    s: Stream<'s, impl Seed +'s, A>,
) -> Stream<'s, impl Seed, B> {
    let x = s.seed;
    let step = move |st| {
        let r = (s.next)(st);
        match r {
            Step::Yield(x, st1) => {
                let y = f(x);
                Step::Yield(y, st1)
            }
            Step::Skip(st1) => Step::Skip(st1),
            Step::Done => Step::Done,
        }
    };
    Stream {
        next: Box::new(step),
        seed: x,
    }
}

// Filter a 'Stream' with a predicate
pub fn filter<'s, A: 's + Copy>(
    f: fn(&A) -> bool,
    s: Stream<'s, impl Seed +'s, A>,
) -> Stream<'s, impl Seed, A> {
    let x = s.seed;
    let step = move |st| {
        let r = (s.next)(st);
        match r {
            Step::Yield(x, st1) => {
                if f(&x) {
                    Step::Yield(x, st1)
                } else {
                    Step::Skip(st1)
                }
            }
            Step::Skip(st1) => Step::Skip(st1),
            Step::Done => Step::Done,
        }
    };
    Stream {
        next: Box::new(step),
        seed: x,
    }
}

// First element of the 'Stream' or None if empty
pub fn head<'s, A: 's + Copy>(s: &Stream<'s, impl Seed, A>) -> Option<A> {
    let mut st1 = s.seed;
    loop {
        let r = (s.next)(st1);
        match r {
            Step::Yield(x, _) => return Some(x),
            Step::Skip(st2) => st1 = st2,
            Step::Done => return None,
        }
    }
}

// Last element of the 'Stream' or None if empty
pub fn last<'s, A: 's + Copy>(s: &Stream<'s, impl Seed, A>) -> Option<A> {
    let mut st1 = s.seed;
    // we do this as two loops. one that iterates until we find at least one value
    // the other that then holds the most recent seen one, until it returns
    let mut result: A;

    loop {
        let r = (s.next)(st1);
        match r {
            Step::Yield(x, st2) => {
                st1 = st2;
                result = x;
                break;
            }
            Step::Skip(st2) => st1 = st2,
            Step::Done => return None,
        }
    }
    // r is definitely initialized now with a possible result
    loop {
        let r = (s.next)(st1);
        match r {
            Step::Yield(y, st2) => {
                st1 = st2;
                result = y;
            }
            Step::Skip(st2) => st1 = st2,
            Step::Done => {
                return Some(result);
            }
        }
    }
}

// The first @n@ elements of a stream
pub fn take<'s, A: Copy>(n: usize, s: &'s Stream<'s, impl Seed, A>) -> Stream<'s, impl Seed, A> {
    let step1 = move |(s0, i)| {
        if i < n {
            let r = (s.next)(s0); // run the first stream
            match r {
                Step::Yield(x, s1) => Step::Yield(x, (s1, i + 1)),
                Step::Skip(s1) => Step::Skip((s1, i)),
                Step::Done => Step::Done,
            }
        } else {
            Step::Done
        }
    };
    Stream {
        next: Box::new(step1),
        seed: (s.seed, 0),
    }
}

/*
 * need trait or hiding for state
 */
pub fn cons<'s, A: 's + Copy>(
    a: A,
    s: Stream<'s, impl Seed + 's, A>,
) -> Stream<'s, impl Seed, A> {
    let s1 = singleton(a);
    append(s1, s) // consumes
}

pub fn basic_bench(n: usize) -> i64 {
    let s1 = range(0, n as i64);
    let s2 = filter(|n| n % 2 == 1, s1);
    let s3 = map(|n| n * 2, s2);
    foldl(|n, x| n + x, 0, &s3)
}

/* basic tests */
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty_m() {
        let s1: Stream<_, i64> = empty_f();
        assert_eq!(true, is_empty_f(&s1));
    }

    /* functional calls */

    #[test]
    fn test_empty() {
        let s1: Stream<_, i64> = empty_f();
        assert_eq!(true, is_empty_f(&s1));
    }

    #[test]
    fn test_singleton() {
        let s1 = singleton(42i64);
        assert_eq!(false, is_empty_f(&s1));
    }

    #[test]
    fn test_append() {
        let s1 = singleton(42i64);
        let s2 = singleton(64i64);
        let s3 = append(s1, s2);

        assert_eq!(false, is_empty_f(&s3));
    }

    #[test]
    fn test_replicate() {
        let s1 = replicate(10, 42i64);
        assert_eq!(false, is_empty_f(&s1));
    }

    #[test]
    fn test_length() {
        let l = 10;
        let s1 = replicate(l, 42i64);
        let s2 = replicate(l, 42i64);
        let s3 = append(s1, s2);
        assert_eq!(length(&s3), 2 * l);
    }

    #[test]
    fn test_map() {
        let s1 = replicate(10, 42i64);
        let s2 = map(|x| x + 1, s1);
        let v = foldl(|n, i| n + i, 0, &s2);
        assert_eq!(v, 43 * 10)
    }

    #[test]
    fn test_head() {
        let s1 = replicate(10, 42i64);
        assert_eq!(Some(42), head(&s1));
        let s1: Stream<_, i64> = empty_f();
        assert_eq!(None, head(&s1));
    }

    #[test]
    fn test_last() {
        let s1 = replicate(10, 42i64);
        assert_eq!(Some(42), last(&s1));
        let s1: Stream<_, i64> = empty_f();
        assert_eq!(None, last(&s1));
    }

    #[test]
    fn test_take() {
        let s1 = replicate(10, 42i64);
        assert_eq!(2, length(&take(2, &s1)));
        let s1: Stream<_, i64> = empty_f();
        assert_eq!(0, length(&take(10, &s1)));
    }

    #[test]
    fn test_cons() {
        let s2 = cons(3, cons(4, cons(6, empty_f())));
        assert_eq!(3, length(&s2));
    }

    #[test]
    fn test_perf_lazy() { // should be fast
        let lim = 10000000;
        let s1 = replicate(lim, 0);
        assert_eq!(1, length(&take(1,&s1)));
    }
}
