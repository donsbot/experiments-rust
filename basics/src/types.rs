const V_I8: i8 = 0;
const V_I16: i16 = -5i16;
const V_I32: i32 = 0;
const V_I64: i64 = 20_922_789_888_000_000;

const V_U8: u8 = b'*';
const V_U16: u16 = 5u16;
const V_U32: u32 = 0;
const V_U64: u64 = 0;

const V_ISIZE: isize = 137;
const V_USIZE: usize = 0xffff_fc00usize;

const V_F32: f32 = 3.1415926536897832;
const V_F64: f64 = 3.1415926536897832;

const V_TRUE: bool = true;
const V_FALSE: bool = false;

const V_CHAR: char = '\u{CA0}';

const V_UNIT: () = ();

const V_TUPLE: (char, u8, i32) = ('x', 42, 256);

#[derive(Debug)]
struct TyStruct {
    x: f32,
    y: f32,
} // type definiton

// named struct
const V_STRUCT: TyStruct = TyStruct { x: 32., y: 64. };

// tuple-struct ...
#[derive(Debug)]
struct TyTupStruct (i32, char);

const V_TUP_STRUCT: TyTupStruct = TyTupStruct (42, 'f');

// from https://doc.rust-lang.org/reference/items/constant-items.html
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &str = "bitstring";

#[derive(Debug)]
struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};

#[derive(Debug)]
struct Z; // woo hoo

const V_Z: Z = Z;

#[allow(dead_code)]
enum X {} // yeah boi

#[allow(dead_code)]
#[derive(Debug)]
enum Y { Z, X }

#[allow(dead_code)]
fn x() -> Y {
     Y::Z
 }

fn main() {
    println!("i8  = {}", V_I8);
    println!("i16 = {}", V_I16);
    println!("i32 = {}", V_I32);
    println!("i64 = {}", V_I64);

    println!("u8  = {}", V_U8);
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

    println!("tuple = {:?}", V_TUPLE);

    println!("struct = {:?}", V_STRUCT);
    println!("struct2 = {:?}", BITS_N_STRINGS);
    println!("struct3 = {:?}", V_TUP_STRUCT);
    println!("struct4 = {:?}", V_Z);
    println!("struct5 = {:?}", x());
}
