use std::io::{self, Read};

use libc;
use termios::*;

fn create_termios() -> Termios {
    let fd = libc::STDIN_FILENO;
    return Termios::from_fd(fd).expect("Failed to create a termios struct");
}

fn disable_raw_mode(mut termios: Termios) -> io::Result<()> {
    let fd = libc::STDIN_FILENO;
    let _ = tcsetattr(fd, libc::TCSAFLUSH, &mut termios);
    Ok(())
}

fn enable_raw_mode() -> io::Result<()> {
    let fd = libc::STDIN_FILENO;
    let mut raw = Termios::from_fd(fd)?;
    let _ = tcgetattr(fd, &mut raw);
    raw.c_lflag &= !(libc::ECHO);
    let _ = tcsetattr(fd, libc::TCSAFLUSH, &mut raw);
    Ok(())
}

fn main() -> () {
    let orig_termios = create_termios();
    let _ = enable_raw_mode();
    let mut buffer = [0; 1];
    loop {
        let _ = io::stdin().read(&mut buffer[..]);
        let c = buffer[0] as char;
        println!("{}", c);
        if c == 'q' {
            let _ = disable_raw_mode(orig_termios);
            break;
        };
    }
}
