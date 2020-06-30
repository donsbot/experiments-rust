// 
// example of how to do stream fusion types in Rust
//
enum Step<S,A> {
    Yield(A, S),
    Skip(S),
    Done,
}

// data Stream a = forall s. Stream (s -> (Step s a)) s
struct Stream<'a, S, A> {
    f: &(dyn FnOnce(S) -> Step<S,A>),
    s: S
}

fn empty<'a,A>() -> Stream<'a,(),A> {
    Stream {
        f: &(|_| Step::Done),
        s: ()
    }
}

/*
fn const_<A,B>(a: A,_ : B) -> A {
     a
}
*/

/*
 * can't get this to capture 'x' correctly yet
 */
fn singleton<'a, A>(x: &A) -> Stream<'a,bool,&A> {

    Stream {
        f: |b| { singleton_helper(b, x) },
        s: true
    }

}

//  fn step<A>(b: bool) -> Step<bool,A> {
fn singleton_helper<'a, A>(b:bool, x :&A) -> Step<bool,&A> {
    if b {
        Step::Yield(x , false)
    } else {
        Step::Done
    }
}


/*

-- | Check if a 'Stream' is empty
null :: Monad m => Stream m a -> m Bool
{-# INLINE_FUSED null #-}
null (Stream step t) = null_loop t
  where
    null_loop s = do
      r <- step s
      case r of
        Yield _ _ -> return False
        Skip s'   -> null_loop s'
        Done      -> return True
*/

pub fn main() {

    let s1: Stream<(),char> = empty();

}
