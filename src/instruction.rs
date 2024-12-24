#[derive(Debug,PartialEq)]
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
    JNEQ
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
        _ => Opcode::IGL
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
}
