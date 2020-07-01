// 
// example of how to do stream fusion types in Rust
//

// Result of taking a single step in a stream
enum Step<'a,S,A> {
    Yield(&'a A, &'a S),
    Skip(&'a S),
    Done,
}

// data Stream a = forall s. Stream (s -> (Step s a)) s
struct Stream<'a, S, A> {
    f: Box<dyn Fn(&S) -> Step<'a,S,A> + 'a>,
    st: &'a S
}

// Check if a 'Stream' is empty
fn null<'a,A,S>(s: &Stream<'a,S,A>) -> bool {
    let mut st1 = s.st;
    loop {
        let r = (s.f)(&st1);
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
        f: Box::new(|_| Step::Done),
        st: &()
    }
}

// A stream with a single element
fn singleton<A>(a: &A) -> Stream<bool, A> {

    let step = move |b: &bool| {
        if *b {
            Step::Yield(a, &false)
        } else {
            Step::Done
        }
    };

    Stream {
        f: Box::new(step),
        st: &true
    }
}

enum Either<A,B> {
    Left(A),
    Right(B)
}

use Either::*;

/*
// Concatenate two streams
// (++) :: Monad m => Stream m a -> Stream m a -> Stream m a
fn append<'a,S: 'a ,T: 'a,A>(l: Stream<'a, S, A>
                  , r: Stream<'a, T, A>) -> Stream<'a, Either<S,T>, A>
{
    let step = move |a: Either<S,T>| {
        match a {
            Left(sa) => {
                let r = (l.f)(sa);
                unimplemented!();
            }

            Right(sb) => {
                let r = (r.f)(sb);
                unimplemented!()
            }
        }
    };

    Stream {
        f: Box::new(step),
        st: Left(l.st)
    }

}
*/

// First element of the 'Stream' or error if empty
// head :: Monad m => Stream m a -> m a
fn head<'a,A,S>(s: &Stream<'a,S,A>) -> Option<&'a A> {
    let mut st1 = s.st;
    loop {
        let r = (s.f)(&st1);
        match r {
            Step::Yield(x,_) => { return Some(x) }
            Step::Skip(st2) => { st1 = st2 }
            Step::Done => { return None } 
        }
    }
}

/* basic tests */
pub fn main() {

    let s1: Stream<(),char> = empty();
    let s2: Stream<bool,i64> = singleton(&42);

    assert_eq!(true, null(&s1));
    assert_eq!(false, null(&s2));
    assert_eq!(true, null(&s1));
    assert_eq!(false, null(&s2));

    assert_ne!(head(&s2),None) ;
    assert_eq!(head(&s2),Some(&42));
}
