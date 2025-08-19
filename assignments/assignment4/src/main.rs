
// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// trait Fix {
//     fn fix (&self) -> String;
// }

// struct User {
//     name: String,
//     age: i32,
// }

// impl Fix for User {
//     fn fix (&self) -> String {
//         return format!("Hii there from fix");
//     }
// }

// impl Summary for User {
//     fn summarize(&self) -> String {
//         return format!("Name is {} and age is {}", self.name, self.age);
//     }
// }
// struct Kalu;

// impl Summary for Kalu {
//     fn summarize(&self) -> String {
//         return format!("This side kalu");
//     }
// }

// // For the string to work 
// impl Summary for String {
//     fn summarize(&self) -> String {
//         return format!("This side String implementation");
//     }
// }

trait Area {
    fn area(&self) -> String;
}

struct circle {
    radius: f64,
}

struct rectangle {
    height: f64,
    width: f64,
}


impl Area for circle {
    fn area(&self) -> String {
        let ans: f64 = (22.0 / 7.0) * self.radius.powf(2.0);

        return format!("circle is {}", ans);
    }
}

impl Area for rectangle {
    fn area(&self) -> String {
        return format!("rectangle is {}", self.height * self.width);
    }
}

fn main() {
    // println!("Hello, world!");

    // let my_vec = vec![1, 2, 3, 4];

    // let new_vec: Vec<i32> = my_vec.iter().filter(|p| *p % 2 == 0).map(|f| f * 2).collect();

    // println!("{:?}", new_vec);

    // let ans = largest(&my_vec);
    // println!("{ans}");


    // let my_user = User {
    //     name: String::from("Sourav Poddar"),
    //     age: 20
    // };

    // println!("{}", my_user.summarize());

    // let kalu = Kalu;

    // Trait as parameter
    // notify(my_user);
    // notify(kalu);
    // notify(1);  // throwing error 
    // notify(String::from("sourav poddar"));  // implemented summary for the string  

    // Implemented trait bound summary
    // do_something(my_user);

    let cir = circle {
        radius: 4.0,
    };

    let rect = rectangle {
        height: 10.0,
        width: 20.0,
    };

    find_area(cir);
    find_area(rect);

    let largest;
    let b = String::from("Sourav Poddar");
    
    {
        let a = String::from("Taniya Sharma");
        largest = find_largest(&a, &b);
        println!("{}", largest);
    }


    
}


fn add_two(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn to_test() {
        let sum_value = add_two(10, 20);

        assert_eq!(sum_value, 30);
    }


}


fn find_area<T: Area>(u: T) {
    println!("Area for {}", u.area());
}

fn find_largest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}




// fn do_something<T: Summary + Fix>(u: T) {
//     println!("{}", u.summarize());
//     println!("{}", u.fix());
// }

// fn notify (u: impl Summary) {
//     println!("{}", u.summarize());
// }



// fn largest<T: PartialOrd>(list: &[T]) -> &T {   // partialord is used for comparision related issue when unknown types
//     let mut largest = &list[0];

//     for i in list {
//         if largest < i {
//             largest = i;
//         }
//     }

//     largest
// }

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // copy trait will used to pass the value not the reference
//     let mut largest = list[0];

//     for i in list {
//         if largest < *i {
//             largest = *i;
//         }
//     }

//     largest
// }

