fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Maximum: {}", find_maximum(&numbers));
    println!("Minimum: {}", find_minimum(&numbers));
}

fn find_maximum(numbers: &[i32]) -> i32 {
    let mut max = numbers[0];
    for &number in numbers {
        if number > max {
            max = number;
        }
    }
    max
}
fn find_minimum(numbers: &[i32]) -> i32 {
    let mut min = numbers[0];
    for &number in numbers {
        if number < min {
            min = number;
        }
    }
    min
}
