use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::registers::{Flag, RegisterName};

pub fn rlc(cpu: &mut CPU, register: RegisterName) {
    let mut value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    let carry_set = value & 0x80 != 0;

    value = (value << 1) | (if carry_set { 1 } else { 0 });

    if register.is_u16_registers() {
        cpu.bus.bus_write(cpu.registers.value_of_u16(&register), value);
    } else {
        cpu.registers.write_in(register, value);
    }

    cpu.registers.update_flag(Flag::C, carry_set);
    cpu.registers.update_flag(Flag::Z, value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
}

pub fn rrc(cpu: &mut CPU, register: RegisterName) {
    let mut value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    let carry_set = value & 0x01 != 0;

    value = (value >> 1) | (if carry_set { 0x80 } else { 0 });

    if register.is_u16_registers() {
        cpu.bus.bus_write(cpu.registers.value_of_u16(&register), value);
    } else {
        cpu.registers.write_in(register, value);
    }

    cpu.registers.update_flag(Flag::C, carry_set); // Set carry flag if LSB was 1
    cpu.registers.update_flag(Flag::Z, value == 0); // Set zero flag if result is zero
    cpu.registers.update_flag(Flag::N, false); // Reset subtract flag
    cpu.registers.update_flag(Flag::H, false); // Reset half-carry flag
}

pub fn rl(cpu: &mut CPU, register: RegisterName) {
    let mut value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    let old_carry = cpu.registers.check_flag(Flag::C);
    let new_carry = (value & 0x80) != 0;

    value = (value << 1) | (if old_carry { 1 } else { 0 });

    if register.is_u16_registers() {
        cpu.bus.bus_write(cpu.registers.value_of_u16(&register), value);
    } else {
        cpu.registers.write_in(register, value);
    }

    // Set or reset flags
    cpu.registers.update_flag(Flag::C, new_carry);
    cpu.registers.update_flag(Flag::Z, value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
}

pub fn rr(cpu: &mut CPU, register: RegisterName) {
    let mut value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    let new_carry = (value & 0x01) != 0;

    value = (value >> 1) | (if cpu.registers.check_flag(Flag::C) { 0x80 } else { 0 });

    if register.is_u16_registers() {
        cpu.bus.bus_write(cpu.registers.value_of_u16(&register), value);
    } else {
        cpu.registers.write_in(register, value);
    }

    // Set or reset flags
    cpu.registers.update_flag(Flag::C, new_carry);
    cpu.registers.update_flag(Flag::Z, value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
}

pub fn sla(cpu: &mut CPU, register: RegisterName) {
    let mut value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    // Check if the most significant bit is set (will go into carry)
    let new_carry = (value & 0x80) != 0;

    // Shift the value to the left
    value <<= 1;

    if register.is_u16_registers() {
        cpu.bus.bus_write(cpu.registers.value_of_u16(&register), value);
    } else {
        cpu.registers.write_in(register, value);
    }

    // Update the flags
    cpu.registers.update_flag(Flag::C, new_carry);
    cpu.registers.update_flag(Flag::Z, value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
}

pub fn sra(cpu: &mut CPU, register: RegisterName) {
    let mut value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    // Check if the least significant bit is set (will go into carry)
    let new_carry = (value & 0x01) != 0;

    // Keep the original Bit 7 and shift the value to the right
    let msb = value & 0x80;
    value = (value >> 1) | msb;

    if register.is_u16_registers() {
        cpu.bus.bus_write(cpu.registers.value_of_u16(&register), value);
    } else {
        cpu.registers.write_in(register, value);
    }

    // Update the flags
    cpu.registers.update_flag(Flag::C, new_carry);
    cpu.registers.update_flag(Flag::Z, value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
}


pub fn swap(cpu: &mut CPU, register: RegisterName) {
    let mut value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    let lower_nibble = value & 0x0F;
    let upper_nibble = (value & 0xF0) >> 4;

    value = (lower_nibble << 4) | upper_nibble;

    if register.is_u16_registers() {
        cpu.bus.bus_write(cpu.registers.value_of_u16(&register), value);
    } else {
        cpu.registers.write_in(register, value);
    }

    // Set or reset flags as needed
    cpu.registers.update_flag(Flag::C, false); // Clear carry flag
    cpu.registers.update_flag(Flag::Z, value == 0); // Set zero flag if result is zero
    cpu.registers.update_flag(Flag::N, false); // Reset subtract flag
    cpu.registers.update_flag(Flag::H, false); // Reset half-carry flag
}


pub fn srl(cpu: &mut CPU, register: RegisterName) {
    let mut value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    let carry_set = (value & 0x01) != 0;

    value >>= 1;

    if register.is_u16_registers() {
        cpu.bus.bus_write(cpu.registers.value_of_u16(&register), value);
    } else {
        cpu.registers.write_in(register, value);
    }

    // Set or reset flags as needed
    cpu.registers.update_flag(Flag::C, carry_set); // Set carry flag if LSB was 1
    cpu.registers.update_flag(Flag::Z, value == 0); // Set zero flag if result is zero
    cpu.registers.update_flag(Flag::N, false); // Reset subtract flag
    cpu.registers.update_flag(Flag::H, false); // Reset half-carry flag
}

pub fn bit(cpu: &mut CPU, bit: u8, register: RegisterName) {
    assert!(bit <= 7, "Bit number should be between 0 and 7 inclusive");

    let value: u8;

    if register.is_u16_registers() {
        value = cpu.bus.bus_read(cpu.registers.value_of_u16(&register));
    } else {
        value = cpu.registers.value_of(&register);
    }

    // Check if the specified bit is set
    let is_set = (value & (1 << bit)) != 0;

    // Update the flags
    cpu.registers.update_flag(Flag::Z, !is_set); // Z is set if bit is 0, reset if 1
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, true);
    // Note: The C flag is not affected by the BIT instruction
}

pub fn res(cpu: &mut CPU, bit: u8, register: RegisterName) {
    assert!(bit <= 7, "Bit number should be between 0 and 7 inclusive");

    let mask = !(1 << bit);  // Generate a mask to reset the specified bit

    if register.is_u16_registers() {
        // If register points to memory (is a 16-bit register)
        let addr = cpu.registers.value_of_u16(&register);
        let value = cpu.bus.bus_read(addr);
        cpu.bus.bus_write(addr, value & mask);
    } else {
        // If register is a standard 8-bit register
        let value = cpu.registers.value_of(&register);
        cpu.registers.write_in(register, value & mask);
    }
}

pub fn set(cpu: &mut CPU, bit: u8, register: RegisterName) {
    assert!(bit <= 7, "Bit number should be between 0 and 7 inclusive");

    let mask = 1 << bit;  // Generate a mask to set the specified bit

    if register.is_u16_registers() {
        // If register points to memory (is a 16-bit register)
        let addr = cpu.registers.value_of_u16(&register);
        let value = cpu.bus.bus_read(addr);
        cpu.bus.bus_write(addr, value | mask);
    } else {
        // If register is a standard 8-bit register
        let value = cpu.registers.value_of(&register);
        cpu.registers.write_in(register, value | mask);
    }
}


