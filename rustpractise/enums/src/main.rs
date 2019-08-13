// Usecase: IP addresses, two standards V4  & V6
// These are the only options for IP adresses, we can enumerate all possible values.
// Either a V4 OR a V6, not both at the same time 
// Enum values can only be one of the variants 
// V4 and V6 are still both IP addresses so they should be treated the as the same type

#![allow(unused_variables)]
fn main () {
    enum IpAddrKind {
        V4,
        V6, // variants of the enum
    }
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
}
// IpAddrKind is now a custom data type that can be used elsewhere

// same concept, but using just an enum 

fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
let home = IpAddr::V4(String::from(127, 0, 0, 1));
let loopback = IpAddr::V6(String::from("::1"));
}
// attatched the data to each variant of the num directly, so there is no need for an extra struct 
// storing IP addresses is so common that the standard libary has a definition

#![allow(unused_variables)]

fn main() {
    struct Ipv4Addr {
    }
struct Ipv6Addr {
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
}
// any kind of data inside enum variants!
// Variety of types embedded in its variants:

#![allow(unused_variables)]
fn main() {
    enum Message {
        Quit, // no data
        Move { x: i32, y: i32 }, // anonymous struct
        Write(String), // single string
        ChangeColor(i32, i32, i32), // i32 variables
    }
impl Message { // defining methods using enums
    fn call(&self) {
        // method body
    }
}
let m = Message::Write(String::from("hello"));
m.call();
}

// rust doesnt have null, however:

#![allow(unused_variables)]
fn main() {
    enum Option<T> { // <T> syntax, generic type parameter 
        Some(T), // variant of Option<T>
        None,
    }
}
// Option<T> has to be converted into a T before you can perform T operations with it
// This helps catching the issue of assuming something isn't null when it actually is
// check the documentation for how.


