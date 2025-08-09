fn main() {
    let name = String::from("Sourav this side!!!");
    let ans = do_something(&name);

    println!("Name from do_something {}", ans);
}

fn do_something(input : &String) -> &str {
    // will return some data
    return &input[0..6];
}
