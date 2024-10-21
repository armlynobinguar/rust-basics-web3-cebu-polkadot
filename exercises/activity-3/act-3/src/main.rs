fn find_maximum(numbers: &[i32]) -> i32 {
    let mut max = numbers[0]; // Initialize the max to the first element
    for &num in numbers.iter() {
        if num > max {
            max = num;
        }
    }
    max
}

fn find_minimum(numbers: &[i32]) -> i32 {
    let mut min = numbers[0]; // Initialize the min to the first element
    for &num in numbers.iter() {
        if num < min {
            min = num;
        }
    }
    min
}

fn main() {
    let numbers = [3, 7, 2, 9, 1, 4];
    
    let max_value = find_maximum(&numbers);
    let min_value = find_minimum(&numbers);
    
    println!("The maximum value is: {}", max_value);
    println!("The minimum value is: {}", min_value);
}
