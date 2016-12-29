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

fn will_it_divide() {
    let mut x = 7; // mut x: i32

    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            break;
        }
    }
}

fn vectors() {
    let vector = vec![1, 2, 3, 4, 5];

    let index: usize = 0;
    // let another_index: i32 = 1;

    println!("Number is: {}", vector[index]);
    match vector.get(5) {
        Some(x) => println!("Item at {} is {}", 5, x),
        None => println!("Sorry, the vector is too small", )
    }
}

fn ownership() {
    let v = vec![1, 2, 3];

    let v2 = v; // Accessing v will now cause an error
}

fn sum_vec(v: &Vec<i32>) -> i32 {
    v.iter().fold(0, |a, &b| a + b)
}

// Lifetimes
struct Food<'a> {
    kind: &'a str
}

impl<'a> Food<'a> {
    fn kind(&self) -> &'a str {
        self.kind
    }
}

fn food_kind() {
    let kind = "Beans";
    let food = Food { kind: &kind };

    println!("You have {}", food.kind());
}

// Match/Patterns

fn range_match() {
    let x = 2;

    match x {
        1 ... 5 => println!("X is in 1-5"),
        _ => println!("lol")
    }
}

#[derive(Debug)]
struct Person {
    name: Option<String>,
}

fn match_bindings() {

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
    will_it_divide();
    vectors();
    ownership();

    let numbers = vec![1, 2, 3];
    let total_number = sum_vec(&numbers);
    println!("Total is {}", total_number);

    food_kind();
    range_match();
}
