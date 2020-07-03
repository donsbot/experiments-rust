// Result of taking a single step in a stream
//
// Trait implementation:
//  - generators and transformers are data types
//  - impl Stream for each type
//  - consumers are trait methods
//  - seed of stream stored in trait types
//
pub enum Step<S: Stream> {
    Yield(S::Item, S),
    Skip(S),
    Done,
}

pub trait Stream: Sized + Copy {
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

    fn foldl<B,F>(self, f: F, w: B) -> B
          where F : Fn(B,Self::Item) -> B
    {
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

    // Map a function over a 'Stream'
    fn map<B,F>(self, f:F) -> Map<Self,F>
            where F: Fn(Self::Item) -> B
    {
        Map { stream: self, mapf: f }
    }

    // Filter a 'stream' with a predicate
    fn filter<F>(self, f:F) -> Filter<Self,F>
            where F: Fn(&Self::Item) -> bool
    {
        Filter { stream: self, filterp: f }
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

#[derive(Clone,Copy)]
pub struct Map<S,F> { stream: S, mapf: F }

impl<S: Stream, B:Copy, F: Copy> Stream for Map<S, F>
    where F: Fn(S::Item) -> B
{
    type Item = B;

    fn next(&self) -> Step<Self> {
        let f = self.mapf;
        match self.stream.next() {
            Step::Done => Step::Done,
            Step::Skip(s) =>
                Step::Skip(Map { 
                    stream: s,
                    mapf: f
                }),
            Step::Yield(x, s) =>
                Step::Yield(f(x), Map {
                    stream: s,
                    mapf: f
                }),
        }
    }
}

#[derive(Clone,Copy)]
pub struct Filter<S,F> { stream: S, filterp: F }

impl<S: Stream, F: Copy> Stream for Filter<S, F>
    where F: Fn(&S::Item) -> bool
{
    type Item = S::Item;

    fn next(&self) -> Step<Self> {
        let p = self.filterp;
        match self.stream.next() {
            Step::Done => Step::Done,
            Step::Skip(s) =>
                Step::Skip(Filter { 
                    stream: s,
                    filterp: p
                }),
            Step::Yield(x, s) => {
                let s1 = Filter { stream: s, filterp: p };
                if p(&x) {
                    Step::Yield (x, s1)
                } else {
                    Step::Skip(s1)
                }
            }
        }
    }
}

/*
// todo : append, head, take, last, cons
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

    #[test]
    fn test_map() {
        let s1 = replicate(42i64, 10);
        let v = s1.map(|x| { x + 1 } ).foldl(|n, i| n + i, 0);
        assert_eq!(v, 43 * 10)
    }

    #[test]
    fn test_filter() {
        let s1 = replicate(42i64, 10);
        let v = s1.map(|x| { x + 1 } )
                  .filter(|x| { x % 2 == 0 } )
                  .foldl(|n, i| n + i, 0);
        assert_eq!(v, 0)
    }

}
