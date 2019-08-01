
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    &s[..]
    }

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
}

// 3 unrelated variables that need to be kept in synch! --> string slice
// adding &str and &s[0..i] --> slice 
// this also eliminates compile time errors
// &str is an immutable reference 
