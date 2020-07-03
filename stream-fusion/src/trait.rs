// Result of taking a single step in a stream
//
pub enum Step<S: Stream> {
    Yield(S::Item, S),
    Skip(S),
    Done,
}

pub trait Stream: Copy + Sized {
    type Item;

    fn next(&self) -> Step<Self>;

    fn is_empty(&self) -> bool {
        let mut stream = *self;
        loop {
            let v = stream.next();
            match v {
                Step::Yield(..) => { return false }
                Step::Skip(s) => { stream = s }
                Step::Done => { return true }
            }
        }
    }

    fn foldl<B>(self, f: fn(B,Self::Item) -> B, w: B) -> B {
        let mut stream = self;
        let mut z = w;
        loop {
            let v = stream.next();
            match v {
                Step::Yield(x, s) => {
                    z = f(z, x);
                    stream = s;
                }
                Step::Skip(s) => { stream = s }
                Step::Done => { return z }
            }
        }
    }

    // Length of a stream
    fn length(&self) -> usize {
        self.foldl(|n, _| n + 1, 0)
    }
}

use std::marker::PhantomData;

#[derive(Copy,Clone)]
pub struct Empty<A> { empty: PhantomData<A> }

impl<A: Copy> Stream for Empty<A> {
    type Item = A;

    fn next(&self) -> Step<Self> {
        Step::Done
    }
}

pub fn empty<A>() -> Empty<A> {
    Empty { empty: PhantomData }
}

#[derive(Copy,Clone)]
pub struct Single<A> { item: A, state: bool } 

impl<A: Copy> Stream for Single<A> {
    type Item = A;

    fn next(&self) -> Step<Self> {
        if self.state {
            Step::Yield(self.item, Single {
                item: self.item,
                state: false
            })
        } else {
            Step::Done
        }
    }
}

pub fn single<A>(a: A) -> Single<A> {
    Single { item: a, state: true }
}

#[derive(Copy,Clone)]
pub struct Replicate<A> { item: A, state: usize }

impl<A: Copy> Stream for Replicate<A> {
    type Item = A;

    fn next(&self) -> Step<Self> {
        if self.state == 0 {
            Step::Done
        } else { 
            Step::Yield(self.item, Replicate {
                item: self.item,
                state: self.state - 1
            })
        }
    }
}

pub fn replicate<A>(a: A, n: usize) -> Replicate<A> {
    Replicate { item: a, state: n }
}

/*
// todo : map , filter, replicate, append, head, take, last, cons
// benchmark with generators
*/

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty() {
        let s: Empty<i64> = empty();
        assert_eq!(true, s.is_empty());
    }

    #[test]
    fn test_single() {
        let s: Single<i64> = single(42);
        assert_eq!(false, s.is_empty());
    }

    #[test]
    fn test_length() {
        let s: Empty<i64> = empty();
        assert_eq!(0, s.length());
        let s: Single<i64> = single(42);
        assert_eq!(1, s.length());
    }

    #[test]
    fn test_replicate() {
        let s = replicate(42i64, 100);
        assert_eq!(false, s.is_empty());
        assert_eq!(100, s.length());
    }

}
