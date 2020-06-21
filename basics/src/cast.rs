fn main() {
    {
        let a:f64 = 1000000.;
        let b = a as i16;

        // 1 as 1
        println!("{} as {}",b,b);
    }
    {
        let a:f64 = 1000000.;
        let b = a as i16;

        // 1000000 as 16960
        println!("{} as {}",a,b);
    }
}
