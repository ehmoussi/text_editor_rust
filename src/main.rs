use std::io::{self, Read};

fn main() -> () {
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
