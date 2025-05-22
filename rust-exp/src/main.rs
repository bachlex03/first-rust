

fn main() {
    let divide1: Result<i32, MyError> = divide(4, 2);
    let divide2: Result<i32, MyError> = divide(2, 3);

    // as_ref() converts the Result<i32, MyError> to Result<&i32, &MyError>
    // This is useful when you want to borrow the value inside the Result
    // without consuming the Result itself.
    let res = divide2.as_ref().expect("500");

    // match divide1 {
    //     Ok(value) => println!("divide1: {}", value),
    //     Err(e) => println!("divide1 error: {:?}", e),
    // }

    if divide1.is_ok() {
        println!("{}", divide1.as_ref().unwrap());
    }

    println!("{}", divide1.unwrap());
    println!("{}", divide2.as_ref().unwrap_or(&100));
    println!("{}", res);
}
#[derive(Debug)]enum MyError {
    NotDivisible,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::NotDivisible)
    } else {
        Ok(dividend / divisor)
    }
}
