use std::io::{self, BufRead, Cursor, Write};

use crate::error::Error;
use crate::lexer::Lexer;

pub const PROMPT: &str = ">> ";

pub struct Console<R: BufRead, W> {
    input: R,
    output: W,
    line_buf: String,
}

impl Console<io::BufReader<io::Stdin>, io::Stdout> {
    pub fn new() -> Self {
        Self::with_input_output(io::BufReader::new(io::stdin()), io::stdout())
    }
}

impl Default for Console<io::BufReader<io::Stdin>, io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: BufRead, W: Write> Console<R, W> {
    pub fn with_input_output(input: R, output: W) -> Self {
        Self {
            input,
            output,
            line_buf: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            write!(self.output, "{}", PROMPT)?;
            self.output.flush()?;

            self.line_buf.clear();
            let _ = self.input.read_line(&mut self.line_buf)?;
            if self.line_buf.is_empty() {
                break;
            }
            let line = Cursor::new(self.line_buf.as_bytes());
            let lexer = Lexer::new(line);
            for tok in lexer {
                let tok = tok?;
                writeln!(self.output, "{:?}", tok)?;
            }
        }
        Ok(())
    }
}
