// use std::thread;
//
// fn main() {
//     let handle = thread::spawn(|| {
//         "Hello from a thread!"
//     });
//
//     println!("{}", handle.join().unwrap());
// }

// 安全なshared mutable state
// use std::thread;
// use std::sync::{Arc, Mutex};
// use std::time::Duration;
//
// fn main() {
//     let data = Arc::new(Mutex::new(vec![1, 2, 3]));
//
//     for i in 0..3 {
//         let data = data.clone();
//         thread::spawn(move || {
//             let mut data = data.lock().unwrap();
//             data[i] += 1;
//         });
//     }
//
//     thread::sleep(Duration::from_millis(50));
// }

// channel
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::sync::mpsc;
//
// fn main() {
//     let data = Arc::new(Mutex::new(0));
//
//     let (tx, rx) = mpsc::channel();
//
//     for _ in 0..10 {
//         let (data, tx) = (data.clone(), tx.clone());
//
//         thread::spawn(move || {
//             let mut data = data.lock().unwrap();
//             *data += 1;
//
//             tx.send(()).unwrap();
//         });
//     }
//
//     for _ in 0..10 {
//         rx.recv().unwrap();
//     }
// }


use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    for i in 0..10 {
        let tx = tx.clone();
        
        thread::spawn(move || {
            let answer = i * i;
            
            tx.send(answer).unwrap();
        });
    }
    
    for _ in 0..10 {
        println!("{}", rx.recv().unwrap());
    }
}
































