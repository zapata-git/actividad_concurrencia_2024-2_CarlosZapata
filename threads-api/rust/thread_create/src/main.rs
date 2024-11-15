use std::thread;

struct MyArgT {
    a: i32,
    b: i32,
}

fn mythread(args: MyArgT) {
    println!("{} {}", args.a, args.b);
    return;
}
fn main() {
    let args = MyArgT { a: 10, b: 20 };

    let handle = thread::spawn(|| {
        mythread(args);
    });

    handle.join().unwrap();
    println!("done");
}
