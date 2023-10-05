use std::collections::HashMap;
use crate::bus::Bus;
use crate::cpu::cb_function::*;
use crate::cpu::CPU;
use crate::cpu::instructions::OpCodeHandler;
use crate::cpu::registers::RegisterName;


pub type CBOpCodeHandler = fn(&mut CPU);


pub fn init_cb_opcode_handlers() -> HashMap<u8, CBOpCodeHandler> {
    let mut map: HashMap<u8, CBOpCodeHandler> = HashMap::new();


    map.insert(0x00, |cpu| rlc(cpu, RegisterName::B));
    map.insert(0x01, |cpu| rlc(cpu, RegisterName::C));
    map.insert(0x02, |cpu| rlc(cpu, RegisterName::D));
    map.insert(0x03, |cpu| rlc(cpu, RegisterName::E));
    map.insert(0x04, |cpu| rlc(cpu, RegisterName::H));
    map.insert(0x05, |cpu| rlc(cpu, RegisterName::L));
    map.insert(0x06, |cpu| rlc(cpu, RegisterName::HL));
    map.insert(0x07, |cpu| rlc(cpu, RegisterName::A));

    map.insert(0x08, |cpu| rrc(cpu, RegisterName::B));
    map.insert(0x09, |cpu| rrc(cpu, RegisterName::C));
    map.insert(0x0A, |cpu| rrc(cpu, RegisterName::D));
    map.insert(0x0B, |cpu| rrc(cpu, RegisterName::E));
    map.insert(0x0C, |cpu| rrc(cpu, RegisterName::H));
    map.insert(0x0D, |cpu| rrc(cpu, RegisterName::L));
    map.insert(0x0E, |cpu| rrc(cpu, RegisterName::HL));
    map.insert(0x0F, |cpu| rrc(cpu, RegisterName::A));

    map.insert(0x10, |cpu| rl(cpu, RegisterName::B));
    map.insert(0x11, |cpu| rl(cpu, RegisterName::C));
    map.insert(0x12, |cpu| rl(cpu, RegisterName::D));
    map.insert(0x13, |cpu| rl(cpu, RegisterName::E));
    map.insert(0x14, |cpu| rl(cpu, RegisterName::H));
    map.insert(0x15, |cpu| rl(cpu, RegisterName::L));
    map.insert(0x16, |cpu| rl(cpu, RegisterName::HL));
    map.insert(0x17, |cpu| rl(cpu, RegisterName::A));

    map.insert(0x18, |cpu| rr(cpu, RegisterName::B));
    map.insert(0x19, |cpu| rr(cpu, RegisterName::C));
    map.insert(0x1A, |cpu| rr(cpu, RegisterName::D));
    map.insert(0x1B, |cpu| rr(cpu, RegisterName::E));
    map.insert(0x1C, |cpu| rr(cpu, RegisterName::H));
    map.insert(0x1D, |cpu| rr(cpu, RegisterName::L));
    map.insert(0x1E, |cpu| rr(cpu, RegisterName::HL));
    map.insert(0x1F, |cpu| rr(cpu, RegisterName::A));

    map.insert(0x20, |cpu| sla(cpu, RegisterName::B));
    map.insert(0x21, |cpu| sla(cpu, RegisterName::C));
    map.insert(0x22, |cpu| sla(cpu, RegisterName::D));
    map.insert(0x23, |cpu| sla(cpu, RegisterName::E));
    map.insert(0x24, |cpu| sla(cpu, RegisterName::H));
    map.insert(0x25, |cpu| sla(cpu, RegisterName::L));
    map.insert(0x26, |cpu| sla(cpu, RegisterName::HL));
    map.insert(0x27, |cpu| sla(cpu, RegisterName::A));

    map.insert(0x28, |cpu| sra(cpu, RegisterName::B));
    map.insert(0x29, |cpu| sra(cpu, RegisterName::C));
    map.insert(0x2A, |cpu| sra(cpu, RegisterName::D));
    map.insert(0x2B, |cpu| sra(cpu, RegisterName::E));
    map.insert(0x2C, |cpu| sra(cpu, RegisterName::H));
    map.insert(0x2D, |cpu| sra(cpu, RegisterName::L));
    map.insert(0x2E, |cpu| sra(cpu, RegisterName::HL));
    map.insert(0x2F, |cpu| sra(cpu, RegisterName::A));

    map.insert(0x30, |cpu| swap(cpu, RegisterName::B));
    map.insert(0x31, |cpu| swap(cpu, RegisterName::C));
    map.insert(0x32, |cpu| swap(cpu, RegisterName::D));
    map.insert(0x33, |cpu| swap(cpu, RegisterName::E));
    map.insert(0x34, |cpu| swap(cpu, RegisterName::H));
    map.insert(0x35, |cpu| swap(cpu, RegisterName::L));
    map.insert(0x36, |cpu| swap(cpu, RegisterName::HL));
    map.insert(0x37, |cpu| swap(cpu, RegisterName::A));

    map.insert(0x38, |cpu| srl(cpu, RegisterName::B));
    map.insert(0x39, |cpu| srl(cpu, RegisterName::C));
    map.insert(0x3A, |cpu| srl(cpu, RegisterName::D));
    map.insert(0x3B, |cpu| srl(cpu, RegisterName::E));
    map.insert(0x3C, |cpu| srl(cpu, RegisterName::H));
    map.insert(0x3D, |cpu| srl(cpu, RegisterName::L));
    map.insert(0x3E, |cpu| srl(cpu, RegisterName::HL));
    map.insert(0x3F, |cpu| srl(cpu, RegisterName::A));

    map.insert(0x40, |cpu| bit(cpu, 0, RegisterName::B));
    map.insert(0x41, |cpu| bit(cpu, 0, RegisterName::C));
    map.insert(0x42, |cpu| bit(cpu, 0, RegisterName::D));
    map.insert(0x43, |cpu| bit(cpu, 0, RegisterName::E));
    map.insert(0x44, |cpu| bit(cpu, 0, RegisterName::H));
    map.insert(0x45, |cpu| bit(cpu, 0, RegisterName::L));
    map.insert(0x46, |cpu| bit(cpu, 0, RegisterName::HL));
    map.insert(0x47, |cpu| bit(cpu, 0, RegisterName::A));

    map.insert(0x48, |cpu| bit(cpu, 1, RegisterName::B));
    map.insert(0x49, |cpu| bit(cpu, 1, RegisterName::C));
    map.insert(0x4A, |cpu| bit(cpu, 1, RegisterName::D));
    map.insert(0x4B, |cpu| bit(cpu, 1, RegisterName::E));
    map.insert(0x4C, |cpu| bit(cpu, 1, RegisterName::H));
    map.insert(0x4D, |cpu| bit(cpu, 1, RegisterName::L));
    map.insert(0x4E, |cpu| bit(cpu, 1, RegisterName::HL));
    map.insert(0x4F, |cpu| bit(cpu, 1, RegisterName::A));

    map.insert(0x50, |cpu| bit(cpu, 2, RegisterName::B));
    map.insert(0x51, |cpu| bit(cpu, 2, RegisterName::C));
    map.insert(0x52, |cpu| bit(cpu, 2, RegisterName::D));
    map.insert(0x53, |cpu| bit(cpu, 2, RegisterName::E));
    map.insert(0x54, |cpu| bit(cpu, 2, RegisterName::H));
    map.insert(0x55, |cpu| bit(cpu, 2, RegisterName::L));
    map.insert(0x56, |cpu| bit(cpu, 2, RegisterName::HL));
    map.insert(0x57, |cpu| bit(cpu, 2, RegisterName::A));

    map.insert(0x58, |cpu| bit(cpu, 3, RegisterName::B));
    map.insert(0x59, |cpu| bit(cpu, 3, RegisterName::C));
    map.insert(0x5A, |cpu| bit(cpu, 3, RegisterName::D));
    map.insert(0x5B, |cpu| bit(cpu, 3, RegisterName::E));
    map.insert(0x5C, |cpu| bit(cpu, 3, RegisterName::H));
    map.insert(0x5D, |cpu| bit(cpu, 3, RegisterName::L));
    map.insert(0x5E, |cpu| bit(cpu, 3, RegisterName::HL));
    map.insert(0x5F, |cpu| bit(cpu, 3, RegisterName::A));

    map.insert(0x60, |cpu| bit(cpu, 4, RegisterName::B));
    map.insert(0x61, |cpu| bit(cpu, 4, RegisterName::C));
    map.insert(0x62, |cpu| bit(cpu, 4, RegisterName::D));
    map.insert(0x63, |cpu| bit(cpu, 4, RegisterName::E));
    map.insert(0x64, |cpu| bit(cpu, 4, RegisterName::H));
    map.insert(0x65, |cpu| bit(cpu, 4, RegisterName::L));
    map.insert(0x66, |cpu| bit(cpu, 4, RegisterName::HL));
    map.insert(0x67, |cpu| bit(cpu, 4, RegisterName::A));

    map.insert(0x68, |cpu| bit(cpu, 5, RegisterName::B));
    map.insert(0x69, |cpu| bit(cpu, 5, RegisterName::C));
    map.insert(0x6A, |cpu| bit(cpu, 5, RegisterName::D));
    map.insert(0x6B, |cpu| bit(cpu, 5, RegisterName::E));
    map.insert(0x6C, |cpu| bit(cpu, 5, RegisterName::H));
    map.insert(0x6D, |cpu| bit(cpu, 5, RegisterName::L));
    map.insert(0x6E, |cpu| bit(cpu, 5, RegisterName::HL));
    map.insert(0x6F, |cpu| bit(cpu, 5, RegisterName::A));

    map.insert(0x70, |cpu| bit(cpu, 6, RegisterName::B));
    map.insert(0x71, |cpu| bit(cpu, 6, RegisterName::C));
    map.insert(0x72, |cpu| bit(cpu, 6, RegisterName::D));
    map.insert(0x73, |cpu| bit(cpu, 6, RegisterName::E));
    map.insert(0x74, |cpu| bit(cpu, 6, RegisterName::H));
    map.insert(0x75, |cpu| bit(cpu, 6, RegisterName::L));
    map.insert(0x76, |cpu| bit(cpu, 6, RegisterName::HL));
    map.insert(0x77, |cpu| bit(cpu, 6, RegisterName::A));

    map.insert(0x78, |cpu| bit(cpu, 7, RegisterName::B));
    map.insert(0x79, |cpu| bit(cpu, 7, RegisterName::C));
    map.insert(0x7A, |cpu| bit(cpu, 7, RegisterName::D));
    map.insert(0x7B, |cpu| bit(cpu, 7, RegisterName::E));
    map.insert(0x7C, |cpu| bit(cpu, 7, RegisterName::H));
    map.insert(0x7D, |cpu| bit(cpu, 7, RegisterName::L));
    map.insert(0x7E, |cpu| bit(cpu, 7, RegisterName::HL));
    map.insert(0x7F, |cpu| bit(cpu, 7, RegisterName::A));

    map.insert(0x80, |cpu| res(cpu, 0, RegisterName::B));
    map.insert(0x81, |cpu| res(cpu, 0, RegisterName::C));
    map.insert(0x82, |cpu| res(cpu, 0, RegisterName::D));
    map.insert(0x83, |cpu| res(cpu, 0, RegisterName::E));
    map.insert(0x84, |cpu| res(cpu, 0, RegisterName::H));
    map.insert(0x85, |cpu| res(cpu, 0, RegisterName::L));
    map.insert(0x86, |cpu| res(cpu, 0, RegisterName::HL));
    map.insert(0x87, |cpu| res(cpu, 0, RegisterName::A));
    map.insert(0x88, |cpu| res(cpu, 1, RegisterName::B));
    map.insert(0x89, |cpu| res(cpu, 1, RegisterName::C));
    map.insert(0x8A, |cpu| res(cpu, 1, RegisterName::D));
    map.insert(0x8B, |cpu| res(cpu, 1, RegisterName::E));
    map.insert(0x8C, |cpu| res(cpu, 1, RegisterName::H));
    map.insert(0x8D, |cpu| res(cpu, 1, RegisterName::L));
    map.insert(0x8E, |cpu| res(cpu, 1, RegisterName::HL));
    map.insert(0x8F, |cpu| res(cpu, 1, RegisterName::A));
    map.insert(0x90, |cpu| res(cpu, 2, RegisterName::B));
    map.insert(0x91, |cpu| res(cpu, 2, RegisterName::C));
    map.insert(0x92, |cpu| res(cpu, 2, RegisterName::D));
    map.insert(0x93, |cpu| res(cpu, 2, RegisterName::E));
    map.insert(0x94, |cpu| res(cpu, 2, RegisterName::H));
    map.insert(0x95, |cpu| res(cpu, 2, RegisterName::L));
    map.insert(0x96, |cpu| res(cpu, 2, RegisterName::HL));
    map.insert(0x97, |cpu| res(cpu, 2, RegisterName::A));
    map.insert(0x98, |cpu| res(cpu, 3, RegisterName::B));
    map.insert(0x99, |cpu| res(cpu, 3, RegisterName::C));
    map.insert(0x9A, |cpu| res(cpu, 3, RegisterName::D));
    map.insert(0x9B, |cpu| res(cpu, 3, RegisterName::E));
    map.insert(0x9C, |cpu| res(cpu, 3, RegisterName::H));
    map.insert(0x9D, |cpu| res(cpu, 3, RegisterName::L));
    map.insert(0x9E, |cpu| res(cpu, 3, RegisterName::HL));
    map.insert(0x9F, |cpu| res(cpu, 3, RegisterName::A));
    map.insert(0xA0, |cpu| res(cpu, 4, RegisterName::B));
    map.insert(0xA1, |cpu| res(cpu, 4, RegisterName::C));
    map.insert(0xA2, |cpu| res(cpu, 4, RegisterName::D));
    map.insert(0xA3, |cpu| res(cpu, 4, RegisterName::E));
    map.insert(0xA4, |cpu| res(cpu, 4, RegisterName::H));
    map.insert(0xA5, |cpu| res(cpu, 4, RegisterName::L));
    map.insert(0xA6, |cpu| res(cpu, 4, RegisterName::HL));
    map.insert(0xA7, |cpu| res(cpu, 4, RegisterName::A));
    map.insert(0xA8, |cpu| res(cpu, 5, RegisterName::B));
    map.insert(0xA9, |cpu| res(cpu, 5, RegisterName::C));
    map.insert(0xAA, |cpu| res(cpu, 5, RegisterName::D));
    map.insert(0xAB, |cpu| res(cpu, 5, RegisterName::E));
    map.insert(0xAC, |cpu| res(cpu, 5, RegisterName::H));
    map.insert(0xAD, |cpu| res(cpu, 5, RegisterName::L));
    map.insert(0xAE, |cpu| res(cpu, 5, RegisterName::HL));
    map.insert(0xAF, |cpu| res(cpu, 5, RegisterName::A));
    map.insert(0xB0, |cpu| res(cpu, 6, RegisterName::B));
    map.insert(0xB1, |cpu| res(cpu, 6, RegisterName::C));
    map.insert(0xB2, |cpu| res(cpu, 6, RegisterName::D));
    map.insert(0xB3, |cpu| res(cpu, 6, RegisterName::E));
    map.insert(0xB4, |cpu| res(cpu, 6, RegisterName::H));
    map.insert(0xB5, |cpu| res(cpu, 6, RegisterName::L));
    map.insert(0xB6, |cpu| res(cpu, 6, RegisterName::HL));
    map.insert(0xB7, |cpu| res(cpu, 6, RegisterName::A));
    map.insert(0xB8, |cpu| res(cpu, 7, RegisterName::B));
    map.insert(0xB9, |cpu| res(cpu, 7, RegisterName::C));
    map.insert(0xBA, |cpu| res(cpu, 7, RegisterName::D));
    map.insert(0xBB, |cpu| res(cpu, 7, RegisterName::E));
    map.insert(0xBC, |cpu| res(cpu, 7, RegisterName::H));
    map.insert(0xBD, |cpu| res(cpu, 7, RegisterName::L));
    map.insert(0xBE, |cpu| res(cpu, 7, RegisterName::HL));
    map.insert(0xBF, |cpu| res(cpu, 7, RegisterName::A));

    map.insert(0xC0, |cpu| set(cpu, 0, RegisterName::B));
    map.insert(0xC1, |cpu| set(cpu, 0, RegisterName::C));
    map.insert(0xC2, |cpu| set(cpu, 0, RegisterName::D));
    map.insert(0xC3, |cpu| set(cpu, 0, RegisterName::E));
    map.insert(0xC4, |cpu| set(cpu, 0, RegisterName::H));
    map.insert(0xC5, |cpu| set(cpu, 0, RegisterName::L));
    map.insert(0xC6, |cpu| set(cpu, 0, RegisterName::HL));
    map.insert(0xC7, |cpu| set(cpu, 0, RegisterName::A));
    map.insert(0xC8, |cpu| set(cpu, 1, RegisterName::B));
    map.insert(0xC9, |cpu| set(cpu, 1, RegisterName::C));
    map.insert(0xCA, |cpu| set(cpu, 1, RegisterName::D));
    map.insert(0xCB, |cpu| set(cpu, 1, RegisterName::E));
    map.insert(0xCC, |cpu| set(cpu, 1, RegisterName::H));
    map.insert(0xCD, |cpu| set(cpu, 1, RegisterName::L));
    map.insert(0xCE, |cpu| set(cpu, 1, RegisterName::HL));
    map.insert(0xCF, |cpu| set(cpu, 1, RegisterName::A));
    map.insert(0xD0, |cpu| set(cpu, 2, RegisterName::B));
    map.insert(0xD1, |cpu| set(cpu, 2, RegisterName::C));
    map.insert(0xD2, |cpu| set(cpu, 2, RegisterName::D));
    map.insert(0xD3, |cpu| set(cpu, 2, RegisterName::E));
    map.insert(0xD4, |cpu| set(cpu, 2, RegisterName::H));
    map.insert(0xD5, |cpu| set(cpu, 2, RegisterName::L));
    map.insert(0xD6, |cpu| set(cpu, 2, RegisterName::HL));
    map.insert(0xD7, |cpu| set(cpu, 2, RegisterName::A));
    map.insert(0xD8, |cpu| set(cpu, 3, RegisterName::B));
    map.insert(0xD9, |cpu| set(cpu, 3, RegisterName::C));
    map.insert(0xDA, |cpu| set(cpu, 3, RegisterName::D));
    map.insert(0xDB, |cpu| set(cpu, 3, RegisterName::E));
    map.insert(0xDC, |cpu| set(cpu, 3, RegisterName::H));
    map.insert(0xDD, |cpu| set(cpu, 3, RegisterName::L));
    map.insert(0xDE, |cpu| set(cpu, 3, RegisterName::HL));
    map.insert(0xDF, |cpu| set(cpu, 3, RegisterName::A));
    map.insert(0xE0, |cpu| set(cpu, 4, RegisterName::B));
    map.insert(0xE1, |cpu| set(cpu, 4, RegisterName::C));
    map.insert(0xE2, |cpu| set(cpu, 4, RegisterName::D));
    map.insert(0xE3, |cpu| set(cpu, 4, RegisterName::E));
    map.insert(0xE4, |cpu| set(cpu, 4, RegisterName::H));
    map.insert(0xE5, |cpu| set(cpu, 4, RegisterName::L));
    map.insert(0xE6, |cpu| set(cpu, 4, RegisterName::HL));
    map.insert(0xE7, |cpu| set(cpu, 4, RegisterName::A));
    map.insert(0xE8, |cpu| set(cpu, 5, RegisterName::B));
    map.insert(0xE9, |cpu| set(cpu, 5, RegisterName::C));
    map.insert(0xEA, |cpu| set(cpu, 5, RegisterName::D));
    map.insert(0xEB, |cpu| set(cpu, 5, RegisterName::E));
    map.insert(0xEC, |cpu| set(cpu, 5, RegisterName::H));
    map.insert(0xED, |cpu| set(cpu, 5, RegisterName::L));
    map.insert(0xEE, |cpu| set(cpu, 5, RegisterName::HL));
    map.insert(0xEF, |cpu| set(cpu, 5, RegisterName::A));
    map.insert(0xF0, |cpu| set(cpu, 6, RegisterName::B));
    map.insert(0xF1, |cpu| set(cpu, 6, RegisterName::C));
    map.insert(0xF2, |cpu| set(cpu, 6, RegisterName::D));
    map.insert(0xF3, |cpu| set(cpu, 6, RegisterName::E));
    map.insert(0xF4, |cpu| set(cpu, 6, RegisterName::H));
    map.insert(0xF5, |cpu| set(cpu, 6, RegisterName::L));
    map.insert(0xF6, |cpu| set(cpu, 6, RegisterName::HL));
    map.insert(0xF7, |cpu| set(cpu, 6, RegisterName::A));
    map.insert(0xF8, |cpu| set(cpu, 7, RegisterName::B));
    map.insert(0xF9, |cpu| set(cpu, 7, RegisterName::C));
    map.insert(0xFA, |cpu| set(cpu, 7, RegisterName::D));
    map.insert(0xFB, |cpu| set(cpu, 7, RegisterName::E));
    map.insert(0xFC, |cpu| set(cpu, 7, RegisterName::H));
    map.insert(0xFD, |cpu| set(cpu, 7, RegisterName::L));
    map.insert(0xFE, |cpu| set(cpu, 7, RegisterName::HL));
    map.insert(0xFF, |cpu| set(cpu, 7, RegisterName::A));

    map
}


