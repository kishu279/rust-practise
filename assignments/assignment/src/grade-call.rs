use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter marks");
    io::stdin().read_line(& mut input).expect("failed to do so");

    let number: i32 = input.trim().parse().expect("Failed to get the number");

    let grade: char;

    if number >= 80 && number <= 100 {
        grade = 'A'; 
    } else if number < 80 && number >= 60 {
        grade = 'B'; 
    } else if number < 60 && number >= 40 {
        grade = 'C'; 
    } else if number < 40 && number >= 0 {
        grade = 'F'; 
    } else {
        println!("invalid marks");
        return;
    }

    println!("Grade Got {}", grade);
}