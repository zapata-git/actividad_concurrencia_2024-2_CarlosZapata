use std::sync::mpsc;
use std::thread;

fn mythread(arg: i64) -> i64 {
    println!("{}", arg);
    return arg + 1;
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let value = mythread(100);
        tx.send(value).unwrap();
    });
    handle.join().unwrap();
    let received = rx.recv().unwrap();
    println!("returned {received}");
}
