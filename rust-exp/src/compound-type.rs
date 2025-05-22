fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5]; // array
    let other_arr: [u8; 5] = [1; 5]; // array with same value

    println!("index: {}, length: {}", arr[0], arr.len());

    // print structure of array and other objects
    println!("{:?}", other_arr);
}

// array and slice
fn sliceTest() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let slice: &[u8] = &arr[0..3]; // slice of array
    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 5], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("Length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}