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
    dispatch_to(line, &mut io::stdout());
}

fn dispatch_to<W: Write>(line: &str, out: &mut W) {
    // TODO — Exercise 1: parse `line` into a command + args, then:
    //   • if command == "exit"  → call handle_exit(args)
    //   • otherwise            → writeln!(out, "unknown command: {command}")
    writeln!(out, "you typed: {line}").expect("write failed");
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

#[cfg(test)]
mod tests {
    use super::*;

    fn capture(line: &str) -> String {
        let mut buf: Vec<u8> = Vec::new();
        dispatch_to(line, &mut buf);
        String::from_utf8(buf).expect("utf8")
    }

    #[test]
    fn dispatch_echoes_input() {
        assert_eq!(capture("hello"), "you typed: hello\n");
    }

    #[test]
    fn dispatch_echoes_multi_word_input() {
        assert_eq!(capture("foo bar baz"), "you typed: foo bar baz\n");
    }

    #[test]
    fn dispatch_echoes_single_char() {
        assert_eq!(capture("x"), "you typed: x\n");
    }

    #[test]
    fn dispatch_echoes_command_with_args() {
        assert_eq!(capture("exit 0"), "you typed: exit 0\n");
    }

    #[test]
    fn dispatch_preserves_inner_whitespace() {
        assert_eq!(capture("a  b"), "you typed: a  b\n");
    }
}
