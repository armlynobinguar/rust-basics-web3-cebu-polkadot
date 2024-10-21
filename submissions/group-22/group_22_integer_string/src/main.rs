use std::io::{self, stdin};

fn main() {
    println!("Enter integer: ");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    let mut integer: i32 = string_to_integer(&mut input_text);
    let string: String = integer_to_string(&mut integer);

    println!("Integer: {} | Type: <{:?}>", integer, print_type(&integer));
    println!("String: '{}' | Type: <{:?}>", string, print_type(&string));

}


fn string_to_integer(string: &mut str) -> i32 {
    string
        .trim()
        .parse()
        .expect("Integer entered was not a number")
}

fn integer_to_string(integer: &mut i32) -> String {
    integer.to_string()
}

fn print_type<T>(_: &T) -> &'static str{
    std::any::type_name::<T>()
}
