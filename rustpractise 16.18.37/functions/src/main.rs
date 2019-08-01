fn main() {
    println!("Hello, world!");
    another_function(5, 6);
}
fn another_function(x :i32, y : i32) { // one parameter, x of type i32
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Parameters
// arguments for functions, concrete parameters 
// requires type annotation for each parameter!

// function bodies = statements + expressions 
// statements = instructions that perform some action and do not return a value
// expressions = evaluate to a resulting value 

fn main () {
    let y = 6; // the 6 is an expression
} // statement

// since it doesnt return a value there is nothing for an other let expression to hold on to
// expressions do not include ; at the end, otherwise it is turned into a statement
// returning values

fn five() -> i32 {
    5
}

fn main() {
    let x = five(); // return value of a function to initialize a variable

    println!("The value of x is: {}", x);
}

fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// add a ; to x+1 will cause an error since it is now a statement and doesnt return anything 
// plus_one sats it will return i32, but now it returns an empty tuple

// IF - expressions 