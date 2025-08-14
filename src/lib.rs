use std::collections::HashMap;

pub mod code;
pub mod parser;
pub mod symbol;

use crate::code::Code;
use crate::parser::{Command, ParseError, Parser, Symbol};
use std::fs::File;
use std::io::{BufRead, Seek, Write};
use std::path::PathBuf;

pub struct Assembler<R: BufRead + Seek> {
    symbols: HashMap<String, String>,
    parser: Parser<R>,
}

impl<R: BufRead + Seek> Assembler<R> {
    pub fn new(mut reader: R) -> Result<Self, symbol::SymbolError> {
        let symbols = symbol::create_table(&mut reader)?;

        Ok(Assembler {
            symbols,
            parser: Parser::new(reader),
        })
    }

    pub fn assemble(&mut self, out: PathBuf) -> Result<(), ParseError> {
        let mut file = File::create(out).expect("failed to create output file");

        let mut output_lines = Vec::new(); 

        while self.parser.has_more_commands() {
            self.parser.advance()?;

            if let Some(s) = self.parser.get_current() {
                match s {
                    Command::ACommand { symbol } => match symbol {
                        Symbol::Decimal(d) => {
                            output_lines.push(format!("{:016b}", d));
                        }
                        Symbol::Name(n) => {
                            if let Some(s) = self.symbols.get(n) {
                                output_lines.push(s.clone());
                            }
                        }
                    },

                    Command::CCommand { dest, comp, jmp } => {
                        let d = dest
                            .as_ref()
                            .map(|d| Code::dest(d).unwrap())
                            .unwrap_or("000");
                        let c = Code::comp(&comp).unwrap();
                        let j = jmp
                            .as_ref()
                            .map(|j| Code::jump(j).unwrap())
                            .unwrap_or("000");

                        output_lines.push(format!("111{c}{d}{j}"));
                    }

                    Command::LCommand { .. } => {
                        //labels are ignored
                    }
                }
            }
        }

        let output = output_lines.join("\n");
        write!(file, "{}", output).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;
    use std::io::BufReader;

    #[test]
    pub fn test_parse() {
        let mut bp = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        bp.push("test");
        bp.push("Pong.asm");

        let mut out = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        out.push("test");
        out.push("Pong.hack");

        let file = fs::File::open(bp).expect("failed to open asm file");

        let reader = BufReader::new(file);

        let mut assembler = Assembler::new(reader).unwrap();

        assert!(assembler.assemble(out).is_ok());
    }
}
