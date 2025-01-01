fn main() {
    // arrays and their slices
    let arr: [i8; 5] = [1, 2, 3, 4, 5];
    let arr_slice: &[i8] = &arr[2..4];


    println!("slice = {:?}", arr_slice);

    // strings and slices 
    let mut s = String::from("Hello, World");
    println!("The string is: {}", s);
    s.push_str(", Rust ");
    println!("The string is: {}", s);

    let s: &str = "Hello &str, World";
    println!("The string slice is: {}", s);
}

