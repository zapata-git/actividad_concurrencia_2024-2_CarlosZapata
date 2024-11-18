use std::env;
use std::process::exit;
use std::thread;

fn mythread(arg: &str) {
    println!("{}", arg);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: usize = args.len();
    if argc != 1 {
        eprintln!("usage: main");
        exit(1);
    }

    println!("main: begin");

    let thread1handler = thread::spawn(|| {
        mythread("A");
    });

    let thread2handler = thread::spawn(|| {
        mythread("B");
    });

    thread1handler.join().unwrap();
    thread2handler.join().unwrap();
    println!("main: end");
}
