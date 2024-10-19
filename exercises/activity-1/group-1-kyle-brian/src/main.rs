fn main() {
    println!("Hello, world!");
    num_to_string();
}

fn num_to_string(){
    let x : i32 = 10;
    let y = x.to_string();

    let y_integer : i32 = y.parse().unwrap();

    println!("Integer: {}", x);
    println!("String: {}", y);
    println!("Converted: {}", y_integer);

}
