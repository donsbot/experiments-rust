
const V_I8  : i8  = 0;
const V_I16 : i16 = -5i16;
const V_I32 : i32 = 0;
const V_I64 : i64 = 20_922_789_888_000_000;

const V_U8  : u8  = b'*';
const V_U16 : u16 = 5u16;
const V_U32 : u32 = 0;
const V_U64 : u64 = 0;

const V_ISIZE : isize = 137;
const V_USIZE : usize = 0xffff_fc00usize;

const V_F32   : f32 = 3.1415926536897832;
const V_F64   : f64 = 3.1415926536897832;

const V_TRUE : bool = true;
const V_FALSE : bool = false;

const V_CHAR : char = '\u{CA0}';

const V_UNIT : () = ();

fn main() {
    println!("i8  = {}",  V_I8);
    println!("i16 = {}", V_I16);
    println!("i32 = {}", V_I32);
    println!("i64 = {}", V_I64);

    println!("u8  = {}",  V_U8);
    println!("u16 = {}", V_U16);
    println!("u32 = {}", V_U32);
    println!("u64 = {}", V_U64);

    println!("isize = {}", V_ISIZE);
    println!("usize = {}", V_USIZE);

    println!("f32 = {}", V_F32);
    println!("f64 = {}", V_F64);

    println!("bool = {}", V_TRUE);
    println!("bool = {}", V_FALSE);

    println!("char = {} {}", V_CHAR, V_CHAR);

    println!("unit = {:?}", V_UNIT); // n.b. not Display

}
