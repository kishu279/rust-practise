use std::io;

// =====================
// Control Flow Section
// =====================

// Example: basic match with TrafficLight
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

fn traffic(light: TrafficLight) -> u8 {
    match light {
        TrafficLight::Red => 60,
        TrafficLight::Green => 30,
        TrafficLight::Yellow => 10,
    }
}

// =====================
// Ownership & Borrowing Section
// =====================

fn get_word_from_string(s: &String) -> &str {
    let words = s.as_bytes();
    for (i, &item) in words.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// =====================
// Slice Section
// =====================

fn return_sum(input: &[i32]) -> i32 {
    let mut sum = 0;
    for i in input {
        sum += i;
    }
    sum
}

// =====================
// Structs Section
// =====================

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

// =====================
// Options Section
// =====================

// Example usage will be in main

// =====================
// Main Function
// =====================

fn main() {
    // Control Flow (Enums + Match)
    println!("Light's seconds {}", traffic(TrafficLight::Red));

    // Ownership & Borrowing
    let input = String::from("Sourav Poddar");
    let first_word = get_word_from_string(&input);
    println!("Getting Reference: {}", first_word);

    // Slice
    let arr = [1, 2, 3, 4, 5];
    let sum = return_sum(&arr[..]);
    println!("Sum of array: {}", sum);

    // Structs
    let rect = Rectangle { width: 10, height: 5 };
    println!("Area: {}", rect.area());

    // Options
    let value :Option<i32> = Some(20);
    match value {
        Some(i) => println!("{}", i),
        None => println!("Jaoo Phirr!!"),
    }

    let my_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let find = 4;

    let pos = find_element(&my_arr[..], find);
    
    match pos {
        Some(i) => println!("Found {}", i),
        None => println!("Not Found!!!"),
    }
    
}

fn find_element (ele: &[i32], find: i32) -> Option<usize>{
    
    for (i, &item) in ele.iter().enumerate() {
        if item == find {
            return Some(i);
        }
    }

    return None;
}