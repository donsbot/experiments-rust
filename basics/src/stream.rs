// 
// example of how to do stream fusion types in Rust
//

// Result of taking a single step in a stream
enum Step<'a,S,A> {
    Yield(&'a A, S),
    Skip(S),
    Done,
}

// data Stream a = forall s. Stream (s -> (Step s a)) s
struct Stream<'a, S, A> {
    next: Box<dyn Fn(S) -> Step<'a,S,A> + 'a>,
    state: S
}

// Check if a 'Stream' is empty
fn null<'a,A,S: Copy>(s: &Stream<'a,S,A>) -> bool {
    let mut st1 = s.state;
    loop {
        let r = (s.next)(st1);
        match r {
            Step::Yield(..) => { return false }
            Step::Skip(st2) => { st1 = st2 }
            Step::Done => { return true }
        }
    }
}

// The empty stream
fn empty<'a,A>() -> Stream<'a, (), A> {
    Stream {
        next: Box::new(|_| Step::Done),
        state: ()
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
        state: true
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
        state: &Left(l.state)
    }
    unimplemented!();

}
*/

// Yield a 'Stream' of values obtained by running the generator a given number of times
fn replicate<A>(n: usize, a: &A) -> Stream<usize,A> {

    let step = move |i:usize| {
        if i == 0 {
            Step::Done
        } else {
            Step::Yield(a, i-1)
        }
    };

    Stream {
        next: Box::new(step),
        state: n
    }
}

/*
replicateM :: Monad m => Int -> m a -> Stream m a
{-# INLINE_FUSED replicateM #-}
replicateM n p = Stream step n
  where
    {-# INLINE_INNER step #-}
    step i | i <= 0    = return Done
           | otherwise = do { x <- p; return $ Yield x (i-1) }
*/

// First element of the 'Stream' or None if empty
// head :: Monad m => Stream m a -> m a
fn head<'a,A,S: Copy>(s: &Stream<'a,S,A>) -> Option<&'a A> {
    let mut st1 = s.state;
    loop {
        let r = (s.next)(st1);
        match r {
            Step::Yield(x,_) => { return Some(x) }
            Step::Skip(st2) => { st1 = st2 }
            Step::Done => { return None } 
        }
    }
}

// Last element of the 'Stream' or None if empty
// last :: Monad m => Stream m a -> m a
fn last<'a,A,S: Copy>(s: &Stream<'a,S,A>) -> Option<&'a A> {
    let mut st1 = s.state;
    // we do this as two loops. one that iterates until we find at least one value
    // the other that then holds the most recent seen one, until it returns
    let mut result: &A;

    loop {
        let r = (s.next)(st1);
        match r {
            Step::Yield(x,st2) => { st1 = st2; result = x; break }
            Step::Skip(st2) => { st1 = st2 }
            Step::Done => { return None } 
        }
    }

    // r is definitely initialized now with a possible result
    loop {
        let r = (s.next)(st1);
        match r {
            Step::Yield(y,st2) => { st1 = st2; result = y; }
            Step::Skip(st2) => { st1 = st2 }
            Step::Done => { return Some(result); } 
        }
    }
}


/* basic tests */
pub fn main() {

    let s1: Stream<(),char> = empty();
    let s2: Stream<bool,i64> = singleton(&42);
    let s3: Stream<usize,i64> = replicate(10,&0);

    assert_eq!(true, null(&s1));
    assert_eq!(false, null(&s2));
    assert_eq!(true, null(&s1));
    assert_eq!(false, null(&s2));

    assert_ne!(head(&s2),None) ;
    assert_ne!(head(&s2),head(&s3));
    assert_eq!(head(&s2),last(&s2));
}
