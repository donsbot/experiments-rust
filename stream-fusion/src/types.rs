
// Result of taking a single step in a stream
pub enum Step<S, A> {
    Yield(A, S),
    Skip(S),
    Done,
}
