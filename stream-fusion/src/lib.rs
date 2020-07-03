pub mod r#trait;
pub mod closure;

extern crate num_traits;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5000, r#trait::basic_bench(100));
        assert_eq!(5000, closure::basic_bench(100));
    }
}
