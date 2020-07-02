//
// Example of how to do stream fusion iterators in Rust
//
// Todo: put in impl, use good syntax
// Hide the seed parameter
//

extern crate either;
use either::Either::*;
use either::Either;

// Result of taking a single step in a stream
enum Step<'a, S, A> {
    Yield(&'a A, S),
    Skip(S), // unboxed seeds only please
    Done,
}

// data Stream a = forall s. Stream (s -> (Step s a)) s
pub struct Stream<'s, 'a, S, A> {
    next: Box<dyn Fn(S) -> Step<'a, S, A> + 's>,
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
pub fn null<'s, 'a, A, S: Copy>(s: &Stream<'s, 'a, S, A>) -> bool {
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
pub fn empty<'s, 'a, A>() -> Stream<'s, 'a, (), A> {
    Stream {
        next: Box::new(|_| Step::Done),
        seed: (),
    }
}

// A stream with a single element
pub fn singleton<A>(a: &A) -> Stream<bool, A> {
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
// (++) :: Monad m => Stream m a -> Stream m a -> Stream m a
//
pub fn append<'s, 'a, S:Copy + 's, T:Copy, A>
    ( l : Stream<'s,'a,S,A>
    , r : &'s Stream<'s,'a,T,A>) -> Stream<'s, 'a, Either<S,T>, A>
{
    let x = l.seed.clone();
    let step = move |a: Either<S,T>| {
        match a {
            Left(sa) => {
                let v = (l.next)(sa);
                match v {
                    Step::Yield(x,sa1) => { Step::Yield(x,Left(sa1))}
                    Step::Skip(sa1)    => { Step::Skip(Left(sa1)) }
                    Step::Done         => { Step::Skip(Right(r.seed))}
                }
            }
            Right(sb) => {
                let v = (r.next)(sb);
                match v {
                    Step::Yield(x,sb1) => { Step::Yield(x,Right(sb1))}
                    Step::Skip(sb1)    => { Step::Skip(Right(sb1)) }
                    Step::Done         => { Step::Done }
                }
            }
        }
    };

    Stream {
        next: Box::new(step),
        seed: Left(x)
    }
}

// Yield a 'Stream' of values obtained by running the generator a given number of times
pub fn replicate<A>(n: usize, a: &A) -> Stream<usize, A> {
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
pub fn head<'s, 'a, A, S: Copy>(s: &Stream<'s, 'a, S, A>) -> Option<&'a A> {
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
pub fn last<'s, 'a, A, S: Copy>(s: &Stream<'s, 'a, S, A>) -> Option<&'a A> {
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
pub fn take<'s, 'a, A, S: Copy>(n: usize, s: &'s Stream<'s, 'a, S, A>) ->
    Stream<'s, 'a, (S, usize), A>
{
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
pub fn cons<'s,'a,A,S:Copy>
    ( a: &'a A
    , s: &'s Stream<'s, 'a, S, A>) -> Stream<'s, 'a, Either<bool,S>, A>
{
    let s1 = singleton(a);
    append(s1, s) // consumes
}

pub fn length<S:Copy,A>(s : &Stream<S, A>) -> usize {
    foldl(|n,_| { n+1 } , 0, s)
}

// Left fold with a strict accumulator and a monadic operator
// foldlM' :: Monad m => (a -> b -> m a) -> a -> Stream m b -> m a
//
pub fn foldl<'s, 'a, B, A, S: Copy>
            (f: fn(B,&A) -> B,
             w: B,
             s: &Stream<'s, 'a, S, A> 
             ) -> B
{
    let mut st = s.seed;
    let mut z  = w;
    loop {
        let r = (s.next)(st);
        match r { 
            Step::Yield(x,s1) => { z = f(z,x); st = s1 } 
            Step::Skip(s1) => { st = s1 }
            Step::Done => { return z }
        }
    }
}


/* basic tests */
mod tests {

    #[test]
    fn test_cons() {
        let s1 = super::replicate(10, &'x');
        let s2 = super::cons(&'x', &s1);
        assert_eq!(11,super::length(&s2));
    }

    #[test]
    fn test_length() {
        let s1 = super::replicate(10, &'x');
        let s2 = super::replicate(10, &'x');
        assert_eq!(20,super::length(&super::append(s1,&s2)));
    }

    #[test]
    fn test_foldl() {
        let s1 = super::replicate(10, &'x');
        assert_eq!(10,super::foldl( |b,_| { b + 1 } 
                               , 0usize
                               , &s1));
    }

    #[test]
    fn test_append() {
        let s1 = super::empty();
        let s2 = super::replicate(10, &'x');
        let s3 = super::append(s1,&s2);
        assert_eq!(super::null(&s3), false);
    }


    #[test]
    fn test_0() {
        let s1: super::Stream<_,i64> = super::empty();
        let s2 = super::singleton(&42);
        let s3 = super::replicate(10, &0);
        let s4 = super::replicate(3, &1);
        let s5 = super::take(0, &s4);
        assert_eq!(true, super::null(&s5));
        assert_eq!(false, super::null(&super::take(2, &s4)));

        assert_eq!(true, super::null(&s1));
        assert_eq!(false, super::null(&s2));
        assert_eq!(true, super::null(&s1));
        assert_eq!(false, super::null(&s2));

        assert_ne!(super::head(&s2), None);
        assert_ne!(super::head(&s2), super::head(&s3));
        assert_eq!(super::head(&s2), super::last(&s2));
    }
}
