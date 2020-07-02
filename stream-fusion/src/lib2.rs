
// Result of taking a single step in a stream
enum Step<S, A> {
    Yield(A, S),
    Skip(S),
    Done,
}

trait Stream<A> {
    type Seed: Seedable;
    fn next(&self, Self::Seed) -> Step<Self::Seed, A>;
    fn start(&self) -> Self::Seed;

    fn is_empty(&self) -> bool {
        let mut state = self.start();
        loop {
            let v = self.next(state);
            match v {
                Step::Yield(..) => { return false }
                Step::Skip(s) => { state = s }
                Step::Done => { return true }
            }
        }
    }
}

trait Seedable: Copy {}
impl Seedable for Empty {}

#[derive(Copy,Clone)]
enum Empty { Empty }

impl<A> Stream<A> for Empty {
    type Seed = Empty;
    fn next(&self, _: Self::Seed) -> Step<Self::Seed,A> {
        Step::Done
    }
    fn start(&self) -> Self::Seed {
        Empty::Empty
    }
}

fn empty<T>() -> impl Stream<T,Seed=Empty> {
    Empty::Empty
}

pub fn main() {
    let s: &dyn Stream<i64,Seed=Empty> = &empty();
    assert_eq!(true, s.is_empty());
}
