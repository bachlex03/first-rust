fn main() { 
    let str: &str = "Hello World";
    let mut string: String = String::from("Hello World");

    let slice = &string[.. 6];

    slice.len();

    string.push("a");
    string.push_str("test");

    string = string.replace("Hello", "Hi");

    println!("{}", string);
}