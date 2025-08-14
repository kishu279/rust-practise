use std::{iter, vec};



fn main() {
    println!("Hello, world!");

    let my_vec = vec![1, 2, 3, 4];
    
    let v: Vec<i32> = my_vec.iter().filter(|p| *p % 2 == 0).map(|f| *f * 2).collect();

    println!("{:?}", v);
}

