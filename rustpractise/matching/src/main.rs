#![allow(unused_variables)]
fn main() {
#[derive(Debug)]
    enum Coin { // enum because there is a fixed limit of cointypes
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // variant quarter now holds a state value
    }
    enum UsState {
        Alabama,
        Alaska,
    }
let coin = Coin::Penny;
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin { // match can return any value, unlike if which only returns bool
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5, // match arm, a pattern + some code
        Coin::Dime => 10, 
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
}

// Matching with Option<T>

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // Some(5) doesnt match the the pattern --> next arm!
            Some(i) => Some(i + 1),
        }
    }
let five = Some(5); 
let six = plus_one(five);
let none = plus_one(None);
}
