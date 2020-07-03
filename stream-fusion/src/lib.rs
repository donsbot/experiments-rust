pub mod r#trait;
pub mod closure;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(r#trait::basic_bench(), closure::basic_bench());
    }
}
