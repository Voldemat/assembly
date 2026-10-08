pub trait Instruction: Sized {
    type Error;
    const OPCODE: u32;
    const OPCODE_MASK: u32;

    fn is_instruction(i: u32) -> bool {
        i & Self::OPCODE_MASK == Self::OPCODE
    }

    fn encode(self: &Self) -> u32 {
        let mut instruction: u32 = Self::OPCODE;
        self.encode_into_u32(&mut instruction);
        instruction
    }

    fn encode_into_u32(self: &Self, instruction: &mut u32);

    fn decode(i: u32) -> Result<Self, Self::Error>;
}
