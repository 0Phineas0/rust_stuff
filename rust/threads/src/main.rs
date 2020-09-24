// Import Types including Atomic Reference Counted Pointer and Mutex
use std::sync::{Arc, Mutex};

// Import Threads from Rust Standard Library to allow running code in parallel
use std::{thread, time};
fn main() {
    try_concurrency();
}

fn try_concurrency() {

    let data = Arc::new(Mutex::new(vec![1u32, 2, 3, 5, 6]));

    println!("Data before thread mutation: {:?}", data);

    for i in 0..=4 {
        let data = data.clone();

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }

    println!("Data immediately after thread mutation: {:?}", data);

    thread::sleep(time::Duration::new(0, 5_000_000));

    println!("Data 5ms after thread mutation: {:?}", data);

    thread::sleep(time::Duration::new(0, 50_000_000));

    println!("Data 50ms after thread mutation: {:?}", data);

}
