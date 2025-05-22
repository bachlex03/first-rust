fn main() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(10);
    let c: MyEnum = MyEnum::C { x: 1, y: 2 };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let MyEnum::B(value) = b {
        println!("B value: {}", value);
    }

    if let MyEnum::C {x, y} = c {
        println!("C values: x = {}, y = {}", x, y);
    }
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(u32),
    C { x: i32, y: i32 },
}