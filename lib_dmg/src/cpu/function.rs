use crate::bus::Bus;
use crate::cpu::CPU;
use crate::cpu::registers::{Flag, RegisterName};

pub enum Direction {
    Increment,
    Decrement,
    None,
}

pub fn ld_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {

    if destination.is_u16_registers() && source.is_u16_registers() {
        let value = cpu.registers.value_of_u16(&source);
        cpu.registers.write_in_u16(destination, value);
        return cycles;
    }
    let value = cpu.registers.value_of(&&source);
    cpu.registers.write_in(destination, value);
    cycles
}

pub fn ld_mr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, direction: Direction, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&destination);
    let value = cpu.registers.value_of(&source);
    cpu.bus.bus_write(address, value);

    match direction {
        Direction::Increment => {
            let new_val = address.wrapping_add(1);
            cpu.registers.write_in_u16(destination, new_val);
        }
        Direction::Decrement => {
            let new_val = address.wrapping_sub(1);
            cpu.registers.write_in_u16(destination, new_val);
        }
        Direction::None => {}
    }
    cycles
}

pub fn ld_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, direction: Direction, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&source);

    let value_from_memory = cpu.bus.bus_read(address);


    cpu.registers.write_in(destination, value_from_memory);

    match direction {
        Direction::Increment => {
            let new_val = address.wrapping_add(1);
            cpu.registers.write_in_u16(source, new_val);
        }
        Direction::Decrement => {
            let new_val = address.wrapping_sub(1);
            cpu.registers.write_in_u16(source, new_val);
        }
        Direction::None => {}
    }
    cycles
}

pub fn ldi_a_mhl(cpu: &mut CPU, cycles: u8) -> u8 {
    let hl = cpu.registers.get_hl();

    let a = cpu.bus.bus_read(hl);

    cpu.registers.a = a;
    cpu.registers.set_hl(hl.wrapping_add(1));
    cycles
}


pub fn ld_ru16(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value = cpu.fetch_word();
    cpu.registers.write_in_u16(destination, value);
    cycles
}

pub fn ld_ru8(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value = cpu.fetch_byte();
    cpu.registers.write_in(destination, value);
    cycles
}


/*ADD*/
pub fn add_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    if source.is_u16_registers() {
        let value = cpu.registers.value_of_u16(&source);
        let (new_value, did_overflow) =
            cpu.registers.value_of_u16(&destination)
                .overflowing_add(value);
        let half_carry = (cpu.registers.value_of_u16(&destination) & 0xFFF) + (value & 0xFFF) > 0xFFF;
        if !destination.is_u16_registers(){
            cpu.registers.update_flag(Flag::Z, new_value == 0);
        }
        cpu.registers.update_flag(Flag::N, false);
        cpu.registers.update_flag(Flag::H, half_carry);
        cpu.registers.update_flag(Flag::C, did_overflow);
        cpu.registers.write_in_u16(destination, new_value);
    } else {
        let value = cpu.registers.value_of(&&source);
        let (new_value, did_overflow) =
            cpu.registers.value_of(&&destination).overflowing_add(value);
        let half_carry = (cpu.registers.value_of(&&destination) & 0xF) + (value & 0xF) > 0xF;
        cpu.registers.update_flag(Flag::Z, new_value == 0);
        cpu.registers.update_flag(Flag::N, false);
        cpu.registers.update_flag(Flag::H, half_carry);
        cpu.registers.update_flag(Flag::C, did_overflow);
        cpu.registers.write_in(destination, new_value);
    }
    cycles
}

pub fn add_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&source);
    let value_from_memory = cpu.bus.bus_read(address);

    let (new_value, did_overflow) =
        cpu.registers.value_of(&&destination).overflowing_add(value_from_memory);
    let half_carry = (cpu.registers.value_of(&&destination) & 0xF) + (value_from_memory & 0xF) > 0xF;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, did_overflow);
    cpu.registers.write_in(destination, new_value);

    cycles
}

pub fn add_rv(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value_from_memory = cpu.fetch_byte();
    let (new_value, did_overflow) =
        cpu.registers.value_of(&&destination).overflowing_add(value_from_memory);
    let half_carry = (cpu.registers.value_of(&destination) & 0xF) + (value_from_memory & 0xF) > 0xF;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, did_overflow);
    cpu.registers.write_in(destination, new_value);
    cycles
}
/*ADC*/
pub fn adc_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let value = cpu.registers.value_of(&&source);
    let carry = if cpu.registers.check_flag(Flag::C) { 1 } else { 0 };
    let (intermediate_value, overflow1) = cpu.registers.value_of(&&destination).overflowing_add(value);
    let (new_value, overflow2) = intermediate_value.overflowing_add(carry);

    let half_carry = ((cpu.registers.value_of(&&destination) & 0xF) + (value & 0xF) + carry) > 0xF;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, overflow1 || overflow2);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn adc_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&source);
    let value = cpu.bus.bus_read(address);
    let carry = if cpu.registers.check_flag(Flag::C) { 1 } else { 0 };
    let (intermediate_value, overflow1) = cpu.registers.value_of(&&destination).overflowing_add(value);
    let (new_value, overflow2) = intermediate_value.overflowing_add(carry);

    let half_carry = ((cpu.registers.value_of(&&destination) & 0xF) + (value & 0xF) + carry) > 0xF;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, overflow1 || overflow2);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn adc_rv(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value_from_memory = cpu.fetch_byte();
    let carry = if cpu.registers.check_flag(Flag::C) { 1 } else { 0 };
    let (intermediate_result, overflow1) = cpu.registers.value_of(&destination).overflowing_add(value_from_memory);
    let (new_value, overflow2) = intermediate_result.overflowing_add(carry);

    let half_carry = (cpu.registers.value_of(&destination) & 0xF) + (value_from_memory & 0xF) + carry > 0xF;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, overflow1 || overflow2);
    cpu.registers.write_in(destination, new_value);
    cycles
}


/*SUB*/
pub fn sub_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    if source.is_u16_registers() {
        let value = cpu.registers.value_of_u16(&source);
        let (new_value, did_underflow) =
            cpu.registers.value_of_u16(&destination)
                .overflowing_sub(value);
        let half_carry = (cpu.registers.value_of_u16(&destination) & 0xFFF) < (value & 0xFFF);

        cpu.registers.update_flag(Flag::N, true);
        cpu.registers.update_flag(Flag::H, half_carry);
        cpu.registers.update_flag(Flag::C, did_underflow);

        cpu.registers.write_in_u16(destination, new_value);
    } else {
        let value = cpu.registers.value_of(&source);
        let (new_value, did_underflow) =
            cpu.registers.value_of(&destination).overflowing_sub(value);
        let half_carry = (cpu.registers.value_of(&destination) & 0xF) < (value & 0xF);

        cpu.registers.update_flag(Flag::Z, new_value == 0);
        cpu.registers.update_flag(Flag::N, true);
        cpu.registers.update_flag(Flag::H, half_carry);
        cpu.registers.update_flag(Flag::C, did_underflow);

        cpu.registers.write_in(destination, new_value);
    }
    cycles
}

pub fn sub_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&source);
    let value_from_memory = cpu.bus.bus_read(address);

    let (new_value, did_overflow) =
        cpu.registers.value_of(&destination).overflowing_sub(value_from_memory);
    let half_carry = (cpu.registers.value_of(&destination) & 0xF) < (value_from_memory & 0xF);

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, did_overflow);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn sub_rv(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value_from_memory = cpu.fetch_byte();

    let (new_value, did_underflow) =
        cpu.registers.value_of(&destination).overflowing_sub(value_from_memory);

    let half_borrow = (cpu.registers.value_of(&destination) & 0xF) < (value_from_memory & 0xF);

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, true); // subtraction always sets the N flag
    cpu.registers.update_flag(Flag::H, half_borrow);
    cpu.registers.update_flag(Flag::C, did_underflow);
    cpu.registers.write_in(destination, new_value);
    cycles
}


/*SBC*/
pub fn sbc_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let value = cpu.registers.value_of(&source);
    let borrow = if cpu.registers.check_flag(Flag::C) { 1 } else { 0 };
    let (intermediate_value, underflow1) = cpu.registers.value_of(&destination).overflowing_sub(value);
    let (new_value, underflow2) = intermediate_value.overflowing_sub(borrow);

    let half_carry = ((cpu.registers.value_of(&destination) & 0xF) < (value & 0xF) + borrow);

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, underflow1 || underflow2);
    cpu.registers.write_in(destination, new_value);

    cycles
}

pub fn sbc_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8{
    let address = cpu.registers.value_of_u16(&source);
    let value_from_memory = cpu.bus.bus_read(address);
    let borrow = if cpu.registers.check_flag(Flag::C) { 1 } else { 0 };
    let (intermediate_value, underflow1) = cpu.registers.value_of(&destination).overflowing_sub(value_from_memory);
    let (new_value, underflow2) = intermediate_value.overflowing_sub(borrow);

    let half_carry = ((cpu.registers.value_of(&destination) & 0xF) < (value_from_memory & 0xF) + borrow);

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, underflow1 || underflow2);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn sbc_rv(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value_from_memory = cpu.fetch_byte();
    let carry = if cpu.registers.check_flag(Flag::C) { 1 } else { 0 };
    let (intermediate_result, overflow1) = cpu.registers.value_of(&destination).overflowing_sub(value_from_memory);
    let (new_value, overflow2) = intermediate_result.overflowing_sub(carry);

    let half_carry = ((cpu.registers.value_of(&destination) & 0xF) as isize) - ((value_from_memory & 0xF) as isize) - (carry as isize) < 0;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, overflow1 || overflow2);
    cpu.registers.write_in(destination, new_value);
    cycles
}


/*AND*/
pub fn and_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let value = cpu.registers.value_of(&source);
    let new_value = cpu.registers.value_of(&destination) & value;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, true);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn and_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&source);
    let value_from_memory = cpu.bus.bus_read(address);
    let new_value = cpu.registers.value_of(&destination) & value_from_memory;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, true);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn and_rv(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value_from_memory = cpu.fetch_byte();
    let new_value = cpu.registers.value_of(&destination) & value_from_memory;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, true);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}


/*XOR*/
pub fn xor_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let value = cpu.registers.value_of(&source);
    let new_value = cpu.registers.value_of(&destination) ^ value;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn xor_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&source);
    let value_from_memory = cpu.bus.bus_read(address);
    let new_value = cpu.registers.value_of(&destination) ^ value_from_memory;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn xor_rv(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value_from_memory = cpu.fetch_byte();
    let new_value = cpu.registers.value_of(&destination) ^ value_from_memory;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}

/*OR*/
pub fn or_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let value = cpu.registers.value_of(&source);
    let new_value = cpu.registers.value_of(&destination) | value;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn or_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&source);
    let value_from_memory = cpu.bus.bus_read(address);
    let new_value = cpu.registers.value_of(&destination) | value_from_memory;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}

pub fn or_rv(cpu: &mut CPU, destination: RegisterName, cycles: u8) -> u8 {
    let value_from_memory = cpu.fetch_byte();
    let new_value = cpu.registers.value_of(&destination) | value_from_memory;

    cpu.registers.update_flag(Flag::Z, new_value == 0);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cpu.registers.update_flag(Flag::C, false);
    cpu.registers.write_in(destination, new_value);
    cycles
}

/*CP*/
pub fn cp_rr(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let value = cpu.registers.value_of(&source);
    let (result, did_underflow) = cpu.registers.value_of(&destination).overflowing_sub(value);
    let half_carry = (cpu.registers.value_of(&destination) & 0xF) < (value & 0xF);

    cpu.registers.update_flag(Flag::Z, result == 0);
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, did_underflow);
    cycles
}

pub fn cp_rm(cpu: &mut CPU, destination: RegisterName, source: RegisterName, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&source);
    let value_from_memory = cpu.bus.bus_read(address);
    let (result, did_underflow) = cpu.registers.value_of(&destination).overflowing_sub(value_from_memory);
    let half_carry = (cpu.registers.value_of(&destination) & 0xF) < (value_from_memory & 0xF);

    cpu.registers.update_flag(Flag::Z, result == 0);
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, did_underflow);
    cycles
}

pub fn cp_rv(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    let value_from_memory = cpu.fetch_byte();
    let register_value = cpu.registers.value_of(&register);

    let half_carry = (register_value & 0xF) < (value_from_memory & 0xF);
    let (result, did_overflow) = register_value.overflowing_sub(value_from_memory);

    cpu.registers.update_flag(Flag::Z, result == 0);
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, did_overflow);
    cycles
}

pub fn ld_m_u8(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    let value = cpu.fetch_byte();

    let address = cpu.registers.value_of_u16(&register);

    cpu.bus.bus_write(address, value);

    cycles
}

pub fn ret_if(cpu: &mut CPU, flag: Flag, condition: bool, cycles: u8) -> u8 {
    if cpu.registers.check_flag(flag) == condition {
        cpu.registers.pc = cpu.pop_stack_word();
    }
    cycles
}

pub fn ret(cpu: &mut CPU, cycles: u8) -> u8 {
    cpu.registers.pc = cpu.pop_stack_word();
    cycles
}

pub fn jr_if(cpu: &mut CPU, flag: Flag, expected_state: bool, cycles: u8) -> u8 {
    let relative_jump: i8 = cpu.bus.bus_read(cpu.registers.pc) as i8;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    if cpu.registers.check_flag(flag) == expected_state {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(relative_jump as u16);
    }
    cycles
}

pub fn jr(cpu: &mut CPU, cycles: u8) -> u8 {
    let relative_jump: i8 = cpu.bus.bus_read(cpu.registers.pc) as i8;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    cpu.registers.pc = cpu.registers.pc.wrapping_add(relative_jump as u16);
    cycles
}

pub fn rlca(cpu: &mut CPU, cycles: u8) -> u8 {
    let a = cpu.registers.a;

    let msb_set = a & 0x80 != 0;

    cpu.registers.a = (a << 1) | (msb_set as u8);

    cpu.registers.update_flag(Flag::C, msb_set);

    cpu.registers.update_flag(Flag::Z, false);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cycles
}

pub fn rla(cpu: &mut CPU, cycles: u8) -> u8{
    let a = cpu.registers.a;
    let carry_set = cpu.registers.check_flag(Flag::C);

    cpu.registers.update_flag(Flag::C, a & 0x80 != 0);

    cpu.registers.a = (a << 1) | (carry_set as u8);

    cpu.registers.update_flag(Flag::Z, false);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cycles
}

pub fn rra(cpu: &mut CPU, cycles: u8) -> u8 {
    let a = cpu.registers.a;
    let carry_set = cpu.registers.check_flag(Flag::C);

    cpu.registers.update_flag(Flag::C, a & 0x01 != 0);

    cpu.registers.a = (a >> 1) | ((carry_set as u8) << 7);

    cpu.registers.update_flag(Flag::Z, false);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cycles
}

pub fn rrca(cpu: &mut CPU, cycles: u8) -> u8 {
    let a = cpu.registers.a;

    let lsb_set = a & 0x01 != 0;

    cpu.registers.a = (a >> 1) | ((lsb_set as u8) << 7);

    cpu.registers.update_flag(Flag::C, lsb_set);

    cpu.registers.update_flag(Flag::Z, false);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cycles
}

pub fn daa(cpu: &mut CPU, cycles: u8) -> u8 {
    let n_flag = cpu.registers.check_flag(Flag::N);
    let mut c_flag = cpu.registers.check_flag(Flag::C);
    let h_flag = cpu.registers.check_flag(Flag::H);
    let mut a = cpu.registers.a;

    if !n_flag {
        if c_flag || a > 0x99 {
            a = a.wrapping_add(0x60);
            c_flag = true;
        }
        if h_flag || (a & 0x0F) > 0x09 {
            a = a.wrapping_add(0x06);
        }
    } else {
        if c_flag {
            a = a.wrapping_sub(0x60);
        }
        if h_flag {
            a = a.wrapping_sub(0x06);
        }
    }

    // these flags are always updated
    let z_flag = a == 0;
    cpu.registers.update_flag(Flag::Z, z_flag);
    cpu.registers.update_flag(Flag::C, c_flag);
    cpu.registers.update_flag(Flag::H, false);

    cpu.registers.a = a;

    cycles
}



pub fn cpl(cpu: &mut CPU, cycles: u8) -> u8 {
    cpu.registers.a = !cpu.registers.a;
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, true);
    cycles
}

pub fn scf(cpu: &mut CPU, cycles: u8) -> u8 {
    cpu.registers.update_flag(Flag::C, true);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cycles
}

pub fn ccf(cpu: &mut CPU, cycles: u8) -> u8 {
    let carry = cpu.registers.check_flag(Flag::C);
    cpu.registers.update_flag(Flag::C, !carry);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, false);
    cycles
}

/*INC*/
pub fn inc_r(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {

    if register.is_u16_registers() {
        let value = cpu.registers.value_of_u16(&register);
        let new_value = value.wrapping_add(1);
        cpu.registers.write_in_u16(register, new_value);

    } else {
        let value = cpu.registers.value_of(&register);
        let new_value = value.wrapping_add(1);

        cpu.registers.update_flag(Flag::Z, new_value == 0);
        cpu.registers.update_flag(Flag::H, (value & 0xF) + 1 > 0xF);
        cpu.registers.update_flag(Flag::N, false);
        cpu.registers.write_in(register, new_value);
    }
    cycles
}

pub fn inc_m(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    if !register.is_u16_registers() {
        panic!("Trying to use a non-16-bit register as a memory address in inc_m!");
    }

    let address = cpu.registers.value_of_u16(&register);
    let value_from_memory = cpu.bus.bus_read(address);
    let incremented_value = value_from_memory.wrapping_add(1);

    cpu.registers.update_flag(Flag::Z, incremented_value == 0);
    cpu.registers.update_flag(Flag::H, (value_from_memory & 0xF) + 1 > 0xF);
    cpu.registers.update_flag(Flag::N, false);
    cpu.bus.bus_write(address, incremented_value);
    cycles
}

pub fn dec_r(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {

    if register.is_u16_registers() {
        let value = cpu.registers.value_of_u16(&register);
        let new_value = value.wrapping_sub(1);
        cpu.registers.write_in_u16(register, new_value);

    } else {
        let value = cpu.registers.value_of(&register);
        let new_value = value.wrapping_sub(1);
        cpu.registers.write_in(register, new_value);


        cpu.registers.update_flag(Flag::Z, new_value == 0);
        cpu.registers.update_flag(Flag::N, true);
        cpu.registers.update_flag(Flag::H, (value & 0x0F) < (new_value & 0x0F));
    }
    cycles
}

pub fn dec_m(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    if !register.is_u16_registers() {
        panic!("Trying to use a non-16-bit register as a memory address in dec_m!");
    }

    let address = cpu.registers.value_of_u16(&register);
    let value_from_memory = cpu.bus.bus_read(address);
    let decremented_value = value_from_memory.wrapping_sub(1);

    cpu.registers.update_flag(Flag::Z, decremented_value == 0);
    cpu.registers.update_flag(Flag::N, true);
    cpu.registers.update_flag(Flag::H, (value_from_memory & 0x0F) == 0);


    cpu.bus.bus_write(address, decremented_value);
    cycles
}


pub fn ld_u16_sp(cpu: &mut CPU, cycles: u8) -> u8 {
    let address = cpu.fetch_word();

    cpu.bus.bus_write_word(address, cpu.registers.sp);

    cycles
}

pub fn pop_r(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    let popped_value = cpu.pop_stack_word();
    cpu.registers.write_in_u16(register, popped_value);

    if register == RegisterName::AF {
        let f_register = (popped_value & 0x00FF) as u8; // Extract the lower byte (F register)

        // Mask out the lower 4 bits
        let masked_f = f_register & 0xF0;

        cpu.registers.write_in(RegisterName::F, masked_f);

        // Update flags based on the bits in the F register
        cpu.registers.update_flag(Flag::Z, (masked_f & 0x80) != 0);
        cpu.registers.update_flag(Flag::N, (masked_f & 0x40) != 0);
        cpu.registers.update_flag(Flag::H, (masked_f & 0x20) != 0);
        cpu.registers.update_flag(Flag::C, (masked_f & 0x10) != 0);
    }

    cycles
}



pub fn push_r(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    let value = cpu.registers.value_of_u16(&register);
    cpu.push_stack_word(value);
    cycles
}

pub fn jp_if(cpu: &mut CPU, flag: Flag, condition: bool, cycles: u8) -> u8{
    if cpu.registers.check_flag(flag) == condition {
        let address = cpu.bus.bus_read_word(cpu.registers.pc);
        cpu.registers.pc = address;
    }else{
        cpu.registers.pc += 2;
    }
    cycles
}

pub fn jp(cpu: &mut CPU, cycles: u8) -> u8 {
    let address = cpu.bus.bus_read_word(cpu.registers.pc);
    cpu.registers.pc = address;
    cycles
}

pub fn jp_r(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    let address = cpu.registers.value_of_u16(&register);
    cpu.registers.pc = address;
    cycles
}

pub fn call_if(cpu: &mut CPU, flag: Flag, condition: bool, cycles: u8) -> u8 {
    if cpu.registers.check_flag(flag) == condition {
        let return_address = cpu.registers.pc + 2;
        let address = cpu.bus.bus_read_word(cpu.registers.pc);
        cpu.push_stack_word(return_address);
        cpu.registers.pc = address;
    }else {
        cpu.registers.pc += 2;
    }

    cycles
}

pub fn call(cpu: &mut CPU, cycles: u8) -> u8 {
    let return_address = cpu.registers.pc + 2;
    let address = cpu.bus.bus_read_word(cpu.registers.pc);
    cpu.push_stack_word(return_address);
    cpu.registers.pc = address;
    cycles
}

pub fn rst(cpu: &mut CPU, address: u16, cycles: u8) -> u8 {
    cpu.push_stack_word(cpu.registers.pc);

    cpu.registers.pc = address;
    cycles
}

pub fn reti(cpu: &mut CPU, cycles: u8) -> u8 {
    let return_address = cpu.pop_stack_word();

    cpu.registers.pc = return_address;

    ei(cpu, 4);
    cycles
}

pub fn ld_ff00_u8_a(cpu: &mut CPU, cycles: u8) -> u8 {
    let offset = cpu.fetch_byte();

    let address = 0xFF00u16.wrapping_add(offset as u16);

    let value = cpu.registers.value_of(&RegisterName::A);

    cpu.bus.bus_write(address, value);

    cycles
}


pub fn ld_a_ff00_plus_u8(cpu: &mut CPU, cycles: u8) -> u8 {
    // Fetch the immediate byte from the next memory location.
    let offset = cpu.fetch_byte();

    // Calculate the address: base address 0xFF00 + fetched byte.
    let address = 0xFF00u16.wrapping_add(offset as u16);


    // Read the byte from the calculated memory address.
    let value = cpu.bus.bus_read(address);


    // Store the read byte value into the A register.
    cpu.registers.write_in(RegisterName::A, value);
    cycles
}

pub fn ld_ff00_c_a(cpu: &mut CPU, cycles: u8) -> u8 {
    let c_value = cpu.registers.value_of(&RegisterName::C);

    let address = 0xFF00 + c_value as u16;

    let value = cpu.registers.value_of(&RegisterName::A);

    cpu.bus.bus_write(address, value);

    cycles
}

pub fn ld_a_ff00_c(cpu: &mut CPU, cycles: u8) -> u8 {
    let c_value = cpu.registers.value_of(&RegisterName::C);

    let address = 0xFF00 + c_value as u16;

    let value = cpu.bus.bus_read(address);

    cpu.registers.write_in(RegisterName::A, value);

    cycles
}

pub fn ld_u16_a(cpu: &mut CPU, cycles: u8) -> u8 {
    let a_value = cpu.registers.value_of(&RegisterName::A);

    let address = cpu.bus.bus_read_word(cpu.registers.pc);

    cpu.bus.bus_write(address, a_value);

    cpu.registers.pc += 2;

    cycles
}

pub fn add_sp_i8(cpu: &mut CPU, cycles: u8) -> u8 {
    let i8_value = cpu.fetch_byte() as i8;
    let i16_value = i8_value as i16 as u16;

    let result = cpu.registers.sp as u32 + i16_value as u32;

    let half_carry = (cpu.registers.sp & 0x0F) + (i16_value & 0x0F) > 0x0F;

    let full_carry = (cpu.registers.sp & 0xFF) + (i16_value & 0xFF) > 0xFF;

    cpu.registers.sp = cpu.registers.sp.wrapping_add(i16_value);

    cpu.registers.update_flag(Flag::Z, false);
    cpu.registers.update_flag(Flag::N, false);
    cpu.registers.update_flag(Flag::H, half_carry);
    cpu.registers.update_flag(Flag::C, full_carry);

    cycles
}


pub fn ld_mvr(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    let address = cpu.bus.bus_read_word(cpu.registers.pc);

    let value_a = cpu.registers.value_of(&register);

    cpu.bus.bus_write(address, value_a);

    cpu.registers.pc += 2;

    cycles
}

pub fn di(cpu: &mut CPU, cycles: u8) -> u8 {
    cpu.interrupt_enabled = false;

    cycles
}

pub fn ei(cpu: &mut CPU, cycles: u8) -> u8 {
    cpu.interrupt_enabled = true;

    cycles
}

pub fn ld_hl_spi8(cpu: &mut CPU, cycles: u8) -> u8 {
    cpu.registers.clear_all_flags();

    let sp = cpu.registers.sp as u16;
    let value = cpu.fetch_byte() as i8;
    let extended_value = value as u16; // If the Rust environment correctly sign-extends, otherwise use manual sign extension.
    let result = sp.wrapping_add(extended_value);

    if (sp & 0xFF) + (extended_value & 0xFF) > 0xFF {
        cpu.registers.set_flag(Flag::C);
    } else {
        cpu.registers.clear_flag(Flag::C);
    }

    if (sp & 0x0F) + (extended_value & 0x0F) > 0x0F {
        cpu.registers.set_flag(Flag::H);
    } else {
        cpu.registers.clear_flag(Flag::H);
    }

    cpu.registers.set_hl(result);
    cycles
}




pub fn ld_rmv(cpu: &mut CPU, register: RegisterName, cycles: u8) -> u8 {
    let address = cpu.bus.bus_read_word(cpu.registers.pc);

    let value_from_address = cpu.bus.bus_read(address);
    cpu.registers.a = value_from_address;

    cpu.registers.pc += 2;
    cycles
}

pub fn halt(cpu: &mut CPU, cycles: u8) -> u8 {
    cpu.is_halted = true;
    cycles
}
