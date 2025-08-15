use std::cmp::PartialOrd;
use std::{vec};

pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: i32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("Name is {} and age is {}", self.name, self.age);
    }
}

fn main() {
    println!("Hello, world!");

    let my_vec = vec![1, 2, 3, 4];

    let new_vec: Vec<i32> = my_vec.iter().filter(|p| *p % 2 == 0).map(|f| f * 2).collect();

    println!("{:?}", new_vec);

    let ans = largest(&my_vec);
    println!("{ans}");


    let my_user = User {
        name: String::from("Sourav Poddar"),
        age: 20
    };

    println!("{}", my_user.summarize());

}

// fn largest<T: PartialOrd>(list: &[T]) -> &T {   // partialord is used for comparision related issue when unknown types
//     let mut largest = &list[0];

//     for i in list {
//         if largest < i {
//             largest = i;
//         }
//     }

//     largest
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // copy trait will used to pass the value not the reference
    let mut largest = list[0];

    for i in list {
        if largest < *i {
            largest = *i;
        }
    }

    largest
}

