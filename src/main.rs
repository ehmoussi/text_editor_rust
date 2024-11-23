use std::io::{self, Read, Write};

use libc;
use termios::*;

fn ctrl_key(key: u8) -> u8 {
    return key & 0x1f;
}

struct OrigTermios {
    termios: Termios,
}

impl Drop for OrigTermios {
    fn drop(&mut self) {
        print!("{}", "\x1b[2J");
        print!("{}", "\x1b[H");
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

fn editor_read_key() -> u8 {
    let mut buffer = [0; 1];
    let _ = io::stdin().read(&mut buffer[..]);
    return buffer[0];
}

fn editor_process_key() -> bool {
    let c = editor_read_key();
    let is_finished = match c {
        _ if c == ctrl_key(b'c') => true,
        _ => {
            print!("{}", c as char);
            io::stdout().flush().unwrap();
            false
        }
    };
    return is_finished;
}

fn editor_draw_rows() {
    for _ in 0..23 {
        print!("~\r\n");
    }
}

fn editor_refresh_screen() {
    print!("{}", "\x1b[2J");
    print!("{}", "\x1b[H");
    editor_draw_rows();
    print!("{}", "\x1b[H");
}

fn main() -> () {
    let _orig_termios = OrigTermios {
        termios: create_termios(),
    };
    let _ = enable_raw_mode();
    loop {
        // editor_refresh_screen();
        let is_finished = editor_process_key();
        if is_finished {
            break;
        }
    }
}
