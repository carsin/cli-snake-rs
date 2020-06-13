extern crate termion;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;

pub fn listen() {
    let mut stdin = async_stdin().bytes();

    loop {
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
    }
}
