use borsh::{BorshSerialize, BorshDeserialize, to_vec};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn main () {
    println!("Serializing and Deserializing the User Struct");

    let user = User {
        name: String::from("Sourav Poddar"),
        age: 20,
    };

    let bytes = to_vec(&user).unwrap();
    let again_user = User::try_from_slice(&bytes).unwrap();
    
    println!("{:?}", bytes);

    println!("{:?}", again_user);
}


