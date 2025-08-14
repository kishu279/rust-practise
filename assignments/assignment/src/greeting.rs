use std::io;

fn main() {
    let mut name = String::new();

    println!("Enter your name");
    io::stdin().read_line(&mut name).expect("Failed to read user input");

    greeting(&name);

    println!("{}", name);
}

fn greeting(input:& String) {
    println!("Hii {}", input);
}