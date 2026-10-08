use crate::a64::{
    shared::{RegisterNumber, SignedOffset},
    traits::Instruction,
};

// Load register signed byte (unprivileged)
// This instruction loads a byte from memory, sign-extends it to 64 bits,
// and writes the result to a register.
// The address that is used for the load is calculated from a base
// register and an immediate offset.
#[derive(Debug)]
pub struct LDTRSB64Bit {
    pub destination_register: RegisterNumber,    // Xt 64 bit
    pub source_address_register: RegisterNumber, // Xn|SP 64 bit
    pub immediate_byte_offset: SignedOffset,     // imm9
}

impl Instruction for LDTRSB64Bit {
    type Error = ();
    const OPCODE: u32 = 0x3880_0800;
    const OPCODE_MASK: u32 = 0xFFC0_0C00;

    fn encode_into_u32(self: &Self, instruction: &mut u32) {
        let rt = (self.destination_register.0 & 0x1F) as u32;
        *instruction |= rt;
        let rn = (self.source_address_register.0 & 0x1F) as u32;
        *instruction |= rn << 5;
        let imm9 = self.immediate_byte_offset.to_twos_complement() as u32;
        *instruction |= imm9 << 12;
    }

    fn decode(instruction: u32) -> Result<Self, ()> {
        let rt_val = (instruction & 0x1F) as u8;
        let destination_register = RegisterNumber::try_from(rt_val).unwrap();
        let rn_val = ((instruction >> 5) & 0x1F) as u8;
        let source_address_register = RegisterNumber::try_from(rn_val).unwrap();
        let raw_imm9 = ((instruction >> 12) & 0x1FF) as u16;
        let immediate_byte_offset = SignedOffset::from_twos_complement(raw_imm9);
        Ok(Self {
            destination_register,
            source_address_register,
            immediate_byte_offset,
        })
    }
}

// Load register signed byte (unprivileged)
// This instruction loads a byte from memory, sign-extends it to 32 bits,
// and writes the result to a register.
// The address that is used for the load is calculated from a base
// register and an immediate offset.
#[derive(Debug)]
pub struct LDTRSB32Bit {
    pub destination_register: RegisterNumber,    // Wt 32 bit
    pub source_address_register: RegisterNumber, // Xn|SP 64 bit
    pub immediate_byte_offset: SignedOffset,     // imm9
}

impl Instruction for LDTRSB32Bit {
    type Error = ();
    const OPCODE: u32 = 0x38C0_0800;
    const OPCODE_MASK: u32 = 0xFFC0_0C00;

    fn encode_into_u32(self: &Self, instruction: &mut u32) {
        let rt = (self.destination_register.0 & 0x1F) as u32;
        *instruction |= rt;
        let rn = (self.source_address_register.0 & 0x1F) as u32;
        *instruction |= rn << 5;
        let imm9 = self.immediate_byte_offset.to_twos_complement() as u32;
        *instruction |= imm9 << 12;
    }

    fn decode(instruction: u32) -> Result<Self, ()> {
        let rt_val = (instruction & 0x1F) as u8;
        let destination_register = RegisterNumber::try_from(rt_val).unwrap();
        let rn_val = ((instruction >> 5) & 0x1F) as u8;
        let source_address_register = RegisterNumber::try_from(rn_val).unwrap();
        let raw_imm9 = ((instruction >> 12) & 0x1FF) as u16;
        let immediate_byte_offset = SignedOffset::from_twos_complement(raw_imm9);
        Ok(Self {
            destination_register,
            source_address_register,
            immediate_byte_offset,
        })
    }
}
