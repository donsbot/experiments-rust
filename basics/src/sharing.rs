/*
 * Pages 114 - 122
 * Sharing and mutation
 */
fn main() {
    // 1. sharing versus mutation
    {
        // v: ptr to contiguous block of i32 on heap, with capacity
        let v: Vec<i32> = vec![4, 8, 19, 27, 34, 10];
        {
            let r = &v; // borrow
            r[0]; // read for its effect?
                  // drop r
        }
        let aside = v; // we can still use 'v', move v to aside
        let r = &aside;
        r[0];
    }

    // 2. more sharing
    {
        fn extend_m(vec: &mut Vec<f64>, slice: &[f64]) {
            for e in slice {
                vec.push(*e);
            }
        }
        let mut a: Vec<f64> = vec![1.0, 2.0, 3.0];
        let b = [4.0, 5.0];
        extend_m(&mut a, &b);
        println!("{:?}", a);

        fn extend_pure<'a>(vec: &Vec<f64>, slice: &[f64]) -> Vec<f64> {
            let mut u = vec.clone();
            for e in slice {
                u.push(*e);
            }
            u // stack return
        }
        let a: Vec<f64> = vec![1.0, 2.0, 3.0];
        let b = [4.0, 5.0];
        let c = extend_pure(&a, &b);
        println!("{:?}", a);
        println!("{:?}", c);
    }
}
