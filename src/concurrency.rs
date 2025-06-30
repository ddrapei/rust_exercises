use std::thread;

pub fn concurrency() {
    let handle = thread::spawn(|| {
        for number in 1..10{
            println!("{:?}", number);
        }
    });
    for number in 1..3{
        println!("{:?}", number);
    };

    handle.join().unwrap();
}