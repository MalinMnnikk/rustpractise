use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;


fn main() {
   let f = File::open("hello.txt");
   let f = match f {
       Ok(file) => file,
       Err(error) => match error.kind(){
           ErrorKind::NotFound => match File::create("hello.txt"){
               Ok(fc) => fc,
               Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
           },   
          other_error => panic!("There was a problem opening the file {:?}", other_error),
       },
   };
}
// look up unwrap_or_else in the standard library afterwards. 

// unwrap in action:

fn main(){
    let f = File::open("hello.txt").unwrap();
}
// --> running without a hello.txt file makes the unwrap method call panic!

// expect in action:

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
// either return the file or call the panic!, will be a parameter that we pass to expect. not the defult panic message unwrap gives

// using unwrap in multiple places can make it tidious to find where exactly the code is breaking

fn main(){
fn read_username_from_file() -> Result<String, io::Error> { // returns a value of type Result<T, E>, if it fails it will call io::Err and give more information to why
    let f = File::open("hello.txt");
    let mut f = match f { // if succeed = save in f and continues
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // no need to say return here since it is the last function
    }
}
}
// result is either a username (success) or Err::io with additional information

// Shortcut to the same thing:

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // works almost in the same way as the match
    let mut s = String::new();
    f.read_to_string(&mut s)?;// return the value inside an Ok to the variable f
    Ok(s)
}
 // errors through the ? operator go through the from function, converts errors from one type to an other

// Even shorter:

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// or even shorter:

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// the ? operator can only be used in functions that return result, not in the main function


// custom type for validation
// for ex. to tell a user if the input is wrong out of scope etc. and ask for a new input
// creating a validation type makes it less tedious to validate more parts of the code, instance of type instead of repeating the validations

fn main() {
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 { // = getter, gets some data from its field and returns it
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess {
                value
            }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
// for the guessing game!

