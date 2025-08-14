use std::io;

fn main() {
    let mut principle = String::new();
    let mut rate = String::new();
    let mut interest = String::new();
    
    println!("Enter principal, interest and rate");
    io::stdin().read_line(& mut principle).expect("Failed to get user input");
    io::stdin().read_line(& mut rate).expect("Failed to get user input");
    io::stdin().read_line(& mut interest).expect("Failed to get user input");

    let  num1:i32 = principle.trim().parse().expect("Failed to get number");
    let  num2:i32 = rate.trim().parse().expect("Failed to get number");
    let  num3:i32 = interest.trim().parse().expect("Failed to get number");

    println!("Amount to be paid {}", num1 * num2 * num3);
}