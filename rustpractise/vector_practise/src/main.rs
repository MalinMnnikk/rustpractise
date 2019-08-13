#![allow(unused_variables)]
fn main() {
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

}// goes out of scope here 

// Iteration 

fn main () {
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
}

//mutable references 

fn main() {
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50; // adds 50 to each element 
}
}

fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
let row = vec! [
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
}

