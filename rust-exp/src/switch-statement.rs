fn main() {
    let i = 5;

    match i {
        0 => println!("i is zero"),
        1 | 2 => println!("i is one or two"),
        3..=4 => println!("i is three or four"),
        _ => println!("default case"),
    }
}