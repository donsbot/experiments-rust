// examples with modules

// 'mem' becomes a local alias for 'std::mem' path
use std::mem;
use std::collections::HashMap;

pub fn main() {
    // paths and imports
    {
        let mut s1 = 1;
        let mut s2 = 2;
        if s1 > s2 {
            mem::swap(&mut s1, &mut s2);
        }
        println!("swapped: {} {}", s2, s1);

    }

    // local aliases
    {
        let mut x = HashMap::<String,u64>::new();
        x.insert("foo".to_string(), 42);
        x.insert("bar".to_string(), 41);
        println!("{}", x.len());
    }
}
