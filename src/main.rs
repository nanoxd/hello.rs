fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(n: i32) -> i32 {
    n + 1
    // Lack of semicolon on returns
    // Return is implied on last line, similar to Ruby
}

fn main() {
    println!("Hello, World!");
    let x = add_one(12);
    println!("add_one to 12 is: {}", x);
    print_number(32)
}
