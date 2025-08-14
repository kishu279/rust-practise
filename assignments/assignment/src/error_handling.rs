// Panic
// fn main() {
//     // drink("lemonade");
//     drink("maazzaa");
// }

// fn drink(beverage: &str) {
//     // You shouldn't drink too many sugary beverages.
//     if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

//     println!("Some refreshing {} is all I need.", beverage);
// }

// abort and unwind
// to make this work panic strategy can be set using this command
// rustc error_hendling.rs -C panic=abort
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         if cfg!(panic = "abort") {
//             println!("This is not your party. Run!!!!");
//         } else {
//             println!("Spitt it out!!!");
//         }
//     } else {
//         println!("Some refreshing {} is all I need.", beverage);
//     }
// }


#[cfg(panic="unwind")]
fn ahh() {
    println!("Spit it out!!!");
}

#[cfg(not(panic="unwind"))]
fn ahh() {
    println!("This is not your party. Run!!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        ahh();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}
