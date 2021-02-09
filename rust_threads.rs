use std::time::Instant;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    thread::JoinHandle,
};

fn compute() {
    let handles: Vec<JoinHandle<_>> = (0..1000)
        .map(|_| {
            std::thread::spawn(move || {
                let mut buffer = [0; 10];
                {
                    let mut dev_urandom = File::open("/dev/urandom").unwrap();
                    dev_urandom.read_exact(&mut buffer).unwrap();
                }
                let mut dev_null = OpenOptions::new().append(true).open("/dev/null").unwrap();
                dev_null.write_all(&mut buffer).unwrap();
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    // warumup
    compute();
    
    let before = Instant::now();
    for _ in 0..1000 {
        compute();
    }
    let elapsed = before.elapsed();
    println!(
        "{:?} total, {:?} avg per iteration",
        elapsed,
        elapsed / 1000
    );
}
