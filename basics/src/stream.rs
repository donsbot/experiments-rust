// 
// example of how to do stream fusion types in Rust
//
enum Step<'a,S,A> {
    Yield(&'a A, S),
    Skip(S),
    Done,
}

// data Stream a = forall s. Stream (s -> (Step s a)) s
struct Stream<'a, S, A> {
    f: Box<dyn Fn(S) -> Step<'a,S,A> + 'a>,
    st: S
}

fn empty<'a,A>() -> Stream<'a, (), A> {
    Stream {
        f: Box::new(|_| Step::Done),
        st: ()
    }
}

fn singleton<'a, A>(a: &'a A) -> Stream<'a, bool, A> {
    Stream {
        f: Box::new(move |b| {
                    if b {
                        Step::Yield(a, false)
                    } else {
                        Step::Done
                    }
                }),
        st: true
    }
}

// Check if a 'Stream' is empty
fn null<'a,A,S>(s: Stream<'a,S,A>) -> bool {
    let mut st1 = s.st;
    loop {
        let r = (s.f)(st1);
        match r {
            Step::Yield(..) => { return false }
            Step::Skip(st2) => { st1 = st2 }
            Step::Done => { return true }
        }
    }
}

pub fn main() {

    let s1: Stream<(),char> = empty();
    let s2: Stream<bool,i64> = singleton(&42);

    println!("{}", null(s1));
    println!("{}", null(s2));

}
