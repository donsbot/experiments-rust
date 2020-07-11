// ch17: pg 392

pub fn main() {
    // latin1
    {
        fn latin1_to_char(c: u8) -> char {
            c as char
        }
        fn char_to_latin1(c: char) -> Option<u8> {
            if c as u32 <= 0xff {
                Some(c as u8)
            } else {
                None
            }
        }

        let cs = [92,102,103,42,43,44,45u8];

        for i in cs.iter() {
            println!("{}", latin1_to_char(*i));
            println!("{:?}", char_to_latin1(latin1_to_char(*i)));
        }
    }

}
