#[derive(Debug)]
pub struct RegisterNumber(pub u8);

impl TryFrom<u8> for RegisterNumber {
    type Error = ();
    fn try_from(value: u8) -> Result<RegisterNumber, Self::Error> {
        if value < 32 {
            Ok(RegisterNumber(value))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct SignedOffset {
    pub positive: bool,
    pub offset: u8,
}

impl SignedOffset {
    pub fn to_twos_complement(self: &Self) -> u16 {
        if self.positive {
            0x200u16.wrapping_sub(self.offset as u16) & 0x1FF
        } else {
            self.offset as u16 & 0x1FF
        }
    }

    pub fn from_twos_complement(raw_imm9: u16) -> Self {
        let raw_imm9 = raw_imm9 & 0x1FF;

        if (raw_imm9 & 0x100) != 0 {
            SignedOffset {
                positive: true,
                offset: (0x200u16.wrapping_sub(raw_imm9) as u8),
            }
        } else {
            SignedOffset {
                positive: false,
                offset: raw_imm9 as u8,
            }
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub source_register: RegisterNumber,              // Wt 32 bit
    pub destination_address_register: RegisterNumber, // Xn|SP 64 bit
    pub immediate_byte_offset: SignedOffset,          // imm9
}

impl Instruction {
    const OPCODE: u32 = 0x3800_8800;
    const OPCODE_MASK: u32 = 0xFFE0_FC00;

    fn encode_into_u32(self: &Self, instruction: &mut u32) {
        let rt = (self.source_register.0 & 0x1F) as u32;
        *instruction |= rt;
        let rn = (self.destination_address_register.0 & 0x1F) as u32;
        *instruction |= rn << 5;
        let imm9 = self.immediate_byte_offset.to_twos_complement() as u32;
        *instruction |= imm9 << 12;
    }

    fn decode(instruction: u32) -> Result<Self, ()> {
        let rt_val = (instruction & 0x1F) as u8;
        let source_register = RegisterNumber::try_from(rt_val).unwrap();
        let rn_val = ((instruction >> 5) & 0x1F) as u8;
        let destination_address_register = RegisterNumber::try_from(rn_val).unwrap();
        let raw_imm9 = ((instruction >> 12) & 0x1FF) as u16;
        let immediate_byte_offset = SignedOffset::from_twos_complement(raw_imm9);
        Ok(Self {
            source_register,
            destination_address_register,
            immediate_byte_offset,
        })
    }
}



