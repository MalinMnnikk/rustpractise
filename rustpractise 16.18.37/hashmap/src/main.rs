// Storing Keys with Associated Values in Hash Maps 

// HashMap <K, V>

fn main(){
    use std::collections::HashMap; // less support from the standard library
    let mut scores = HashMap::new(); 
    
    scores.insert(String::from("Calgary Blues"), 10);
    scores.insert(String::from("LA Kings"), 50);

    for (key, value) in &scores { // iterating over key/value pair in a hash map
        println!("{}: {}", key, value);
    }
}

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // will overwrite the first value of 10

    println!("{:?}", scores);
}

// checking for keys and giving them values in case they do not yet have one

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

// or_insert = returns a mutable reference to the value for the entry key, if it doesn't exist inserts the parameter as the new value

fn main() {
    use std::collections::HashMap;
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);//storing the mutable reference in count
        *count += 1;
    }
println!("{:?}", map);
}