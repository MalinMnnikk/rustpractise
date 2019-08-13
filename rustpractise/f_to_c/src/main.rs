use std::io;

fn main() {

    loop {

    println!("Please input your temperature in fahrenheit, we'll convert it to Celcius!");
    
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input: f64 = match user_input.trim() .parse() {
            Ok(num) => num,
            Err(_) => continue,
    };
    let result: f64 = (user_input - 32.0) / 1.800;
    println!("{} fahrenheit is {} celsius!", user_input, result);
    }
}

