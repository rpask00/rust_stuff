use std::ops::Deref;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::TryRecvError;

fn main() {
    // creatingthread();
    // movekeyword();
    // messagepassing();
    sharingState()
}


fn creatingthread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn movekeyword() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}

#[warn(dead_code)]
fn messagepassing() {
    let (tx, rx) = mpsc::channel();

    let h1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        let msg = String::from("hi");
        println!("msg {} - send", msg);
        match tx.send(msg) {
            Ok(msg) => println!("msg sent"),
            Err(_) => println!("no listener??:!")
        }
    });

    let h2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        match rx.try_recv() {
            Ok(msg) => println!("msg {} - receive", msg),
            Err(_) => println!("no value at the time")
        }

        thread::sleep(Duration::from_millis(2000));

        match rx.try_recv() {
            Ok(msg) => println!("msg {} - receive", msg),
            Err(_) => println!("no value at the time")
        }
    });

    h1.join().unwrap();;
    h2.join().unwrap();;
}

#[warn(dead_code)]
fn sharingState() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut v = counter.lock().unwrap();
            *v = *v + 1;

            thread::sleep(Duration::from_millis((12-i) * 500));
        }))
    }

    let mut i = 0;

    for h in handles {
        i += 1;
        println!("joining {} handle", i);
        h.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());
}

