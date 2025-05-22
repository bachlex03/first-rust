
// "None" indicates that the operation has failed. (failure or lack of value)
// "Some" indicates that the operation has succeeded. (success or presence of value)
fn main() {
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    // Unwrapping a "Some" variant will extract the value wrapped inside.
    print!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // Unwrapping a "None" variant will cause a panic.
    print!("{:?} unwraps to {}", divide2, divide2.unwrap());
}

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
         None
    } else {
        Some(dividend / divisor)
    }
}
