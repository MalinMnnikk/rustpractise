use std::collections::HashMap;

fn mean(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();
    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn main() {    
    let mut numbers = [ 1, 1, 2, 3, 3, 3, 4, 4, 4, 4, 5, 7];
    println!("Mean: {}", mean(&numbers));
    println!("Median: {}", median(&mut numbers));
    println!("Mode: {}", mode(&numbers));
}
