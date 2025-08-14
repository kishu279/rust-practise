use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Enter two number");
    io::stdin().read_line(&mut input1).expect("Failed to get user input");
    io::stdin().read_line(&mut input2).expect("Failed to get user input");


    let mut num1:i32 = input1.trim().parse().expect("Failed to get number");
    let mut num2:i32 = input2.trim().parse().expect("Failed to get number");

    println!("Before Swapping A:{} B:{}", num1, num2);
    
    println!("Swapping...");
    
    num1 = num1 + num2;
    num2 = num1 - num2;
    num1 = num1 - num2;
    
    println!("After Swapping A:{} B:{}", num1, num2);
}