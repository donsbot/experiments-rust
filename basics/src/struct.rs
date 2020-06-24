// pg. 193: structs
// 
// types have CamelCase. fields and functions have snake case
//

#[derive(Clone)]
struct GSM { // greyscale map
    pixels: Vec<u8>,
    size: (usize, usize),
}

// tuple-like struct. product type with a newtype and indexing
#[derive(Debug)]
struct P(usize, usize, usize);

// hehey you can use them as newtypes
#[allow(dead_code)]
struct T(Vec<u8>);

// empty data types, wohoo
#[allow(dead_code)]
struct U;

fn mk_p(a: usize, b: usize, c: usize) -> P {
    P(a,b,c)
}

fn mk_gsm(w: usize, h: usize) -> GSM {
    let image = GSM {
        pixels: vec![0; w*h],
        size: (w,h)
    };
    image

}

pub fn main() {

    let w = 1024;
    let h = 576;

    let a = mk_gsm(w,h);
    let b = mk_gsm(w/2, h/2);

    let v = vec![0;0];
    let c = GSM { pixels: v, .. b };

    // move pixels from 'a' , not copied!
    let d = GSM { pixels: a.pixels.clone(), .. a };

    println!("{}", a.pixels.len());
    println!("{}", b.pixels.len());
    println!("{}", c.pixels.len());
    println!("{}", d.pixels.len());

    let a = mk_p(1,2,3);
    println!("{:?}", a);
    println!("{:?}", a.1);
}
