pub struct Code;

impl Code {
    const CMP_MAP: [&'static str; 28] = [
        "0101010",
        "0111111",
        "0111010",
        "0001100",
        "0110000",
        "0001101",
        "0110001",
        "0001111",
        "0110011",
        "0011111",
        "0110111",
        "0001110",
        "0110010",
        "0000010",
        "0010011",
        "0000111",
        "0000000",
        "0010101",
        "1110000",
        "1110001",
        "1110011",
        "1110111",
        "1110010",
        "1000010",
        "1010011",
        "1000111",
        "1000000",
        "1010101",
    ];

    const DST_MAP: [&'static str; 8] = [
        "000",
        "001",
        "010",
        "011",
        "100",
        "101",
        "110",
        "111",
    ];

    const JMP_MAP: [&'static str; 8] = [
        "000",
        "001",
        "010",
        "011",
        "100",
        "101",
        "110",
        "111",
    ];

    fn cmp_index(cmd: &str) -> Option<usize> {
        match cmd {
            "0" => Some(0),
            "1" => Some(1),
            "-1" => Some(2),
            "D" => Some(3),
            "A" => Some(4),
            "!D" => Some(5),
            "!A" => Some(6),
            "-D" => Some(7),
            "-A" => Some(8),
            "D+1" => Some(9),
            "A+1" => Some(10),
            "D-1" => Some(11),
            "A-1" => Some(12),
            "D+A" => Some(13),
            "D-A" => Some(14),
            "A-D" => Some(15),
            "D&A" => Some(16),
            "D|A" => Some(17),
            "M" => Some(18),
            "!M" => Some(19),
            "-M" => Some(20),
            "M+1" => Some(21),
            "M-1" => Some(22),
            "D+M" => Some(23),
            "D-M" => Some(24),
            "M-D" => Some(25),
            "D&M" => Some(26),
            "D|M" => Some(27),
            _ => None,
        }
    }

    fn dst_index(cmd: &str) -> Option<usize> {
        match cmd {
            "null" => Some(0),
            "M" => Some(1),
            "D" => Some(2),
            "DM" => Some(3),
            "A" => Some(4),
            "AM" => Some(5),
            "AD" => Some(6),
            "ADM" => Some(7),
            _ => None,
        }
    }

    fn jmp_index(cmd: &str) -> Option<usize> {
        match cmd {
            "null" => Some(0),
            "JGT" => Some(1),
            "JEQ" => Some(2),
            "JGE" => Some(3),
            "JLT" => Some(4),
            "JNE" => Some(5),
            "JLE" => Some(6),
            "JMP" => Some(7),
            _ => None,
        }
    }

    pub fn comp(cmd: &str) -> Option<&'static str> {
        Self::cmp_index(cmd).map(|i| Self::CMP_MAP[i])
    }

    pub fn dest(cmd: &str) -> Option<&'static str> {
        Self::dst_index(cmd).map(|i| Self::DST_MAP[i])
    }

    pub fn jump(cmd: &str) -> Option<&'static str> {
        Self::jmp_index(cmd).map(|i| Self::JMP_MAP[i])
    }
}

#[cfg(test)]
mod tests {
    use super::Code;

    #[test]
    fn test_comp() {
        assert_eq!(Code::comp("D+A"), Some("0000010"));
        assert_eq!(Code::comp("M"), Some("1110000"));
        assert_eq!(Code::comp("X"), None);
    }

    #[test]
    fn test_dest() {
        assert_eq!(Code::dest("ADM"), Some("111"));
        assert_eq!(Code::dest("D"), Some("010"));
        assert_eq!(Code::dest("XYZ"), None);
    }

    #[test]
    fn test_jump() {
        assert_eq!(Code::jump("JGT"), Some("001"));
        assert_eq!(Code::jump("JMP"), Some("111"));
        assert_eq!(Code::jump("INVALID"), None);
    }
}
