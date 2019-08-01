fn fibonacci(n:32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}
fn main () {
    println!("{}," fibonacci);
}
