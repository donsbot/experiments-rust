// ch 16: pg 374: vecdeques
//
use std::collections::VecDeque;
use std::collections::BinaryHeap;

pub fn main() {
    // introduction
    {
        let mut x: VecDeque<i64> = VecDeque::new();
        x.push_front(4);
        x.push_back(3);
        x.push_back(2);
        for i in x {
            println!("{}",i );
        }
    }

    // binary heaps
    {
        let mut x: BinaryHeap<i64> = BinaryHeap::new();
        x.push(7);
        x.push(3);
        x.push(1);
        println!("{}", x.pop().unwrap());
    }
}
