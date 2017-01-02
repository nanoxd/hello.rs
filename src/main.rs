mod closures;
mod trait_objects;

use trait_objects::Dog;
use trait_objects::Bark;
use trait_objects::make_noise;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io;
use std::num;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

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
    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }
}

// Methods

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
    fn is_larger(&self, &Self) -> bool;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn is_larger(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}

impl Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
}

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn methods() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("Circle's area: {}", c.area());
    println!("Circle's perimeter: {}", c.perimeter());


    let other_circle = c.grow(3.0);
    println!("Circle 1 is bigger than Circle 2: {}", c.is_larger(&other_circle));
}

fn traits_on_generic_structs() {
    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };

    assert!(r.is_square());

    r.height = 42;
    assert!(!r.is_square());
}

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(CliError::Io(err)),
    };

    let mut contents = String::new();

    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(CliError::Io(err));
    }

    let n: i32 = match contents.trim().parse() {
        Ok(n)    => n,
        Err(err) => return Err(CliError::Parse(err)),
    };

    Ok(2 * n)
}

fn file_double_try<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(
        File::open(file_path)
            .map_err(CliError::Io)
    );

    let mut contents = String::new();
    try!(
        file
            .read_to_string(&mut contents)
            .map_err(CliError::Io)
    );

    let n = try!(
        contents
            .trim()
            .parse::<i32>()
            .map_err(CliError::Parse)
    );

    Ok(2 * n)
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
    match_bindings();
    methods();
    traits_on_generic_structs();

    let dog = Dog { breed: "chihuahua".to_string() };
    let noise = make_noise(&dog as &Bark);
    println!("{:?}", noise);

    closures::main();

    match file_double("num") {
        Ok(n)    => println!("{}", n),
        Err(err) => println!("{:?}", err),
    }

    match file_double_try("num") {
        Ok(n)    => println!("{}", n),
        Err(err) => println!("{:?}", err),
    }
}
