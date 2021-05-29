use std::sync::{mpsc, Arc, Mutex};
use std::thread;
/// thread : 1:1(one thread per OS thread) & M:N(green threads -- OS threads)
/// message passing

fn thread_mpsc() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let t = thread::spawn(move || {
        let v = 3;
        tx.send(v).unwrap();
    });
    t.join().unwrap();
    let r = rx.recv().unwrap();
}
/// shared-state: share memory
/// signal : aquire lock then use data ,last automatically unlock the data
fn mutex_() {
    let c = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let c = Arc::clone(&c);
        let handle = thread::spawn(move || {
            let mut v = c.lock().unwrap();
            *v += 1;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("c:{}", c.lock().unwrap());
}

#[test]
fn test_mpsc() {
    thread_mpsc();
}
#[test]
fn test_mutex() {
    mutex_();
}
