// updates number to string
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    // activity 1
    let num: i32 = 42;
    let num_as_string = format!("Number: {}", num);
    print_type_of(&num);
    print_type_of(&num_as_string);
    // activity 2
    let my_char: char = 'D';
    println!("The value of my_char is: {}",my_char);
    print_type_of(&my_char);
    println!("My name is {}ylan Balagtas ",my_char);

    // activity 3
    let my_array: [i32; 5] = [1, 10, 7, 4, 5];
    println!("First element: {}", my_array[0]);

    hello_world();

    println!("the greatest num is {}",get_the_max(&my_array));
    
}

fn hello_world() {
    println!("hello world from a function");
}

fn get_the_max (a: &[i32]) -> i32{
    let mut max_num = a[0];
    for &element in a.iter(){
        if element > max_num {
            max_num = element;
        }
    }
    max_num
}
