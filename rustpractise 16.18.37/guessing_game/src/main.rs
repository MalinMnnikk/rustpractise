use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    println!("Guess my number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); //two lines for two method calls!

        let guess: u32 = match guess.trim() .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("That's it! You win!");
                break;
            }
        }
    }
}

// io libary = input/output from ex. users, std = standard library
// let mut guess = stores user input
// let = creates variables, immutable as deafult, add mut = mutable
// :: = associated function, implemented on a type (static method)
// read_line = user input, place into a string, mutable 
//& = this argument is a reference, gives you a way to let mutliple parts of code access one piece of data without needing to copy. Immutable by default, hence add the mut 

// crate = collection of Rust src files. Each has documentation on the instructions on methods and functions.
// dependencies need a network to download when new
// .Rng = methods that random number generators implement
// gen_range = defined by the .Rng, brought into scope
// inclusive on lower, but exclusive on higher bound
// u32 = an unsigned 32-bit number 

// Ordering = variants: Less, Greater and Equal. 
// match = arms --> pattern and the code that should run if the arms pattern matches

// two values guess! = rust allows shadowing = used often when you want to convert a value from one type to another type.
// lets reuse guess variable instead of creating two unique values

// Adding a loop! 
// Stopping the loop, quitting = add break

// Invalid input: 
// switching from .expect to match --> from crashing to handling!
// continue = ask again (restart loop)
