use std::collections::HashMap;
use std::iter::Cycle;
use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::function::*;


use crate::cpu::registers::{Flag, RegisterName};

pub type OpCodeHandler = fn(&mut CPU) -> u8;

pub fn init_opcode_handlers() -> HashMap<u8, OpCodeHandler> {
    let mut map: HashMap<u8, OpCodeHandler> = HashMap::new();

    map.insert(0x00, |_| 4);
    map.insert(0x0E, |cpu| ld_ru8(cpu, RegisterName::C, 8));
    map.insert(0x11, |cpu| ld_ru16(cpu, RegisterName::DE, 12));
    map.insert(0x21, |cpu| ld_ru16(cpu, RegisterName::HL, 12));
    map.insert(0x47, |cpu| ld_rr(cpu, RegisterName::B, RegisterName::A, 4));
    map.insert(0xC3, |cpu| jp(cpu, 16));

    map.insert(0x01, |cpu| ld_ru16(cpu, RegisterName::BC, 12));
    map.insert(0x02, |cpu| ld_mr(cpu, RegisterName::BC, RegisterName::A, Direction::None, 8));
    map.insert(0x03, |cpu| inc_r(cpu, RegisterName::BC, 8));
    map.insert(0x04, |cpu| inc_r(cpu, RegisterName::B, 4));
    map.insert(0x05, |cpu| dec_r(cpu, RegisterName::B, 4));
    map.insert(0x06, |cpu| ld_ru8(cpu, RegisterName::B, 8));
    map.insert(0x07, |cpu| rlca(cpu, 4));
    map.insert(0x08, |cpu| ld_u16_sp(cpu, 20));
    map.insert(0x09, |cpu| add_rr(cpu, RegisterName::HL, RegisterName::BC, 8));
    map.insert(0x0A, |cpu| ld_rm(cpu, RegisterName::A, RegisterName::BC, Direction::None, 8));
    /*C*/
    map.insert(0x0B, |cpu| dec_r(cpu, RegisterName::BC, 8));
    map.insert(0x0C, |cpu| inc_r(cpu, RegisterName::C, 4));
    map.insert(0x0D, |cpu| dec_r(cpu, RegisterName::C, 4));
    map.insert(0x0F, |cpu| rrca(cpu, 4));

    //Stop
    map.insert(0x10, |cpu| panic!("STOP"));
    /*DE*/
    /*D*/
    map.insert(0x12, |cpu| ld_mr(cpu, RegisterName::DE, RegisterName::A, Direction::None, 8));
    map.insert(0x13, |cpu| inc_r(cpu, RegisterName::DE, 8));
    map.insert(0x14, |cpu| inc_r(cpu, RegisterName::D, 4));
    map.insert(0x15, |cpu| dec_r(cpu, RegisterName::D, 4));
    map.insert(0x16, |cpu| ld_ru8(cpu, RegisterName::D, 8));
    map.insert(0x17, |cpu| rla(cpu, 4));
    map.insert(0x18, |cpu| jr(cpu, 12));
    map.insert(0x19, |cpu| add_rr(cpu, RegisterName::HL, RegisterName::DE, 8));
    map.insert(0x1A, |cpu| ld_rm(cpu, RegisterName::A, RegisterName::DE, Direction::None, 8));
    /*E*/
    map.insert(0x1B, |cpu| dec_r(cpu, RegisterName::DE, 8));
    map.insert(0x1C, |cpu| inc_r(cpu, RegisterName::E, 4));
    map.insert(0x1D, |cpu| dec_r(cpu, RegisterName::E, 4));
    map.insert(0x1E, |cpu| ld_ru8(cpu, RegisterName::E, 8));
    map.insert(0x1F, |cpu| rra(cpu, 4));

    map.insert(0x20, |cpu| jr_if(cpu, Flag::Z, false, 8));
    /*HL*/
    /*H*/
    map.insert(0x21, |cpu| ld_ru16(cpu, RegisterName::HL, 12));
    map.insert(0x22, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::A, Direction::Increment, 8));
    map.insert(0x23, |cpu| inc_r(cpu, RegisterName::HL, 8));
    map.insert(0x24, |cpu| inc_r(cpu, RegisterName::H, 4));
    map.insert(0x25, |cpu| dec_r(cpu, RegisterName::H, 4));
    map.insert(0x26, |cpu| ld_ru8(cpu, RegisterName::H, 8));
    map.insert(0x27, |cpu| daa(cpu, 4));
    map.insert(0x28, |cpu| jr_if(cpu, Flag::Z, true, 8));
    map.insert(0x29, |cpu| add_rr(cpu, RegisterName::HL, RegisterName::HL, 8));
    map.insert(0x2A, |cpu| ld_rm(cpu, RegisterName::A, RegisterName::HL, Direction::Increment, 8));
    /*L*/
    map.insert(0x2B, |cpu| dec_r(cpu, RegisterName::HL, 8));
    map.insert(0x2C, |cpu| inc_r(cpu, RegisterName::L, 4));
    map.insert(0x2D, |cpu| dec_r(cpu, RegisterName::L, 4));
    map.insert(0x2E, |cpu| ld_ru8(cpu, RegisterName::L, 8));
    map.insert(0x2F, |cpu| cpl(cpu, 4));


    map.insert(0x30, |cpu| jr_if(cpu, Flag::C, false, 8));
    /*HL*/
    /*H*/
    map.insert(0x31, |cpu| ld_ru16(cpu, RegisterName::SP, 12));
    map.insert(0x32, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::A, Direction::Decrement, 8));
    map.insert(0x33, |cpu| inc_r(cpu, RegisterName::SP, 8));
    map.insert(0x34, |cpu| inc_m(cpu, RegisterName::HL, 12));
    map.insert(0x35, |cpu| dec_m(cpu, RegisterName::HL, 12));
    map.insert(0x36, |cpu| ld_m_u8(cpu, RegisterName::HL, 12));
    map.insert(0x37, |cpu| scf(cpu, 4));
    map.insert(0x38, |cpu| jr_if(cpu, Flag::C, true, 8));
    map.insert(0x39, |cpu| add_rr(cpu, RegisterName::HL, RegisterName::SP, 8));
    map.insert(0x3A, |cpu| ld_rm(cpu, RegisterName::A, RegisterName::HL, Direction::Decrement, 8));
    /*E*/
    map.insert(0x3B, |cpu| dec_r(cpu, RegisterName::SP, 8));
    map.insert(0x3C, |cpu| inc_r(cpu, RegisterName::A, 4));
    map.insert(0x3D, |cpu| dec_r(cpu, RegisterName::A, 4));
    map.insert(0x3E, |cpu| ld_ru8(cpu, RegisterName::A, 8));
    map.insert(0x3F, |cpu| ccf(cpu, 4));

    //LD B
    map.insert(0x40, |cpu| ld_rr(cpu, RegisterName::B, RegisterName::B, 4));
    map.insert(0x41, |cpu| ld_rr(cpu, RegisterName::B, RegisterName::C, 4));
    map.insert(0x42, |cpu| ld_rr(cpu, RegisterName::B, RegisterName::D, 4));
    map.insert(0x43, |cpu| ld_rr(cpu, RegisterName::B, RegisterName::E, 4));
    map.insert(0x44, |cpu| ld_rr(cpu, RegisterName::B, RegisterName::H, 4));
    map.insert(0x45, |cpu| ld_rr(cpu, RegisterName::B, RegisterName::L, 4));
    map.insert(0x46, |cpu| ld_rm(cpu, RegisterName::B, RegisterName::HL, Direction::None, 8));
    map.insert(0x47, |cpu| ld_rr(cpu, RegisterName::B, RegisterName::A, 4));

    //LD C
    map.insert(0x48, |cpu| ld_rr(cpu, RegisterName::C, RegisterName::B, 4));
    map.insert(0x49, |cpu| ld_rr(cpu, RegisterName::C, RegisterName::C, 4));
    map.insert(0x4A, |cpu| ld_rr(cpu, RegisterName::C, RegisterName::D, 4));
    map.insert(0x4B, |cpu| ld_rr(cpu, RegisterName::C, RegisterName::E, 4));
    map.insert(0x4C, |cpu| ld_rr(cpu, RegisterName::C, RegisterName::H, 4));
    map.insert(0x4D, |cpu| ld_rr(cpu, RegisterName::C, RegisterName::L, 4));
    map.insert(0x4E, |cpu| ld_rm(cpu, RegisterName::C, RegisterName::HL, Direction::None, 8));
    map.insert(0x4F, |cpu| ld_rr(cpu, RegisterName::C, RegisterName::A, 4));

    //LD D
    map.insert(0x50, |cpu| ld_rr(cpu, RegisterName::D, RegisterName::B, 4));
    map.insert(0x51, |cpu| ld_rr(cpu, RegisterName::D, RegisterName::C, 4));
    map.insert(0x52, |cpu| ld_rr(cpu, RegisterName::D, RegisterName::D, 4));
    map.insert(0x53, |cpu| ld_rr(cpu, RegisterName::D, RegisterName::E, 4));
    map.insert(0x54, |cpu| ld_rr(cpu, RegisterName::D, RegisterName::H, 4));
    map.insert(0x55, |cpu| ld_rr(cpu, RegisterName::D, RegisterName::L, 4));
    map.insert(0x56, |cpu| ld_rm(cpu, RegisterName::D, RegisterName::HL, Direction::None, 8));
    map.insert(0x57, |cpu| ld_rr(cpu, RegisterName::D, RegisterName::A, 4));

    //LD E
    map.insert(0x58, |cpu| ld_rr(cpu, RegisterName::E, RegisterName::B, 4));
    map.insert(0x59, |cpu| ld_rr(cpu, RegisterName::E, RegisterName::C, 4));
    map.insert(0x5A, |cpu| ld_rr(cpu, RegisterName::E, RegisterName::D, 4));
    map.insert(0x5B, |cpu| ld_rr(cpu, RegisterName::E, RegisterName::E, 4));
    map.insert(0x5C, |cpu| ld_rr(cpu, RegisterName::E, RegisterName::H, 4));
    map.insert(0x5D, |cpu| ld_rr(cpu, RegisterName::E, RegisterName::L, 4));
    map.insert(0x5E, |cpu| ld_rm(cpu, RegisterName::E, RegisterName::HL, Direction::None, 8));
    map.insert(0x5F, |cpu| ld_rr(cpu, RegisterName::E, RegisterName::A, 4));

    //LD H
    map.insert(0x60, |cpu| ld_rr(cpu, RegisterName::H, RegisterName::B, 4));
    map.insert(0x61, |cpu| ld_rr(cpu, RegisterName::H, RegisterName::C, 4));
    map.insert(0x62, |cpu| ld_rr(cpu, RegisterName::H, RegisterName::D, 4));
    map.insert(0x63, |cpu| ld_rr(cpu, RegisterName::H, RegisterName::E, 4));
    map.insert(0x64, |cpu| ld_rr(cpu, RegisterName::H, RegisterName::H, 4));
    map.insert(0x65, |cpu| ld_rr(cpu, RegisterName::H, RegisterName::L, 4));
    map.insert(0x66, |cpu| ld_rm(cpu, RegisterName::H, RegisterName::HL, Direction::None, 8));
    map.insert(0x67, |cpu| ld_rr(cpu, RegisterName::H, RegisterName::A, 4));

    //LD L
    map.insert(0x68, |cpu| ld_rr(cpu, RegisterName::L, RegisterName::B, 4));
    map.insert(0x69, |cpu| ld_rr(cpu, RegisterName::L, RegisterName::C, 4));
    map.insert(0x6A, |cpu| ld_rr(cpu, RegisterName::L, RegisterName::D, 4));
    map.insert(0x6B, |cpu| ld_rr(cpu, RegisterName::L, RegisterName::E, 4));
    map.insert(0x6C, |cpu| ld_rr(cpu, RegisterName::L, RegisterName::H, 4));
    map.insert(0x6D, |cpu| ld_rr(cpu, RegisterName::L, RegisterName::L, 4));
    map.insert(0x6E, |cpu| ld_rm(cpu, RegisterName::L, RegisterName::HL, Direction::None, 8));
    map.insert(0x6F, |cpu| ld_rr(cpu, RegisterName::L, RegisterName::A, 4));

    //LD (HL)
    map.insert(0x70, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::B, Direction::None, 8));
    map.insert(0x71, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::C, Direction::None, 8));
    map.insert(0x72, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::D, Direction::None, 8));
    map.insert(0x73, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::E, Direction::None, 8));
    map.insert(0x74, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::H, Direction::None, 8));
    map.insert(0x75, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::L, Direction::None, 8));
    map.insert(0x76, |cpu| halt(cpu, 4));
    map.insert(0x77, |cpu| ld_mr(cpu, RegisterName::HL, RegisterName::A, Direction::None, 8));

    //LD A
    map.insert(0x78, |cpu| ld_rr(cpu, RegisterName::A, RegisterName::B, 4));
    map.insert(0x79, |cpu| ld_rr(cpu, RegisterName::A, RegisterName::C, 4));
    map.insert(0x7A, |cpu| ld_rr(cpu, RegisterName::A, RegisterName::D, 4));
    map.insert(0x7B, |cpu| ld_rr(cpu, RegisterName::A, RegisterName::E, 4));
    map.insert(0x7C, |cpu| ld_rr(cpu, RegisterName::A, RegisterName::H, 4));
    map.insert(0x7D, |cpu| ld_rr(cpu, RegisterName::A, RegisterName::L, 4));
    map.insert(0x7E, |cpu| ld_rm(cpu, RegisterName::A, RegisterName::HL, Direction::None, 8));
    map.insert(0x7F, |cpu| ld_rr(cpu, RegisterName::A, RegisterName::A, 4));

    //ADD
    map.insert(0x80, |cpu| add_rr(cpu, RegisterName::A, RegisterName::B , 4));
    map.insert(0x81, |cpu| add_rr(cpu, RegisterName::A, RegisterName::C , 4));
    map.insert(0x82, |cpu| add_rr(cpu, RegisterName::A, RegisterName::D , 4));
    map.insert(0x83, |cpu| add_rr(cpu, RegisterName::A, RegisterName::E , 4));
    map.insert(0x84, |cpu| add_rr(cpu, RegisterName::A, RegisterName::H , 4));
    map.insert(0x85, |cpu| add_rr(cpu, RegisterName::A, RegisterName::L , 4));
    map.insert(0x86, |cpu| add_rm(cpu, RegisterName::A, RegisterName::HL , 8));
    map.insert(0x87, |cpu| add_rr(cpu, RegisterName::A, RegisterName::A , 4));

    //ADC
    map.insert(0x88, |cpu| adc_rr(cpu, RegisterName::A, RegisterName::B , 4));
    map.insert(0x89, |cpu| adc_rr(cpu, RegisterName::A, RegisterName::C , 4));
    map.insert(0x8A, |cpu| adc_rr(cpu, RegisterName::A, RegisterName::D , 4));
    map.insert(0x8B, |cpu| adc_rr(cpu, RegisterName::A, RegisterName::E , 4));
    map.insert(0x8C, |cpu| adc_rr(cpu, RegisterName::A, RegisterName::H , 4));
    map.insert(0x8D, |cpu| adc_rr(cpu, RegisterName::A, RegisterName::L , 4));
    map.insert(0x8E, |cpu| adc_rm(cpu, RegisterName::A, RegisterName::HL , 8));
    map.insert(0x8F, |cpu| adc_rr(cpu, RegisterName::A, RegisterName::A , 4));

    //SUB
    map.insert(0x90, |cpu| sub_rr(cpu, RegisterName::A, RegisterName::B , 4));
    map.insert(0x91, |cpu| sub_rr(cpu, RegisterName::A, RegisterName::C , 4));
    map.insert(0x92, |cpu| sub_rr(cpu, RegisterName::A, RegisterName::D , 4));
    map.insert(0x93, |cpu| sub_rr(cpu, RegisterName::A, RegisterName::E , 4));
    map.insert(0x94, |cpu| sub_rr(cpu, RegisterName::A, RegisterName::H , 4));
    map.insert(0x95, |cpu| sub_rr(cpu, RegisterName::A, RegisterName::L , 4));
    map.insert(0x96, |cpu| sub_rm(cpu, RegisterName::A, RegisterName::HL , 8));
    map.insert(0x97, |cpu| sub_rr(cpu, RegisterName::A, RegisterName::A , 4));

    //SBC
    map.insert(0x98, |cpu| sbc_rr(cpu, RegisterName::A, RegisterName::B , 4));
    map.insert(0x99, |cpu| sbc_rr(cpu, RegisterName::A, RegisterName::C , 4));
    map.insert(0x9A, |cpu| sbc_rr(cpu, RegisterName::A, RegisterName::D , 4));
    map.insert(0x9B, |cpu| sbc_rr(cpu, RegisterName::A, RegisterName::E , 4));
    map.insert(0x9C, |cpu| sbc_rr(cpu, RegisterName::A, RegisterName::H , 4));
    map.insert(0x9D, |cpu| sbc_rr(cpu, RegisterName::A, RegisterName::L , 4));
    map.insert(0x9E, |cpu| sbc_rm(cpu, RegisterName::A, RegisterName::HL , 8));
    map.insert(0x9F, |cpu| sbc_rr(cpu, RegisterName::A, RegisterName::A , 4));

    //AND
    map.insert(0xA0, |cpu| and_rr(cpu, RegisterName::A, RegisterName::B , 4));
    map.insert(0xA1, |cpu| and_rr(cpu, RegisterName::A, RegisterName::C , 4));
    map.insert(0xA2, |cpu| and_rr(cpu, RegisterName::A, RegisterName::D , 4));
    map.insert(0xA3, |cpu| and_rr(cpu, RegisterName::A, RegisterName::E , 4));
    map.insert(0xA4, |cpu| and_rr(cpu, RegisterName::A, RegisterName::H , 4));
    map.insert(0xA5, |cpu| and_rr(cpu, RegisterName::A, RegisterName::L , 4));
    map.insert(0xA6, |cpu| and_rm(cpu, RegisterName::A, RegisterName::HL , 8));
    map.insert(0xA7, |cpu| and_rr(cpu, RegisterName::A, RegisterName::A , 4));

    //XOR
    map.insert(0xA8, |cpu| xor_rr(cpu, RegisterName::A, RegisterName::B , 4));
    map.insert(0xA9, |cpu| xor_rr(cpu, RegisterName::A, RegisterName::C , 4));
    map.insert(0xAA, |cpu| xor_rr(cpu, RegisterName::A, RegisterName::D , 4));
    map.insert(0xAB, |cpu| xor_rr(cpu, RegisterName::A, RegisterName::E , 4));
    map.insert(0xAC, |cpu| xor_rr(cpu, RegisterName::A, RegisterName::H , 4));
    map.insert(0xAD, |cpu| xor_rr(cpu, RegisterName::A, RegisterName::L , 4));
    map.insert(0xAE, |cpu| xor_rm(cpu, RegisterName::A, RegisterName::HL , 8));
    map.insert(0xAF, |cpu| xor_rr(cpu, RegisterName::A, RegisterName::A , 4));


    //OR
    map.insert(0xB0, |cpu| or_rr(cpu, RegisterName::A, RegisterName::B , 4));
    map.insert(0xB1, |cpu| or_rr(cpu, RegisterName::A, RegisterName::C , 4));
    map.insert(0xB2, |cpu| or_rr(cpu, RegisterName::A, RegisterName::D , 4));
    map.insert(0xB3, |cpu| or_rr(cpu, RegisterName::A, RegisterName::E , 4));
    map.insert(0xB4, |cpu| or_rr(cpu, RegisterName::A, RegisterName::H , 4));
    map.insert(0xB5, |cpu| or_rr(cpu, RegisterName::A, RegisterName::L , 4));
    map.insert(0xB6, |cpu| or_rm(cpu, RegisterName::A, RegisterName::HL , 8));
    map.insert(0xB7, |cpu| or_rr(cpu, RegisterName::A, RegisterName::A , 4));

    //CP
    map.insert(0xB8, |cpu| cp_rr(cpu, RegisterName::A, RegisterName::B , 4));
    map.insert(0xB9, |cpu| cp_rr(cpu, RegisterName::A, RegisterName::C , 4));
    map.insert(0xBA, |cpu| cp_rr(cpu, RegisterName::A, RegisterName::D , 4));
    map.insert(0xBB, |cpu| cp_rr(cpu, RegisterName::A, RegisterName::E , 4));
    map.insert(0xBC, |cpu| cp_rr(cpu, RegisterName::A, RegisterName::H , 4));
    map.insert(0xBD, |cpu| cp_rr(cpu, RegisterName::A, RegisterName::L , 4));
    map.insert(0xBE, |cpu| cp_rm(cpu, RegisterName::A, RegisterName::HL , 8));
    map.insert(0xBF, |cpu| cp_rr(cpu, RegisterName::A, RegisterName::A , 4));

    //0xC0
    map.insert(0xC0, |cpu| ret_if(cpu, Flag::Z, false, 8));
    map.insert(0xC1, |cpu| pop_r(cpu, RegisterName::BC, 12));
    map.insert(0xC2, |cpu| jp_if(cpu, Flag::Z, false, 12));


    map.insert(0xC4, |cpu| call_if(cpu, Flag::Z, false, 12));
    map.insert(0xC5, |cpu| push_r(cpu, RegisterName::BC, 16));
    map.insert(0xC6, |cpu| add_rv(cpu, RegisterName::A, 8));
    map.insert(0xC7, |cpu| rst(cpu, 0x0000, 16));

    //0xC8
    map.insert(0xC8, |cpu| ret_if(cpu, Flag::Z, true, 8));
    map.insert(0xC9, |cpu| ret(cpu, 16));
    map.insert(0xCA, |cpu| jp_if(cpu, Flag::Z, true, 12));
    //Prefix CB map.insert(0xCB, |cpu| cp_rr(cpu, RegisterName::A, RegisterName::E));
    map.insert(0xCC, |cpu| call_if(cpu, Flag::Z, true, 12));
    map.insert(0xCD, |cpu| call(cpu, 24));
    map.insert(0xCE, |cpu| adc_rv(cpu, RegisterName::A, 8));
    map.insert(0xCF, |cpu| rst(cpu, 0x0008, 16));

    //0xD0
    map.insert(0xD0, |cpu| ret_if(cpu, Flag::C, false,8));
    map.insert(0xD1, |cpu| pop_r(cpu, RegisterName::DE,12));
    map.insert(0xD2, |cpu| jp_if(cpu, Flag::C, false,12));
    map.insert(0xD4, |cpu| call_if(cpu, Flag::C, false,12));
    map.insert(0xD5, |cpu| push_r(cpu, RegisterName::DE,16));
    map.insert(0xD6, |cpu| sub_rv(cpu, RegisterName::A,8));
    map.insert(0xD7, |cpu| rst(cpu, 0x0010,16));

    //0xD8
    map.insert(0xD8, |cpu| ret_if(cpu, Flag::C, true,8));
    map.insert(0xD9, |cpu| reti(cpu, 16));
    map.insert(0xDA, |cpu| jp_if(cpu, Flag::C, true, 12));
    map.insert(0xDC, |cpu| call_if(cpu, Flag::C, true, 12));
    map.insert(0xDE, |cpu| sbc_rv(cpu, RegisterName::A, 8));
    map.insert(0xDF, |cpu| rst(cpu, 0x0018, 16));

    //0xE0
    map.insert(0xE0, |cpu| ld_ff00_u8_a(cpu, 12));
    map.insert(0xE1, |cpu| pop_r(cpu, RegisterName::HL, 12));
    map.insert(0xE2, |cpu| ld_ff00_c_a(cpu, 8));
    map.insert(0xE5, |cpu| push_r(cpu, RegisterName::HL, 16));
    map.insert(0xE6, |cpu| and_rv(cpu, RegisterName::A,8));
    map.insert(0xE7, |cpu| rst(cpu, 0x0020, 16));

    //0xE8
    map.insert(0xE8, |cpu| add_sp_i8(cpu, 16));
    map.insert(0xE9, |cpu| jp_r(cpu, RegisterName::HL, 4));
    map.insert(0xEA, |cpu| ld_mvr(cpu, RegisterName::A, 16));
    map.insert(0xEE, |cpu| xor_rv(cpu, RegisterName::A, 8));
    map.insert(0xEF, |cpu| rst(cpu, 0x0028, 16));


    //0xF0
    map.insert(0xF0, |cpu| ld_a_ff00_plus_u8(cpu, 12));
    map.insert(0xF1, |cpu| pop_r(cpu, RegisterName::AF, 12));
    map.insert(0xF2, |cpu| ld_a_ff00_c(cpu, 8));
    map.insert(0xF3, |cpu| di(cpu, 4));
    map.insert(0xF5, |cpu| push_r(cpu, RegisterName::AF, 16));
    map.insert(0xF6, |cpu| or_rv(cpu, RegisterName::A, 8));
    map.insert(0xF7, |cpu| rst(cpu, 0x0030, 16));

    //0xF8
    map.insert(0xF8, |cpu| ld_hl_spi8(cpu, 12));
    map.insert(0xF9, |cpu| ld_rr(cpu, RegisterName::SP, RegisterName::HL, 8));
    map.insert(0xFA, |cpu| ld_rmv(cpu, RegisterName::A, 16));
    map.insert(0xFB, |cpu| ei(cpu, 4));

    map.insert(0xFE, |cpu| cp_rv(cpu, RegisterName::A, 8));
    map.insert(0xFF, |cpu| rst(cpu, 0x0038, 16));


    map
}