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

    // functions
    let result_addition = addition(5, 6);
    println!("The result of addition is: {}", result_addition);
    let result_multiplication = multiplication(5, 6);
    println!("The result of multiplication is: {}", result_multiplication);

    // tuples:
    // ordered list of different types different types
    // fixed sizes
    // immutable

    let person_tuple: (String, i8, bool) = ("Alice".to_string(), 30, true);
    println!("The name of the person {}", person_tuple.0);
    println!("The age of the person {}", person_tuple.1);
    println!("Is the person employed: {}", person_tuple.2);

    let (product, sum) = multiplication_and_addition(5, 6);
    println!("The product is {}, the sum is {}", product, sum);


    // for loop

    for num in 1..5 {
        println!("The number in the loop is {}", num);
    }

    //while loop

    let mut counter: i8 = 0;

    while counter <= 5 {
        println!("The counter is: {}", counter);
        counter += 1
    }

    let mut counter = 0;

    // loop loop

    loop {
        counter += 1;
        println!("The counter from loop loop is: {}", counter);
        if counter == 5{
            println!("The loop loop ends.");
            break;
        }
    }

    // continue statement

    let mut counter = 0;

    while counter < 6 {
       
        counter += 1;

        if counter == 3 {
            println!("Number 3 is scipped");
            continue;
        }
        
        println!("The counter from while loop: {}", counter);

    }

    let mut counter = 0;

    while counter <= 100 {
        if counter % 2 == 0 {
            counter += 1;
            continue;
        }
    
        println!("The uneven numbers before 100 are: {}", counter);
        counter += 1;

    }

    for num in 1..=10 {
        if num % 2 != 0 {
            continue;
        }
        println!("The even numbers until 10 are: {}", num);
    }
}


// functions

fn addition (a:i32, b:i32) -> i32 {
    a + b
}

fn multiplication (a: i32, b:i32) -> i32 {
    a * b
}

fn multiplication_and_addition (a:i32, b:i32) -> (i32, i32) {
    (a+b, a*b)
}
