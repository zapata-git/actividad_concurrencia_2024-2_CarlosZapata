use std::{
    thread::{self, sleep},
    time,
};

static mut DONE: bool = false;

fn child() {
    println!("child");
    let time = time::Duration::from_secs(5);
    sleep(time);
    unsafe {
        DONE = true;
    }
}

fn main() {
    println!("parent: begin");

    thread::spawn(|| {
        child();
    });
    unsafe {
        while DONE == false {} //Spin
    }
    println!("parent: end");
}
