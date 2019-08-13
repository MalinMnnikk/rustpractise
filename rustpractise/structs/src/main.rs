fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64, 
        active: bool, 
    }
let user1 = User {
    email: String::from("someone@example.com"), //key: value pairs
    username: String::from("someusername123"),
    active: true, // doesnt have to be in the same order 
    sign_in_count: 1,
};
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
   ..user1 // same for the rest as for user 1 
};

user1.email = String::from("anotheremail@example.com"); // dotnotation to get the value 
fn build_user(email: String, username: String) -> User{
    User {
        email, // parameter and field same, no need to write email: email,
        username,
        active: true,
        sign_in_count: 1,
    }

}
}

// structs are similar to tuples
// unlike: each piece of data is named, making them more flexible to use
// here they are named string, u64 and bool respectedly. 

