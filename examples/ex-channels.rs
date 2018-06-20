extern crate crossbeam_channel;

use std::sync::mpsc;
use std::io::{self, Write};
use std::thread;
use crossbeam_channel::unbounded;

fn report_after(time: u32, tx: &mpsc::Sender<&'static str>) {
    let tx = tx.clone();
    thread::spawn(
        move || {
            thread::sleep_ms(time);
            tx.send("hi").expect("Could not send");
        }
    );
}

fn basic_channels() {
    let (tx, rx) = mpsc::channel();
    report_after(1000, &tx);
    report_after(2400, &tx);
    let mut count = 2;
    while count > 0 {
        match rx.try_recv() {
            Ok(val) => {
                println!("\nresult: {}", val);
                count -= 1;
            }
            Err(mpsc::TryRecvError::Empty) => {
                print!(".");
                io::stdout().flush().expect("could not flush stdout");
                thread::sleep_ms(10);
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Thread disconnected");
                count -= 1;
            }
        }
    };
}

fn receiver(rx: &crossbeam_channel::Receiver<&'static str>, name: &'static str) -> thread::JoinHandle<()> {
    let rx = rx.clone();
    thread::spawn(move || loop {
        match rx.recv() {
            Some(msg) => println!("[{}] {}", name, msg),
            None => break,
        }
    })
}

fn crossbeam_multi_rx() {
    let receivers = {
        let (tx, rx) = unbounded();
        let r1 = receiver(&rx, "one");
        let r2 = receiver(&rx, "two");
        let r3 = receiver(&rx, "tre");
        for i in 0..100u8 {
            tx.send("hello");
            thread::sleep_ms(5);
        }
        (r1, r2, r3)
    };
    receivers.0.join().unwrap();
    receivers.1.join().unwrap();
    receivers.2.join().unwrap();
}

fn main() {
    // basic_channels();
    crossbeam_multi_rx();
}
