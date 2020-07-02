//
// Example of how to do stream fusion iterators in Rust
//
// Todo: put in impl, use good syntax
// Hide the seed parameter
//

// Result of taking a single step in a stream
enum Step<'a, S, A> {
    Yield(&'a A, S),
    Skip(S), // unboxed seeds only please
    Done,
}

// data Stream a = forall s. Stream (s -> (Step s a)) s
struct Stream<'a, S, A> {
    next: Box<dyn Fn(S) -> Step<'a, S, A> + 'a>,
    seed: S,
}

// alternative, implement as a trait
trait Stream1<A> {
    type S;
    fn next1<'a>(sd: Self::S) -> Step<'a, Self::S, A>;
    fn seed1() -> Self::S;
}

// Check if a 'Stream' is empty
fn null<'a, A, S: Copy>(s: &Stream<'a, S, A>) -> bool {
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
fn empty<'a, A>() -> Stream<'a, (), A> {
    Stream {
        next: Box::new(|_| Step::Done),
        seed: (),
    }
}

// A stream with a single element
fn singleton<A>(a: &A) -> Stream<bool, A> {
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

/*
enum Either<A,B> {
    Left(A),
    Right(B)
}

use Either::*;
*/

// Concatenate two streams
// (++) :: Monad m => Stream m a -> Stream m a -> Stream m a
//
/*
fn append<'a, S, T, A>
    ( l : &Stream<'a,S,A>
    , r : &Stream<'a,T,A>) -> Stream<'a, Either<S,T>, A>
{

    let step = move |a: Either<S,T>| {
        match a {
            Left(sa) => {
    //            let r = (l.f)(sa);
                unimplemented!();
            }

            Right(sb) => {
    //            let r = (r.f)(sb);
                unimplemented!()
            }
        }
    };

    Stream {
        next: Box::new(step),
        seed: &Left(l.seed)
    }
    unimplemented!();

}
*/

// Yield a 'Stream' of values obtained by running the generator a given number of times
fn replicate<A>(n: usize, a: &A) -> Stream<usize, A> {
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

// First element of the 'Stream' or None if empty
// head :: Monad m => Stream m a -> m a
fn head<'a, A, S: Copy>(s: &Stream<'a, S, A>) -> Option<&'a A> {
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
// last :: Monad m => Stream m a -> m a
fn last<'a, A, S: Copy>(s: &Stream<'a, S, A>) -> Option<&'a A> {
    let mut st1 = s.seed;
    // we do this as two loops. one that iterates until we find at least one value
    // the other that then holds the most recent seen one, until it returns
    let mut result: &A;

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
// take :: Monad m => Int -> Stream m a -> Stream m a
fn take<'a, A, S: Copy>(n: usize, s: &'a Stream<S, A>) -> Stream<'a, (S, usize), A> {
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

/* basic tests */
pub fn main() {
    let s1: Stream<(), char> = empty();
    let s2: Stream<bool, i64> = singleton(&42);
    let s3: Stream<usize, i64> = replicate(10, &0);
    let s4: Stream<usize, i64> = replicate(3, &1);
    let s5 = take(0, &s4);
    assert_eq!(true, null(&s5));
    assert_eq!(false, null(&take(2, &s4)));

    assert_eq!(true, null(&s1));
    assert_eq!(false, null(&s2));
    assert_eq!(true, null(&s1));
    assert_eq!(false, null(&s2));

    assert_ne!(head(&s2), None);
    assert_ne!(head(&s2), head(&s3));
    assert_eq!(head(&s2), last(&s2));
}
