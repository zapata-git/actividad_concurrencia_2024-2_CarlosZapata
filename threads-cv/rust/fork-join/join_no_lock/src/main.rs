use std::sync::{Condvar, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

static mut DONE: bool = false;
static mut MUTEX: Mutex<bool> = Mutex::new(false);
static mut CVAR: Condvar = Condvar::new();

fn child() {
    println!("child: begin");
    sleep(Duration::from_secs(1));
    // Changes the value of the DONE condition variable and notifies.
    unsafe {
        DONE = true;
        println!("child: signal");
        // We notify the condvar that the value has changed.
        CVAR.notify_one();
    }
}
fn main() {
    println!("parent: begin");

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(|| child());
    unsafe {
        //Acquires the MUTEX lock
        let mut started = MUTEX.lock().unwrap();
        println!("parent: check condition");

        /* The problem that originates our deadlock occurs mainly because the child thread doesn't utilize the locks.
         * The execution flow makes the parent thread check the DONE condition(which at the moment is false), then sleeps.
         * Then the child thread is called, and it changes the DONE condition and notifies to wake up.
         * But the parent thread wasn't waiting on the condition yet. It was simply on sleep(Duration).
         * Then the parent thread executes normally and the call to wait sends it to sleep until CVAR notifies something.
         * There occurs our deadlock. CVAR has already notified, but at the wrong time. Ultimatelly our main thread remains waiting and our program is deadlocked.
         */
        //Checks the condition variable
        while DONE == false {
            sleep(Duration::from_secs(2));
            println!("parent: wait to be signalled...");
            // Orders the thread to wait. Doing that it releases the "started" MutexGuard
            started = CVAR.wait(started).unwrap();
        }
    }
    // Rust automatically unlocks when the Mutex gets out of scope
    println!("parent: end");
}
