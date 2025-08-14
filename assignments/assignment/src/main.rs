// steps to calculate
// 1. get user input
// 2. breakdown the variables and operators
// 3. do operation


use std::io;

fn main() {
    let mut input: String = String::new();

    println!("enter to calculate");
    io::stdin().read_line(& mut input).expect("Failed to get user input");
    
    let data: Vec<&str> = input.split_whitespace().collect();

    println!("Data {:?}", data);

    // parse the element
    let a : i32= data[0].parse().ok().unwrap();
    let b : i32= data[2].parse().ok().unwrap();

    println!("{} opr {}", a, b);

    let opr = match data[1] () {
        Ok("+") => a + b,
        Err(err) => panic!("error on the operator {}", err),
    };
}
