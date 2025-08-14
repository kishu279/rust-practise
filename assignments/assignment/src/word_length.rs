use std::io;

fn main() {
    let mut input = String::new();

    println!("Write something");
    io::stdin().read_line(& mut input).expect("Failed to read user input");


    println!("Word length {}",   input.len());

}