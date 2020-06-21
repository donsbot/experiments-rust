fn main() {
    let x: &str = match Some(("foo", 7)) {
        None => "none",
        Some((y @ "bar", _)) => y,
        Some(("foo", 7)) => "foo_7",
        _ => "anything else",
    };
    println!("{}", x);

    // 1. declarations
    {
        let name: i64 = if 1 > 2 { 7 } else { 8 };
        let name2;
        if 1 > 2 {
            name2 = 7;
        } else {
            name2 = 8;
        }
        assert_eq!(name, name2);
    }

    // 2 case analysis
    {
        let _v = if true {
            ()
        } else if 7 > 8 {
            ()
        } else {
            ()
        };
    }

    // 3. matching
    {
        let _v = match (42, 3) {
            (0, 6) => "7",
            (1, 4) => "8",
            (2, 5) => "9",
            (3, 6) => "10",
            (4, 7) => "11",
            // non-exhaustive patterns, nice!
            _ => "sure",
        };
    }

    // 4.0 enums
    {
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg = Message::Write("my custom message".to_string());

        let v: String = match msg {
            Message::Quit => "The Quit variant has no data to destructure.".to_string(),
            Message::Move { x, y } => {
                format!("Move in the x direction {} and in the y direction {}", x, y)
            }
            Message::Write(text) => text,
            Message::ChangeColor(r, g, b) => {
                format!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        };
        println!("{}", v);
    }

    // 4. matching on sum types
    {
        // generic ADT-like
        enum ETy<T> {
            A { f: T, sz: u32 },
            B { f: T, sz: u32, _a: u64 },
            C { f: T, sz: u32, _count: u32 },
        }

        let v1 = ETy::A {
            f: "my_v1".to_string(),
            sz: 7,
        };
        let v2 = ETy::B {
            f: "my_v2".to_string(),
            sz: 8,
            _a: 7,
        };
        let v3 = ETy::C {
            f: "my_v3".to_string(),
            sz: 7,
            _count: 0,
        };

        let res = match (v2, v1, v3) {
            (ETy::A { f, sz, .. }, _, _) => format!("first is A : {} @ {}", f, sz),
            (ETy::B { f, sz, .. }, _, _) => format!("first is B : {} @ {}", f, sz),
            (ETy::C { f, sz, .. }, _, _) => format!("first is C : {} @ {}", f, sz),
            // unreachable!! _ => "fall through".to_string(),
        };

        println!("{:?}", res);
    }

    // 5. if let
    {
        let v = Some("foo");

        if let None = v {
            println!("WAT");
        } else {
            println!("OK");
        };
    }

    // 6. loops
    {
        let mut n: u64 = 1;
        while n < 100 {
            n *= 2;
        };
        println!("{}", n);

        let mut n: Option<u64> = Some(1);
        while let Some(x) = n {
            if x < 100 {
                n = Some(x*2);
            } else {
                break;
            }
        };
        println!("{:?}", n);
    }
}
