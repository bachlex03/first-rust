fn main() {
    let tuple: (i8, bool, f32) = (1, true, 1.2); // tuple
    let tuple2 = (5, 10);

    println!("tuple: {}, {}, {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    // destructuring tuple
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);
}