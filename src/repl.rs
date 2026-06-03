use std::io::{self, Write};

pub fn run() {
    loop {
        print_prompt();

        match read_line() {
            None => break, // EOF — Ctrl-D
            Some(raw) => {
                let line = raw.trim();
                if line.is_empty() {
                    continue;
                }
                dispatch(line);
            }
        }
    }
}

fn dispatch(line: &str) {
    // TODO — Exercise 1: parse `line` into a command + args, then:
    //   • if command == "exit"  → call handle_exit(args)
    //   • otherwise            → println!("unknown command: {command}")
    println!("you typed: {line}");
}

// ---------------------------------------------------------------------------
// I/O helpers — these are done for you, don't change them yet.
// ---------------------------------------------------------------------------

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().expect("flush failed");
}

/// Returns None on EOF (Ctrl-D), Some(line) otherwise.
fn read_line() -> Option<String> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(0) => None,
        Ok(_) => Some(buf),
        Err(_) => None,
    }
}
