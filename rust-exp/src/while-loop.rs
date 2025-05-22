fn main() {
    let mut n = 0;

    while n < 5 {
        println!("n is {}", n);
        n += 1;

        if n == 3 {
            println!("exist");
            break
            // continue; // skip the rest of the loop
        }
    }
}