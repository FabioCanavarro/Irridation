use nom::types::CompleteStr;

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum Opcode{
    HLT,
    IGL, // illegal
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPB,
    JMPF,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    JNEQ,
    NOP
}

#[derive(Debug,PartialEq)]
pub struct Instruction{
    opcode: Opcode
}

impl Instruction{
    pub fn new(opcode: Opcode) -> Instruction{
        Instruction {opcode}
    }
}

impl From<u8> for Opcode{
    fn from(v: u8)-> Self{
        match v {
        0 => Opcode::LOAD,
        1 => Opcode::ADD,
        2 => Opcode::SUB,
        3 => Opcode::MUL,
        4 => Opcode::DIV,
        5 => Opcode::JMP,
        6 => Opcode::JMPF,
        7 => Opcode::JMPB,
        8 => Opcode::EQ,
        9 => Opcode::NEQ,
        10 => Opcode::GT,
        11 => Opcode::LT,
        12 => Opcode::GTQ,
        13 => Opcode::LTQ,
        14 => Opcode::JEQ,
        15 => Opcode::JNEQ,
        16 => Opcode::NOP,
        _ => Opcode::IGL
        }
    }
}

impl From<Opcode> for u8 {
    fn from(code: Opcode) -> u8 {
        match code {
            Opcode::LOAD => 0,  // Maps LOAD back to 0
            Opcode::ADD => 1,   // Maps ADD back to 1
            Opcode::SUB => 2,   // Maps SUB back to 2
            Opcode::MUL => 3,   // Maps MUL back to 3
            Opcode::DIV => 4,   // Maps DIV back to 4
            Opcode::JMP => 5,   // Maps JMP back to 5
            Opcode::JMPF => 6,  // Maps JMPF back to 6
            Opcode::JMPB => 7,  // Maps JMPB back to 7
            Opcode::EQ => 8,    // Maps EQ back to 8
            Opcode::NEQ => 9,   // Maps NEQ back to 9
            Opcode::GT => 10,   // Maps GT back to 10
            Opcode::LT => 11,   // Maps LT back to 11
            Opcode::GTQ => 12,  // Maps GTQ back to 12
            Opcode::LTQ => 13,  // Maps LTQ back to 13
            Opcode::JEQ => 14,  // Maps JEQ back to 14
            Opcode::JNEQ => 15, // Maps JNEQ back to 15
            Opcode::NOP => 16,
            Opcode::IGL => 255,  // For illegal instructions, we could use a sentinel value like 255
            Opcode::HLT => 254
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode{
    fn from(value: CompleteStr) -> Self {
        match value{
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gtq") => Opcode::GTQ,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("ltq") => Opcode::LTQ,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("jeq") => Opcode::JEQ,
            CompleteStr("jneq") => Opcode::JNEQ,
            CompleteStr("nop") => Opcode::NOP,
            _ => Opcode::IGL,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn make_instruction() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode,Opcode::HLT );
    }

    #[test]
    fn create_instruction() {
        let instruct = Instruction::new(Opcode::HLT);
        assert_eq!(instruct.opcode,Opcode::HLT);
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("load"));
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from(CompleteStr("illegal"));
        assert_eq!(opcode, Opcode::IGL);
    }
}
