use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter temperature in fahrenhiet ");
    io::stdin().read_line(&mut input).expect("Failed to read user input");

    let number: f32 = input.trim().parse().expect("Faild to get the number fom user input");

    println!("converting into celsius ...");

    let temp: f32;
    temp = (number - 32.0) * (5.0 / 9.0);
    println!("Temperature in celsius {}", temp);

}
