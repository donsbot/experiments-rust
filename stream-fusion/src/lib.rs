pub mod r#trait;
pub mod closure;
pub mod traitnoskip;

extern crate num_traits;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5000, r#trait::basic_bench(100));
        assert_eq!(5000, closure::basic_bench(100));
        assert_eq!(5000, traitnoskip::basic_bench(100));
    }
}
