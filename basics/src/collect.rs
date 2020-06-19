// references let you access a value without changing its ownership
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

// this is like an ownership destructor. the function param takes ownership of the table, and then
// destorys the ks and values.
fn show(t: &Table) { // ref to Table
    for (k,vs) in t { // n.b. iterating over a shared ref produces shared refs to components
        println!("key : {}:", k);
        for v in vs {
            println!(" {}", v);
        }
    }
}

fn sort_works(t: &mut Table) {
    for (_,vs) in t {
        vs.sort();
    }
}

fn main() {
    let mut t = Table::new();

    t.insert("a".to_string(),
            vec!["x".to_string() , "y".to_string(), "z".to_string()]);
    t.insert("b".to_string(),
            vec!["2".to_string() , "3".to_string(), "1".to_string()]);

    show(&t); // create a shared ref T (read-only is fine)
    sort_works(&mut t); // mutate the table
    show(&t); // create a shared ref T (read-only is fine)
}
