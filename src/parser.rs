use std::io::{BufRead, Lines};
use std::iter::Peekable;

const VALID_COMPS: &[&str] = &[
    "0", "1", "-1", "D", "A", "!D", "!A", "-D", "-A", "D+1", "A+1", "D-1", "A-1", "D+A", "D-A",
    "A-D", "D&A", "D|A", "M", "!M", "-M", "M+1", "M-1", "D+M", "D-M", "M-D", "D&M", "D|M",
];

const VALID_JUMPS: &[&str] = &["JGT", "JEQ", "JGE", "JLT", "JNE", "JLE", "JMP"];

const VALID_DESTS: &[&str] = &["M", "D", "DM", "A", "AM", "AD", "ADM"];

pub struct Parser<R: BufRead> {
    lines: Peekable<Lines<R>>,
    current: Option<Command>,
    line_num: u32,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidLabel(String, u32),
    InvalidDest(String, u32),
    InvalidJmp(String, u32),
    InvalidCmp(String, u32),
    InvalidCommand(String),
    IntegerOverflow(u16, u32),
    EndOfFile,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Decimal(u16),
    Name(String),
}

#[derive(Debug, Clone)]
pub enum Command {
    ACommand {
        symbol: Symbol,
    },

    CCommand {
        dest: Option<String>,
        comp: String,
        jmp: Option<String>,
    },

    LCommand {
        label: String,
    },
}

impl<R: BufRead> Parser<R> {
    pub fn new(reader: R) -> Self {
        Parser {
            lines: reader.lines().peekable(),
            current: None,
            line_num: 0,
        }
    }

    pub fn get_current(&self) -> Option<&Command> {
        self.current.as_ref()
    }

    pub fn has_more_commands(&mut self) -> bool {
        //skip lines that are comments or whitespace
        //we can peek to check without advancing iterator
        while let Some(line_result) = self.lines.peek() {
            match line_result {
                Ok(line) if line.trim().is_empty() || line.trim().starts_with("//") => {
                    if let Some(Ok(l)) = self.lines.next() {
                        println!("found comment/empty: {}", l);
                    }
                }
                _ => break,
            }
        }

        //here we are either at end of file, or at a valid command (so far)
        //return true if there is a command and it's valid string, will only parse when we actually advance
        if let Some(l) = self.lines.peek() {
            if l.is_ok() {
                return true;
            }
        }

        return false;
    }

    //this should only be run after has_more_commands has been run
    //next command should be valid string so no need to check
    pub fn advance(&mut self) -> Result<(), ParseError> {
        let mut command_str = self.lines.next().unwrap().unwrap();
        command_str.retain(|c| !c.is_whitespace());

        //need to also parse command here to ensure that it's valid
        let command = match command_str.chars().next() {
            Some('@') => match self.parse_a_command(&command_str, self.line_num) {
                Ok(c) => c,
                Err(e) => return Err(e),
            },

            Some('(') => match self.parse_l_command(&command_str, self.line_num) {
                Ok(c) => c,
                Err(e) => return Err(e),
            },

            Some('M') | Some('A') | Some('D') | Some('0') => {
                match self.parse_c_command(&command_str, self.line_num) {
                    Ok(c) => c,
                    Err(e) => return Err(e),
                }
            }

            Some(_) => {
                return Err(ParseError::InvalidCommand(command_str));
            }

            None => {
                return Err(ParseError::EndOfFile);
            }
        };

        self.current = Some(command);
        self.line_num += 1;
        println!("current command - {:?}", &self.current.as_ref().unwrap());
        Ok(())
    }

    fn parse_a_command(&self, cmd: &str, line: u32) -> Result<Command, ParseError> {
        let symbol = &cmd[1..];

        match symbol.parse::<u16>() {
            
            Ok(i) if i <= 32767 => Ok(Command::ACommand {
                symbol: Symbol::Decimal(i),
            }),

            Ok(i) => Err(ParseError::IntegerOverflow(i, line)),

            Err(_) => Ok(Command::ACommand {
                symbol: Symbol::Name(symbol.to_string()),
            }),
        }
    }

    fn parse_l_command(&self, cmd: &str, line: u32) -> Result<Command, ParseError> {
        //item inside () must be a string, cannot be a number
        //since opening brace already checked, can check that there is a last brace, then take a slice of the middle
        //not checking the validity of brackets, justs if they start with brace, end with brace, and have a symbol in middle
        if let Some(c) = cmd.chars().last() {
            if c != ')' {
                return Err(ParseError::InvalidLabel(cmd.into(), line));
            }
        } else {
            return Err(ParseError::InvalidLabel(cmd.into(), line));
        }

        let label_contents = &cmd[1..cmd.len() - 1];

        match label_contents.parse::<u16>() {
            Ok(_) => return Err(ParseError::InvalidLabel(label_contents.into(), line)),
            Err(_) => Ok(Command::LCommand {
                label: label_contents.into(),
            }),
        }
    }

    fn parse_c_command(&self, cmd: &str, line: u32) -> Result<Command, ParseError> {
        //in form dest=comp;jmp
        //where "dest=" and ";jmp" parts are optional
        //if not a jump, then it must have dest= (i think)
        let mut dest: Option<String> = None;
        let mut jmp: Option<String> = None;
        let mut comp = String::new();

        let mut c_iter = cmd.chars();

        if cmd.contains('=') {
            //iterate until = and check if valid dest
            let mut temp_dest = String::new();

            loop {
                match c_iter.next() {
                    Some('=') | None => break,
                    Some(c) => temp_dest.push(c),
                }
            }

            let mut chars: Vec<char> = temp_dest.chars().collect();
            chars.sort();
            let temp_dest: String = chars.into_iter().collect();

            if !VALID_DESTS.contains(&temp_dest.as_str()) {
                return Err(ParseError::InvalidDest(temp_dest, line));
            } else {
                dest = Some(temp_dest);
            }
        }

        //here we should be at the beginning of 'comp' part of instruction
        let mut has_jmp = false;
        loop {
            match c_iter.next() {
                Some(';') => {
                    has_jmp = true;
                    break;
                }
                Some(c) => comp.push(c),
                None => break,
            }
        }

        if !VALID_COMPS.contains(&comp.as_str()) {
            return Err(ParseError::InvalidCmp(comp, line));
        }

        if has_jmp {
            //iterator will already be at the ";", so can just collect the rest
            let temp_jmp = c_iter.collect::<String>();

            if !VALID_JUMPS.contains(&temp_jmp.as_str()) {
                return Err(ParseError::InvalidJmp(temp_jmp, line));
            }

            jmp = Some(temp_jmp);
        }

        Ok(Command::CCommand { dest, comp, jmp })
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::io::BufReader;

    #[test]
    fn label_invalid_test() {
        let label = "(123)";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_err());
    }

    #[test]
    fn label_valid_test() {
        let label = "(hello)";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_ok());
    }

    #[test]
    fn valid_a_test() {
        let label = "@hello";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_ok());
    }

    #[test]
    fn c_no_jmp_test() {
        let label = "M=D+M";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_ok());

        let command = p.get_current().unwrap();

        match command {
            Command::CCommand { dest, comp, jmp } => {
                assert_eq!(dest.as_ref().unwrap(), "M");
                assert_eq!(comp, "D+M");
                assert!(jmp.is_none())
            }

            _ => {}
        }
    }

    #[test]
    fn c_jmp_test() {
        let label = "M=D+M;JMP";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_ok());

        let command = p.get_current().unwrap();

        match command {
            Command::CCommand { dest, comp, jmp } => {
                assert_eq!(dest.as_ref().unwrap(), "M");
                assert_eq!(comp, "D+M");
                assert_eq!(jmp.as_ref().unwrap(), "JMP")
            }

            _ => {}
        }
    }
    #[test]
    fn c_jmp_no_dst_test() {
        let label = "0;JMP";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_ok());

        let command = p.get_current().unwrap();

        match command {
            Command::CCommand { dest, comp, jmp } => {
                assert!(dest.is_none());
                assert_eq!(comp, "0");
                assert_eq!(jmp.as_ref().unwrap(), "JMP")
            }

            _ => {}
        }
    }

    #[test]
    fn c_invalid_jmp() {
        let label = "0;LOL";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_err());
    }

    #[test]
    fn c_invalid_cmp() {
        let label = "M=3+1";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_err());
    }

    #[test]
    fn c_invalid_dst() {
        let label = "M+A=D+M";
        let bytes = label.as_bytes();
        let reader = BufReader::new(bytes);

        let mut p = Parser::new(reader);

        let res = p.advance();

        assert!(res.is_err());
    }
}
