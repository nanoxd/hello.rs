pub fn main() {
    let plus_one = |x: i32| x + 1;

    // Types do not have to be annotated for closures
    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    };

    assert_eq!(2, plus_one(1));
    assert_eq!(3, plus_two(1));
}
