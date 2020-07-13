extern crate regex;
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
        // s6.extend("foo".chars());
        // s6.extend("bar".chars());
        s6.push_str("foo");
        s6.push_str("bar");
        println!("{}", s6);
    }

    // writes
    {
        use std::fmt::Write;

        let mut l = String::new();
        let e = writeln!(l, "This TODO is the string").ok();
        println!("{:?}", e);
        println!("{}", l);
    }

    // searching
    {
        let s1 = "One fine day, in the middle of the night";

        // find is very overloaded nice.
        assert_eq!(s1.find(','), Some(12));
        assert_eq!(s1.find("night"), Some(35));
        assert_eq!(s1.find(char::is_whitespace), Some(3));
    }

    // matching
    {
        let code = "\t function noodle() {";
        assert_eq!(code.trim_start_matches(&[' ', '\t'] as &[char]), "function noodle() {");
    }

    // formatting types
    {
        // Display
        let x  = (3.14159265, "foo");
        let y  = (314159265, "foo");
        println!("{}", x.0);
        println!("{:?}", x);

        // n.b. this a bug in the book
        // use std::fmt::Binary;
        println!("{:#b}", y.0);

        struct Complex { r: f64, i: f64};

        use std::fmt;
        impl fmt::Display for Complex {
            fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
                let i_sign = if self.i < 0. { '-' } else { '+' };
                write!(dest, "{} {} {}i", self.r, i_sign, f64::abs(self.i))
            }
        }
        println!("{}", Complex { r: 42.32, i: -1.23 });
    }

    // regex package
    {
        use regex::{Regex};

        let semver: Regex =
                Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();

        let yes = r#"regex = "0.2.5""#;
        let yes1 = r#"regex = "0.2.5--Xfoo--YFoo""#;
        assert!(semver.is_match(yes));
        assert!(semver.is_match(yes1));
    }

}
