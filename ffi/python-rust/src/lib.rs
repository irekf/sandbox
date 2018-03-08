use std::thread;
use std::sync::mpsc;

extern crate libc;
use std::slice;
use libc::{size_t, int32_t};

extern crate rand;
use rand::Rng;

const THREAD_COUNT: usize = 10;
const NUMBERS_PER_THREAD: usize = 32;

// let's prduce some random stuff for our python caller

#[no_mangle]
pub extern "C" fn produce_us_some_numbers(array: *mut int32_t, array_size: size_t) {
    let values = unsafe { slice::from_raw_parts_mut(array, array_size as usize) };

    let (tx, rx) = mpsc::channel();

    for _ in 0..THREAD_COUNT {
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..NUMBERS_PER_THREAD {
                match tx1.send(rng.gen::<i32>() % 512) {
                    Ok(..) => {}
                    Err(..) => {
                        println!("apparently, they hung up the phone, bye...");
                        break;
                    }
                };
            }
        });
    }

    for i in 0..values.len() {
        values[i] = rx.recv().unwrap();
    }
}
