use std::io;

fn main() {
    let guess = 40;
    let mut input = String::new();

    println!("enter anything");

    io::stdin().read_line(& mut input).expect("Failed to get the user input");

    let number:i32 = input.trim().parse().expect("faild to get user input");

    if number == guess {
        println!("Exactly correct");
    } else {
        let diff: i32= guess - number; // may be positive or negatit

        if diff.abs() > 10 {
            println!("Guess Wrong");
        } else {
            println!("close to number");
        }
    }
}