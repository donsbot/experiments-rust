// Interior mutability

use std::fs::File;
use std::mem::MaybeUninit;
use std::mem;

#[derive(Debug)]
pub struct Spider {
    species: String,
    web: bool,
    legs: [File;8],
}


fn mk_spider() -> Result<Spider,std::io::Error> {

    let fs: [File;8] = {

        let mut fs: [MaybeUninit<File>;8] = unsafe {
                MaybeUninit::uninit().assume_init()
        };

        for i in 0..8 {
            let f = File::create(format!("/tmp/f-{}.txt", i))?;
            fs[i] = MaybeUninit::new(f);
        }

        // now convert
        unsafe { mem::transmute::<_,[File;8]>(fs) }
    };

    Ok(
        Spider {
            species: "scary".to_string(),
            web: false,
            legs: fs,
        }
    )
}

pub fn main() {
    let spider = mk_spider().expect("Unable to create spider!");
    println!("{:?}",spider);
}
