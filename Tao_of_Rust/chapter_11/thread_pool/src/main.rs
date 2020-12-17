use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Thunk<'a> = Box<FnBox + Send + 'a>;

struct ThreadPoolSharedData {
    name: Option<String>,
    job_receiver: Mutex<Receiver<Thunk<'static>>>,
    empty_trigger: Mutex<()>,
    empty_condvar: Condvar,
    queued_count: AtomicUsize,
    active_count: AtomicUsize,
    max_thread_count: AtomicUsize,
    panic_count: AtomicUsize,
    static_size: Option<usize>,
}

impl ThreadPoolSharedData {
    fn has_work(&self) -> bool {
        self.queued_count.load(Ordering::SeqCst) > 0
        ||
        self.active_count.load(Ordering::SeqCst) > 0
    }
    fn no_work_notify_all(&self) {
        if !self.has_work() {
            *self.empty_trigger.lock()
                .expect("Unable to notify all joining threads");
            self.empty_condvar.notify_all();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
