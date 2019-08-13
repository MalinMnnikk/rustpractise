// Variables
// immutable variables mean you cannot assign a variable multiple values to the same variable
// add 'mut' and the value is now mutable and can be assigned multiple times 
// mutation can be faster than copying and returning newly allocated instances

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}


// constants = values that are bound to a name, immutable, cannot use 'mut', must be annotated
// useful for parts that all of the code need to know about
// const can only be set to a constant expression, not the result of a function callor other values that can only be computed at runtime 
// named with all uppercase and _ between words 

const MAX_POINTS: u32 = 100_000;

// Shadowing = declaring a new variable with the same name as the previous variable, the new one shadows the first one. 

fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2; 
    println!("The value of x is: {}", x);
}
// binds x to 5, the shadows the previous x and adds 1 to it making 6, after which that value is shadowed and multiplied by 2 giving 12
// complietime error if the 'let' is not used! the value stays immutable even after we are done with it.

let spaces = "    ";
let spaces = spaces.len();

// mut cannot be used because we are not allowed to mutate a variables TYPE
// hence let mut spaces = "   "; spaces = spaces.len(); will cause a compiletime error 

// Data types = what kind of data is being specified so Rust knows how to work with it
// statically typed language, must know the types of all variables at compile time
// when many types are possible, use type annotation 
// if not the compiler will show an error requesting more information

// Scalar types>
// = represents a sing;e value, int, float, bool and char

// Integer types
// = number without a fractional component 
// signed = can be negative since it has the sign - infront of it 
// unsigned = only positive 
// isize or usize = when indexing a collection
// assigned integers out of range causes integer overflow
// compiling in release mode causes a overflow to 'wrap' --> in a u8 system a number 256 would become 0 and 257 1 and so on

// Floating types = numbers with decimal points
// f32 or f63, default f64

fn main() {
    let x = 2.0;
    let y: f32 = 3.0;
}

// Numeric operations

fn main() {
    let sum = 5 + 10;
    let difference = 9.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 43 % 5;
}

// Boolean type 

fn main() {
    let t = true;
    let f: bool = false; // explicit type annotation
}

// Character type

fn main() {
    let c = 'z'; // single quotes, not double as for strings
    let z = 'Z';
    let heart_eyed_cat = '😻';
}

// Compound types

// Tuple type
// general way of grouping number of other values with a variety of types into one compound type
// fixed length

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

// individual values out of a tuple --> pattern matching 

fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructing, breaks the tup into parts
    println!("The value of y is {}", y);
}

// alternativley:

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; // first (count starts from 0)
    let six_point_four = x.1; // second
    let one = x.2; // third 
}

// Array type 
// every elemet must have the same type
// fixed length

fn main() {
    let a = [1, 2, 3, 4, 5];
}

// vectors are allowed to grow and shrink, when in doubt use vectors

let a = [3; 5]; 
// same as:
let a = [ 3, 3, 3, 3, 3];

// Accessing 

fn main() {
    let a = [ 1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
} 

// accessing an element beyond the array will cause a runtime error, the program wont exit successfully 

// Functions

// snake case = lower case and uses _ between words 
// doesnt care about when a function is defined, before or after the main one 
