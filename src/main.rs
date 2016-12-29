fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(n: i32) -> i32 {
    n + 1
    // Lack of semicolon on returns
    // Return is implied on last line, similar to Ruby
}

fn point_me(to: String) -> String {
    to
}

fn array_fun() {
    let array = [1, 2, 3];
    println!("Array has {} elements", array.len());

    let strings = ["Ronnie Ray", "All Day", "Na Na"];
    println!("Who sings that song? {}", strings[0]);
}

fn main() {
    println!("Hello, World!");
    let x = add_one(12);
    println!("add_one to 12 is: {}", x);
    print_number(32);
    let function = point_me;
    let string = String::from("Here");
    function(string);

    array_fun();
}
