use std::io::{self, Read, Write};

use libc;
use termios::*;

fn cltr_key(key: u8) -> u8 {
    return key & 0x1f;
}

struct OrigTermios {
    termios: Termios,
}

impl Drop for OrigTermios {
    fn drop(&mut self) {
        println!("disable raw mode");
        let _ = disable_raw_mode(self.termios);
    }
}

fn create_termios() -> Termios {
    println!("create originel termios");
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
    raw.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    raw.c_oflag &= !(OPOST);
    raw.c_cflag |= CS8;
    raw.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;
    let _ = tcsetattr(fd, libc::TCSAFLUSH, &mut raw);
    Ok(())
}

fn main() -> () {
    let _orig_termios = OrigTermios {
        termios: create_termios(),
    };
    let _ = enable_raw_mode();
    let mut buffer = [0; 1];
    loop {
        let _ = io::stdin().read(&mut buffer[..]);
        let c = buffer[0] as char;
        print!("{}", buffer[0]);
        io::stdout().flush().unwrap();
        if buffer[0] == cltr_key(b'q') {
            break;
        };
    }
}
