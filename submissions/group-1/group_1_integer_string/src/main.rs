fn main() {
    let x = 10;
    let x_str = x.to_string();
    let y = x_str.parse::<i32>().unwrap();
    println!("String: {}, Integer: {}",x_str,  y);
}


