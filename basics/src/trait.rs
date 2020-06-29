// pg 245, traits

trait V {
    fn is_good(&self) -> bool {
        false // default impl
    }
}

extern crate num_traits;
use num_traits::identities as I;

/* whoo num class */
impl<O: PartialOrd + I::Zero> V for O {
    #[inline]
    fn is_good(&self) -> bool {
        *self > I::zero()
    }
}

/*
#[inline]
fn generic_gtz<T: PartialOrd>(v: T, zero: T) -> bool {
    v > zero
}

impl V for i128 { fn is_good(&self) -> bool {  generic_gtz(*self, 0) } }
impl V for i64  { fn is_good(&self) -> bool {  generic_gtz(*self, 0) } }
impl V for i32  { fn is_good(&self) -> bool {  generic_gtz(*self, 0) } }
impl V for i16  { fn is_good(&self) -> bool {  generic_gtz(*self, 0) } }
impl V for i8   { fn is_good(&self) -> bool {  generic_gtz(*self, 0) } }
*/

pub fn main() {
    println!("{}",V::is_good(&1i8));
    println!("{}",V::is_good(&0i16));
    println!("{}",V::is_good(&1i64));
    println!("{}",V::is_good(&0i128));
}
