use std::io::{self, Read, Write};

use libc::{self, winsize};
use termios::*;

fn ctrl_key(key: u8) -> u8 {
    return key & 0x1f;
}

struct EditorConfig {
    screen_rows: u16,
    screen_cols: u16,
    termios: Termios,
}

impl Drop for EditorConfig {
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

fn get_window_size() -> (u16, u16) {
    let mut ws = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    unsafe {
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) == 0 {
            return (ws.ws_row, ws.ws_col);
        }
    };
    return (0, 0);
}

fn editor_draw_rows(config: &EditorConfig) {
    for _ in 0..config.screen_rows {
        print!("~\r\n");
    }
}

fn editor_refresh_screen(config: &EditorConfig) {
    print!("{}", "\x1b[2J");
    print!("{}", "\x1b[H");
    editor_draw_rows(config);
    print!("{}", "\x1b[H");
}

fn main() -> () {
    let (screen_rows, screen_cols) = get_window_size();
    println!("Size of the screen {}x{}", screen_rows, screen_cols);
    let _editor_config = EditorConfig {
        screen_rows,
        screen_cols,
        termios: create_termios(),
    };
    let _ = enable_raw_mode();
    loop {
        // editor_refresh_screen(&_editor_config);
        let is_finished = editor_process_key();
        if is_finished {
            break;
        }
    }
}
