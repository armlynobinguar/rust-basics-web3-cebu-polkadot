// Dominic and Kenjie
fn main() {
    let mut num = 12;
    let mut int_to_str = integer_to_string(&mut num);
    let str_to_int = string_to_integer(&mut int_to_str);

    println!("Integer to String: {}", int_to_str);
    println!("String to Integer: {}", str_to_int);
}

fn string_to_integer(str: &mut String) -> i32 {
    str.to_string().parse::<i32>().unwrap()
}

fn integer_to_string(num: &mut i32) -> String {
    num.to_string()
}
