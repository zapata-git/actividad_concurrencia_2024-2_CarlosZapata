use std::sync::mpsc;
use std::thread;
struct ObjectFormat {
    a: i32,
    b: i32,
}

fn mythread(mut args: ObjectFormat) -> ObjectFormat {
    println!("args {} {}", args.a, args.b);
    args.a = 1;
    args.b = 2;
    return args;
}

fn main() {
    let args = ObjectFormat { a: 10, b: 20 };

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let value = mythread(args);
        tx.send(value).unwrap();
    });

    handle.join().unwrap();
    let received = rx.recv().unwrap();
    println!("returned {} {}", received.a, received.b);
}
