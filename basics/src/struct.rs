// pg. 193: structs
// 
// types have CamelCase. fields and functions have snake case
//

pub struct GSM { // greyscale map
    pixels: Vec<u8>,
    size: (usize, usize),
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

    let gsm = mk_gsm(w,h);

    println!("{}", gsm.pixels.len());
}
