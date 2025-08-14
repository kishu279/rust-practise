// steps
// 1. get the user input
// 2. alag krna parega special character and numbers
// 3. put into the stack
// 4. 

// trim -> remove spc chr -> store and use

use std::io;
// use std::any::type_name;

fn main() {
    let mut input = String::new();

    println!("Just calculate it!!!");
    io::stdin().read_line(& mut input).expect("FAILED TO GET USER INPUT");

    let data = input.split(" ");

    println!("Vector data i think so {:?}", data);
    
    let num1 = data.chars.nth(0).unwrap().parse::<i32>();
    let num2 = data.chars.nth(2).unwrap().parse::<i32>();

    let opr = Some(charAt(1));

    let ans = match opr {
        Some("*") => num1 * num2,
        Some("/") => num1 / num2,
        Some("+") => num1 + num2,
        Some("-") => num1 - num2,
        None => "Pls do this operation only [+, -, *, /]",
    };

    println!("Ans is this {}", ans);
    
}

// fn typeof<T>(_: &T) {

// }

// // returns the position right after the end
// fn get_number(input: &str) {
//     // get the numbers from the input untill the another character get into it
    
    

// }