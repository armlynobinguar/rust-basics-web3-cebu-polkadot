// function to check data type
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("Activity 1");
    activity_1();
    println!("\n");
    println!("Activity 2");
    activity_2();
    println!("\n");
    println!("Activity 3");
    activity_3();
}

// Activity 1
fn activity_1 () {
    let num: i32 = 69;
    let num_to_string: String = num.to_string();

    println!("Number: {}", num);
    print_type_of(&num);

    println!("Stringed num: {} ", num);
    print_type_of(&num_to_string);
} 

// Activity 2
fn activity_2 () {
    let mut char_name = String::from("A");

    println!("pt. 1");
    let mut character = 'A';
    println!("{}ngelo cutie \n", character);

    println!("pt. 2");
    println!("character: {}", char_name);
    change_text(&mut char_name);
    println!("to: {} ", char_name);
}

fn change_text(text2: &mut String) {
    text2.push_str("ngelo");
}

// Activity 3
fn activity_3 () {
    let arr: [i64; 5] = [1, 2, 3, 4, 5];

    let max_value = find_maximum(&arr);
    let min_value = find_minimum(&arr);
    println!("Maximum element in the array: {}", max_value);
    println!("Minimum element in the array: {}", min_value);
}

fn find_maximum(arr: &[i64]) -> i32 {
    let mut large: i64 = 0;
    let mut i: usize = 0;

    large = arr[0];
    while i < arr.len() {
        if large < arr[i] {
            large = arr[i]
        }
        i = i + 1;
    }

    return large.try_into().unwrap();
}

fn find_minimum(arr: &[i64]) -> i32 {
    let mut small: i64 = 0;
    let mut i: usize = 0;

    small = arr[0];
    while i < arr.len() {
        if small > arr[i] {
            small = arr[i]
        }
        i = i + 1;
    }

    return small.try_into().unwrap();
}

