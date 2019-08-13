
// duplicating code
fn main() {
    let number_list = vec! [ 34, 500, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list{
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2 , 43, 9];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

// tedious and error prone to write the same thing twice, or more!


fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec! [34, 45, 53, 59, 30];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 59);

    let number_list = vec! [1002, 300, 40, 600, 2, 43, 9];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 1002);
}

// 1. identify duplicated code
// 2. extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature
// 3. update the two instances of duplicated code to call the function instead

// largest value in slice

fn largest_i32(list: &[i32]) -> i32 { // largest i32 in a slice
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char { // largest char in a slice
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T>(list:&[T]) -> T { // declaration before usage, generic data type
// the function largest is generic over some type T, 1 parameter named list, which is a slice balues of type T, will return a value of the same type T
   let mut largest = list[0];
   for &item in list.iter() {
       if item > largest {
           largest = item;
       }
   }
}

fn main() {
    let number_list = vec![34, 300, 50];
    let result = largest_i32(&number_list);
    println!("the largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("the largest char is {}", result);
}

// Struct definitions

struct Point <T, U> {
    x: T,
    y: U,
}
fn main() {
    let integer = Point { x:5, y:10};
    let float = Point { x:1.0, y:4.0};
    let both = Point {x: 5, y: 4.3};
}

// Enum definitions 

enum Result <T, E> {
    Ok(T),
    Err(E), 
}

// Method definitions

struct Point <T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main(){
    let p = Point{ x:5, y:10};
    println! ("p.x = {}", p.x());
}
