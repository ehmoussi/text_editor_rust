use std::io::{self, Read};

use libc;
use termios::*;

fn enable_raw_mode() -> io::Result<()> {
    let fd = libc::STDIN_FILENO;
    let mut raw = Termios::from_fd(fd)?;
    let _ = tcgetattr(fd, &mut raw);
    raw.c_lflag &= !(libc::ECHO);
    let _ = tcsetattr(fd, libc::TCSAFLUSH, &mut raw);
    Ok(())
}

fn main() -> () {
    let _ = enable_raw_mode();
    let mut buffer = [0; 1];
    loop {
        let _ = io::stdin().read(&mut buffer[..]);
        let c = buffer[0] as char;
        println!("{}", c);
        if c == 'q' {
            break;
        };
    }
}
