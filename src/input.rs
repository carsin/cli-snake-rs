use std::io::{stdin};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

pub fn spawn_input_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    let stdin = stdin();

    thread::spawn(move || loop {
        let mut buffer = [0u8; 1];
        stdin.read_exact(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });

    rx
}

pub fn listen() {

}
