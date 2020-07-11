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

    // unicode methods
    {
        assert_eq!(false, 'X'.is_numeric());
        assert_eq!(true, 'X'.is_alphabetic());
        assert_eq!(true, 'X'.is_alphanumeric());
        assert_eq!(false, 'X'.is_whitespace());
    }

    // String / str
    {
        let s = String::new();
        assert_eq!(s.len(), 0);

        let s1: &str = "foo";
        let s2: String = s1.to_string();
        assert_eq!(s1.len(), 3);
        assert_eq!(s2.len(), 3);

        let s3  = "man hat tan";
        let s4 :String  = s3.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(s4.len(), 9);

        let s5 = &s4[2..4];
        println!("{}", s5);
    }

    // joins/appends
    {
        let mut s6 = String::new();
        s6.extend("foo".chars());
        s6.extend("bar".chars());
        println!("{}", s6);
    }

    // writes
    {
        use std::fmt::Write;

        let mut l = String::new();
        let e = writeln!(l, "This {} is the string", "TODO").ok();
        println!("{:?}", e);
        println!("{}", l);


    }

}
