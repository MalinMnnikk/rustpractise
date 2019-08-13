use std::io;

fn main() {
loop{  
    println!("Input the word you want to latinalize!");  
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    println!("{}", piglatinize(&input));
}

fn piglatinize(s: &str) -> String {
    let mut answer = s.to_string();
    let _ = match answer.pop() {
        None => panic!("Empty string"),
        Some(c) => c,
    };
    let len = answer.len();
    let initial_letter = first_char(&s);
    let consonant_initial = is_consonant_initial(initial_letter);
    if consonant_initial {
        answer = answer[1..len] .to_string();
        answer.push(initial_letter);
        answer.push_str("-ay");
    } else {
        answer.push_str("-hay");
    }
    format!("{}", answer)
}

fn first_char(s: &str) -> char {
    let mut chars = s.chars();
    match chars.next() {
        None => panic!("Empty string"),
        Some(c) => c,
    }
}

fn is_consonant_initial(c: char) -> bool {
    for vowel in ['a', 'A', 'I', 'i', 'U', 'u', 'E', 'e', 'O', 'o','Y', 'y', 'ä', 'å', 'ö'].iter() {
        if c == *vowel {
            return false;
        }
    }
    true
}
}
