#[derive(Debug,PartialEq)]
pub enum Opcode{
    HLT,
    IGL, // illegal
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV
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
        0 => Opcode::HLT,
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
