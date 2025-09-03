use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle_out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        writeln!(handle_out, "You said: {}", line).expect("Failed to write line");
    }
}
