
// also call scalar type
fn main() {
    // unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned_integer: u8 = 10; // 8 bits

    // signed integer
    // i8, i16, i32, i64, i128
    let signed_integer: i8 = -10; // 8 bits

    // floating point
    // f32, f64
    let floating_point: f64 = 1.2; // 64 bits

    println!("unsigned_integer: {}", unsigned_integer);
    println!("signed_integer: {}", signed_integer);
    println!("floating_point: {}", floating_point);

    // character - can only be
    let letter = 'a';
    let emoji = '\u{1F600}'; // '😀'

    println!("letter: {}", letter);
    println!("emoji: {}", emoji);

    // boolean
    let is_true: bool = true;
    let is_bool: bool = false && true || false; // false

    println!("is_true: {}", is_true);
}
