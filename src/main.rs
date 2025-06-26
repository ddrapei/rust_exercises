use std::fs::File;
use std::collections::HashMap;

// traits
trait Animal {
    fn  make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound (&self) {
        println!("Bark");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow");
    }
}

trait Summarisible {
    fn summary (&self) -> String;
} 

struct Novel {
    title: String,
    author: String
}

impl Summarisible for Novel {
    fn summary (&self) -> String {
        format!("{}: {}", self.title, self.author)
    }
}

trait Draw {
    fn draw(&self);
}

struct Circle;

impl Draw for Circle {
    fn draw(&self){
        println!("Drawing a circle");
    }
}

struct Square;

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

// generic methods

struct Point <T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new (x: T, y: T) -> Self {
        Self {x, y}
    }
}

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

    // error handling
    let file = File::open("non-existent.txt");

    match file {
        Ok(_file) => {
            println!("Opened a file");
        },
        Err(error) => {
            println!("Cannot open the file: {error}");
        }
    }

    // binary search
    let vector = vec![1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12];
    match binary_search(&vector, 6) {
        Some(index) => println!("The number was found under the index {}", index),
        None => println!("The number is not on the list")
    }

    // mutable borrowing

    let mut x = 5;
    println!("Before mutable borrowing, x: {}", x);
    let y = &mut x;

    *y = 10;
    println!("x: {}", x);

    // immutable borrowing
    let x = 5;
    let y = &x;
    println!("x: {}", x);
    println!("y: {}", y);

    // traits

    let dog = Dog;
    dog.make_sound();
    let cat = Cat;
    cat.make_sound();

    let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    for animal in animals {
        animal.make_sound();
    }

    let novel = Novel {
        title: String::from("Harry Potter"),
        author: String::from("J.K. Rowling")
    };

    println!("{}", novel.summary());


    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square)
    ];

    for shape in shapes {
        shape.draw();
    }

    // generic functions
    let unsorted_number = [1, 5, 29, 2, 0, -5, -99, 129, 24, 8, 1, 924, -57283];
    let largest_number = largest(&unsorted_number);

    println!("The largest number in the array: {}", largest_number);

    // generic methods
    let random_point = Point::new(10, 7);
    let int_point = Point {x: 5, y: 10};
    let float_point = Point {x: 7.5, y: 15.00};

    println!("Different points from the same struct with generic type: 
    random point: ({}, {}), 
    integer point: ({}, {}),
    float point: ({}, {})", random_point.x, random_point.y, int_point.x, int_point.y, float_point.x, float_point.y);

    // nested loops and named loops

    let mut counting_number = 0;

    'counting_loop: loop {
        println!("Counter {}", counting_number);
        let mut remainder = 10;
        counting_number += 1;
        loop {
            println!("Remainder: {}", remainder);

            if remainder == 9 {
                break;
            } 

            if counting_number == 10 {
                break 'counting_loop;
            }

            remainder -= 1;
        }

    }

    // Function that returns 2 variables:

    string_length("I love Rust".to_string());

    // HashMap examples

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);

    println!("{:?}", scores);

    // string slice example

    let string_hello = "hello world";

    let slice = &string_hello[0..5];
    println!("{slice}"); 

    // creating and pushing to vector
    let mut numbers :Vec<i32> = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("The elements of the vector are:");
    for number in &numbers {
        println!("{}", number);
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


// binary search function
fn binary_search (vec: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = vec.len() - 1;

    while low <= high {
        
        let mid = (low + high) / 2;
        let guess = vec[mid];

        if guess == target{
            return Some(mid)
        } else if guess >= target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}

// generic functions

fn largest<T: PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }   
    }
largest
} 

// function that returns multiple values in a tuple

fn string_length (s: String) -> (String, usize) {
    let length = s.len();
    println!("The string is '{}', the length of the string is {} characters", s, length);
    (s, length)
}



