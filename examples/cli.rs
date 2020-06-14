extern crate single_digit_rpn;

use single_digit_rpn::rpn;
use std::io;

fn prompt(s: &str) -> io::Result<()> {
    use ::std::io::{stdout, Write};

    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            // Calculate RPN from inpute string.
            match rpn(line.as_str()) {
                Ok(n) => println!("{}", n),
                Err(err) => println!("{}", err),
            };
        } else {
            break;
        }
    }
}
