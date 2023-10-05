pub enum Flag {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterName {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    DE,
    BC,
    SP,
    PC,
    AF
}

impl RegisterName {
    pub fn is_u16_registers(&self) -> bool {
        match self {
            RegisterName::HL => true,
            RegisterName::DE => true,
            RegisterName::BC => true,
            RegisterName::SP => true,
            RegisterName::PC => true,
            _ => false
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        //Set initial values according to pandocs
        Registers {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            f: 0x00,
            pc: 0x0000,
            sp: 0xFFFE,
        }
    }

    pub fn value_of(&self, register: &RegisterName) -> u8 {
        match register {
            RegisterName::A => self.a,
            RegisterName::F => self.f,
            RegisterName::B => self.b,
            RegisterName::C => self.c,
            RegisterName::D => self.d,
            RegisterName::E => self.e,
            RegisterName::H => self.h,
            RegisterName::L => self.l,
            _ => panic!("Only write u8 registers")
        }
    }

    pub fn value_of_u16(&self, register: &RegisterName) -> u16 {
        match register {
            RegisterName::HL => self.get_hl(),
            RegisterName::DE => self.get_de(),
            RegisterName::BC => self.get_bc(),
            RegisterName::SP => self.sp,
            RegisterName::PC => self.pc,
            RegisterName::AF => self.get_af(),
            _ => panic!("Only read u16 registers")
        }
    }

    pub fn write_in(&mut self, register: RegisterName, value:u8){
        match register {
            RegisterName::A => self.a = value,
            RegisterName::F => self.f = value,
            RegisterName::B => self.b = value,
            RegisterName::C => self.c = value,
            RegisterName::D => self.d = value,
            RegisterName::E => self.e = value,
            RegisterName::H => self.h = value,
            RegisterName::L => self.l = value,
            _ => panic!("Only write u8 registers")
        }
    }

    pub fn write_in_u16(&mut self, register: RegisterName, value:u16){
        match register {
            RegisterName::HL => self.set_hl(value),
            RegisterName::DE => self.set_de(value),
            RegisterName::BC => self.set_bc(value),
            RegisterName::AF => self.set_af(value),
            RegisterName::SP => self.sp = value,
            RegisterName::PC => self.pc = value,
            _ => panic!("Only write u16 registers")
        }
    }

    pub fn set_flag(&mut self, flag: Flag){
        self.f |= flag as u8
    }

    pub fn update_flag(&mut self, flag: Flag, condition: bool) {
        if condition {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn clear_flag(&mut self, flag: Flag) {
        self.f &= flag as u8 ^ 0xFF
    }

    pub fn clear_all_flags(&mut self) {
        self.clear_flag(Flag::Z);
        self.clear_flag(Flag::C);
        self.clear_flag(Flag::H);
        self.clear_flag(Flag::N);
    }

    pub fn check_flag(&self, flag: Flag) -> bool {
        let flag_value = flag as u8;
        self.f & flag_value == flag_value as u8
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = value as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }


}
