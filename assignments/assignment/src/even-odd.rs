use std::io;

fn main() {
    let mut input: String = String::new();
    
    println!("enter the number");
    io::stdin().read_line(& mut input).expect("Failed to get the user input");

    let number: i32 = input.trim().parse().expect("failed to get the input as number");

    if number % 2 == 0 {
        println!("Even {}", number);
    } else {
        println!("Odd {}", number);
    }
}