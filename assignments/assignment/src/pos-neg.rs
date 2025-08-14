use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number ");
    io::stdin().read_line(&mut input).expect("Failed to read user input");

    let number: f32 = input.trim().parse().expect("Faild to get the number fom user input");

    if number > 0.0 {
        println!("Positive");
    } else {
        println!("Negative");
    }
    
}
