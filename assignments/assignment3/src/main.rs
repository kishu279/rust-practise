use std::env;
use std::fs;

use std::{collections::HashMap};

fn main() {
    println!("Hello, world!");

    // Vectors

    // let mut my_vec = vec![10, 20, 30, 40, 50];

    // push elements
    // my_vec.push(60);
    // my_vec.push(70);
    // my_vec.push(80);
    // my_vec.push(90);
    // my_vec.push(100);
    
    // pop elements
    // let pop_ele = Some( my_vec.remove(0));

    // match pop_ele {
    //     Some(i) => println!("Popped {}", i),
    //     None => println!("No more elements!!!"),
    // }

    // iterate over each elements
    // for (i, &item) in my_vec.iter().enumerate() {
    //     println!("{} {}", i + 1, item);
    // }


    // Strings

    let mut string_input = String::from("Sourav ");

    // pushing to the end
    string_input.push_str(" poddar");

    // pushing to the 0 pos
    string_input.insert_str(0, "🦀 ");

    println!("String {}", string_input);

    // length as bytes
    let len_as_bytes = string_input.len();
    println!("length of string {}", len_as_bytes);  // 19

    let len_as_chars = string_input.chars().count();
    println!("length {}", len_as_chars);    // 16

    for (_i, &item) in string_input.as_bytes().iter().enumerate() {
        if item == b's' {
            let _  = string_input.replacen("s", "S", 1);
        } else if item == b'p' {
            let _ =string_input.replacen("p", "P", 1);
        }
    }

    println!("String Input {}", string_input);


    // Hashmap
    let mut my_hash: HashMap<String, i32> = HashMap::new();

    my_hash.insert(String::from("sourav"), 20);
    my_hash.insert(String::from("apple"), 20);
    my_hash.insert(String::from("banana"), 30);

    println!("My Hash Map {:?}", my_hash);

    let find: i32 = match my_hash.get(&String::from("sourav")) {
        Some(i) => *i,
        None => -1,
    };

    println!("found age {}", find);

    for (i, items) in my_hash.iter().enumerate() {
        println!("Item {} {:?}", i, items);
    }

    // Error Handling
    
    // drink("water");
    // drink("lemonade");

    let path = String::from("/home/kishu/workspace/DEV/rust-practise/assignments/assignment3/src/poem.txt");
    let read_content = read_file_content(&path[..]);

    println!("File {:?}", read_content)

}

fn read_file_content(path: &str) -> Result<String, std::io::Error> {
    // read file logic
    println!("In file {path}");

    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

// #[cfg(panic="unwind")]
// fn ah() {
//     println!("Spit it out!!!!");
// } 

// #[cfg(not(panic="unwind"))]
// fn ah() {
//     println!("This is not your party. Run!!!!");
// }

// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         // if cfg!(panic="abort") {
//         //     println!("This is not your party. Run!!!!");
//         // } else {
//         //     println!("Spit it out!!!!");
//         // }
    
//         ah();
//     } else {
//         println!("Some refreshing {} is all I need.", beverage);
//     }
// }