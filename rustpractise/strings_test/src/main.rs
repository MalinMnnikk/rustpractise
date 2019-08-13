

fn main() {
    let mut s = String::new();
} 

fn main() {
    let data = "initial contents";
    let s = data.to_string();
    // or like this:
    let s = "intial contents".to_string();
    // or like this:
    let s = String::from("initial contents");
}

// String::from --> creates a String from a string literal


// Strings are UTF-8 encoded. Any properly encoded data can be included

fn main() {
    let mut s = String::from("foo");
    s.push_str("bar"); // growing a string 
}
// s will contain foobar now

fn main() {
let mut s1 = String::from("foo");
let  s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
}
// if push_str took ownershop, printing s2 value wouldn't work.

fn main() { 
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);
}

fn main() {
    let s1 = String::from("Hello, "); // no longer valid after the operation since it is moved to the add call
    let s2 = String::from("world!"); // still a valid string after the operation due to add not taking ownership
    let s3 = s1 + &s2; // adding a reference, compiler coerces &String to &str = compiles
    println!("{}, {}, {}", s3, s2, s1);
}
fn main () {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}",s);
}

