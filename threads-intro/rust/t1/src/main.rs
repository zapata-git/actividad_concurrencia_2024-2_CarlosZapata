use std::env;
use std::process::exit;
use std::ptr::addr_of;
use std::thread;

static mut MAX: i32 = 1; // Hay que buscar como obtenerlo desde argv
static mut COUNTER: i32 = 0;

fn mythread(arg: &str) {
    let letter = arg;
    let mut i: i32 = 0;
    let i_address = &i;
    println!("{letter}: begin [addr of i: {:p}]", i_address);

    unsafe {
        while i < MAX {
            COUNTER = COUNTER + 1;
            i += 1;
        }
    }
    println!("{letter}: done");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: usize = args.len();

    if argc != 2 {
        eprintln!("usage: main-first <loopcount>");
        exit(1);
    }

    unsafe {
        MAX = (&args[1]).parse().expect("Not a valid loopback value");

        let counter_address = addr_of!(COUNTER);
        println!("main: begin [counter = {COUNTER}] [{:p}]", counter_address);
    }

    let p1handler = thread::spawn(|| {
        mythread("A");
    });

    let p2handler = thread::spawn(|| {
        mythread("B");
    });

    p1handler.join().unwrap();
    p2handler.join().unwrap();
    unsafe {
        print!(
            "main done\n [counter: {}]\n [should: {}]\n",
            COUNTER,
            MAX * 2
        );
    }
}
