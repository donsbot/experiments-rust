//
// Example of how to do stream fusion iterators in Rust
//
// Todo: put in impl, use good syntax
// Hide the seed parameter
//

extern crate either;
use either::Either;
use either::Either::*;

// Result of taking a single step in a stream
enum Step<S, A> {
    Yield(A, S),
    Skip(S), // unboxed seeds only please
    Done,
}

// data Stream a = forall s. Stream (s -> (Step s a)) s
pub struct Stream<'s, S, A> {
    next: Box<dyn Fn(S) -> Step<S, A> + 's>,
    seed: S,
}

/*
// alternative, implement as a trait
trait Stream1<A> {
    type Seed;
    fn next1<'a>(sd: Self::Seed) -> Step<'a, Self::Seed, A>;
    fn seed1() -> Self::Seed;
}
*/

// Check if a 'Stream' is empty
pub fn null<'s, A, S: Copy>(s: &Stream<'s, S, A>) -> bool {
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
pub fn empty<'s, A>() -> Stream<'s, (), A> {
    Stream {
        next: Box::new(|_| Step::Done),
        seed: (),
    }
}

// A stream with a single element
pub fn singleton<'s, A: 's + Copy>(a: A) -> Stream<'s, bool, A> {
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
pub fn append<'s, S: Copy + 's, T: Copy + 's, A: Copy + 's>(
    l: Stream<'s, S, A>,
    r: Stream<'s, T, A>,
) -> Stream<'s, Either<S, T>, A> {
    let x = l.seed;
    let step = move |a: Either<S, T>| match a {
        Left(sa) => {
            let v = (l.next)(sa);
            match v {
                Step::Yield(x, sa1) => Step::Yield(x, Left(sa1)),
                Step::Skip(sa1) => Step::Skip(Left(sa1)),
                Step::Done => Step::Skip(Right(r.seed)),
            }
        }
        Right(sb) => {
            let v = (r.next)(sb);
            match v {
                Step::Yield(x, sb1) => Step::Yield(x, Right(sb1)),
                Step::Skip(sb1) => Step::Skip(Right(sb1)),
                Step::Done => Step::Done,
            }
        }
    };

    Stream {
        next: Box::new(step),
        seed: Left(x),
    }
}

// Yield a 'Stream' of values obtained by running the generator a given number of times
pub fn replicate<'s, A: 's + Copy>(n: usize, a: A) -> Stream<'s, usize, A> {
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

// Left fold with a accumulator and an operator
pub fn foldl<'s, A: 's + Copy, B: 's + Copy, S: Copy>(
    f: fn(B, A) -> B,
    w: B,
    s: &Stream<'s, S, A>,
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
pub fn length<S: Copy, A: Copy>(s: &Stream<S, A>) -> usize {
    foldl(|n, _| n + 1, 0, s)
}

// Map a function over a 'Stream'
pub fn map<'s, A: 's + Copy, B: 's + Copy, S: 's + Copy>(
    f: fn(A) -> B,
    s: Stream<'s, S, A>,
) -> Stream<'s, S, B> {
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

// First element of the 'Stream' or None if empty
pub fn head<'s, A: 's + Copy, S: 's + Copy>(s: &Stream<'s, S, A>) -> Option<A> {
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
pub fn last<'s, A: 's + Copy, S: 's + Copy>(s: &Stream<'s, S, A>) -> Option<A> {
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
pub fn take<'s, A, S: Copy>(n: usize, s: &'s Stream<'s, S, A>) -> Stream<'s, (S, usize), A> {
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
pub fn cons<'s, A: 's + Copy, S: 's + Copy>(
    a: A,
    s: Stream<'s, S, A>,
) -> Stream<'s, Either<bool, S>, A> {
    let s1 = singleton(a);
    append(s1, s) // consumes
}

/* basic tests */
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty() {
        let s1: Stream<_, i64> = empty();
        assert_eq!(true, null(&s1));
    }

    #[test]
    fn test_singleton() {
        let s1 = singleton(42i64);
        assert_eq!(false, null(&s1));
    }

    #[test]
    fn test_append() {
        let s1 = singleton(42i64);
        let s2 = singleton(64i64);
        let s3 = append(s1, s2);

        assert_eq!(false, null(&s3));
    }

    #[test]
    fn test_replicate() {
        let s1 = replicate(10, 42i64);
        assert_eq!(false, null(&s1));
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
        let s1: Stream<_, i64> = empty();
        assert_eq!(None, head(&s1));
    }

    #[test]
    fn test_last() {
        let s1 = replicate(10, 42i64);
        assert_eq!(Some(42), last(&s1));
        let s1: Stream<_, i64> = empty();
        assert_eq!(None, last(&s1));
    }

    #[test]
    fn test_take() {
        let s1 = replicate(10, 42i64);
        assert_eq!(2, length(&take(2, &s1)));
        let s1: Stream<_, i64> = empty();
        assert_eq!(0, length(&take(10, &s1)));
    }

    #[test]
    fn test_cons() {
        let s2 = cons(3, cons(4, cons(6, empty())));
        assert_eq!(3, length(&s2));
    }
}
