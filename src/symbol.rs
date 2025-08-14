use std::collections::HashMap;
use std::io::{BufRead, Seek};

#[derive(Debug)]
pub enum SymbolError {
    InvalidSymbol,
    OutofBoundsAddress,
}

pub fn create_table<R: BufRead + Seek>(
    reader: &mut R,
) -> Result<HashMap<String, String>, SymbolError> {
    //dont need to do too much parsing here, just as little as possible
    //first pass for labels then pass for variables
    let mut table = HashMap::new();

    //insert the predefined symbols
    table.insert("R0".into(), format!("{:016b}", 0));
    table.insert("R1".into(), format!("{:016b}", 1));
    table.insert("R2".into(), format!("{:016b}", 2));
    table.insert("R3".into(), format!("{:016b}", 3));
    table.insert("R4".into(), format!("{:016b}", 4));
    table.insert("R5".into(), format!("{:016b}", 5));
    table.insert("R6".into(), format!("{:016b}", 6));
    table.insert("R7".into(), format!("{:016b}", 7));
    table.insert("R8".into(), format!("{:016b}", 8));
    table.insert("R9".into(), format!("{:016b}", 9));
    table.insert("R10".into(), format!("{:016b}", 10));
    table.insert("R11".into(), format!("{:016b}", 11));
    table.insert("R12".into(), format!("{:016b}", 12));
    table.insert("R13".into(), format!("{:016b}", 13));
    table.insert("R14".into(), format!("{:016b}", 14));
    table.insert("R15".into(), format!("{:016b}", 15));
    table.insert("SCREEN".into(), format!("{:016b}", 16384));
    table.insert("KBD".into(), format!("{:016b}", 24576));
    table.insert("SP".into(), format!("{:016b}", 0));
    table.insert("LCL".into(), format!("{:016b}", 1));
    table.insert("ARG".into(), format!("{:016b}", 2));
    table.insert("THIS".into(), format!("{:016b}", 3));
    table.insert("THAT".into(), format!("{:016b}", 4));

    let mut lines: Vec<String> = reader
        .lines()
        .map(|line| line.map_err(|_| SymbolError::InvalidSymbol)) // Map errors to your custom error type
        .collect::<Result<Vec<_>, _>>()?;

    //rewind reader to reuse later
    reader.rewind().unwrap();

    lines.retain(|s| !s.trim().is_empty() && !s.starts_with("//"));

    let mut command_index: u16 = 0;

    for l in lines.iter() {
        let mut s_iter = l.chars();

        match s_iter.next() {
            Some('(') => {
                let symbol = s_iter.take_while(|&c| c != ')').collect::<String>();

                //labels are removed from compiled code so their current index points to the next command
                let mem_str = format!("{:016b}", command_index);
                table.insert(symbol, mem_str);
            }
            Some(_) => {
                command_index += 1;
            }
            None => {}
        };
    }

    //second pass to add variable, variables stored from mem address 1024
    //if the symbol is a number, then this is an A-Instruction and gets ignored
    let mut base_addr: u16 = 16;
    for (_, l) in lines.iter().enumerate() {
        let mut s_iter = l.chars();

        match s_iter.next() {
            Some('@') => {
                let symbol = s_iter.collect::<String>();

                //failing to parse means it must be a string
                match symbol.parse::<u16>() {
                    Ok(_) => continue,
                    Err(_) => {}
                }

                if let None = table.get(&symbol) {
                    let mem_str = format!("{:016b}", base_addr);
                    table.insert(symbol, mem_str);
                    base_addr += 1;
                }
            }
            Some(_) => continue,
            None => {}
        };
    }

    Ok(table)
    //second pass to add variables
}
