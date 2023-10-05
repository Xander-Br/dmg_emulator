use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use crate::bus::{Bus, LCDSTAT_VECTOR, TIMER_VECTOR, VBLANK_VECTOR};
use crate::cpu::cb_instructions::CBOpCodeHandler;
use crate::cpu::instructions::OpCodeHandler;
use crate::cpu::registers::Registers;

mod registers;
mod function;
mod cb_function;
mod instructions;
mod cb_instructions;

pub struct CPU {
    pub registers: Registers,
    pub bus: Bus,
    pub is_halted: bool,
    cb_opcode_handler: HashMap<u8, CBOpCodeHandler>,
    opcode_handler: HashMap<u8, OpCodeHandler>,
    interrupt_enabled: bool,
    log_buffer: Vec<String>,
    log_index: usize,
}

impl CPU {
    pub fn new(data: &[u8]) -> CPU {
        let registers: Registers = Registers::new();
        let bus: Bus = Bus::new(data);
        init_log();
        CPU {
            registers: registers,
            bus: bus,
            log_buffer: Vec::new(),
            is_halted: false,
            cb_opcode_handler: cb_instructions::init_cb_opcode_handlers(),
            opcode_handler: instructions::init_opcode_handlers(),
            interrupt_enabled: true,
            log_index: 0,
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let value = self.bus.bus_read(self.registers.pc);
        self.registers.pc += 1;
        value
    }

    pub fn fetch_word(&mut self) -> u16 {
        let lsb = self.fetch_byte() as u16;
        let msb = self.fetch_byte() as u16;
        (msb << 8) | lsb
    }

    pub fn step(&mut self) -> u8 {
        /*self.log_to_buffer();
        self.log_index += 1;
        if self.log_buffer.len() > self.flush_frequency_based_on_index() {
            self.flush_log_buffer();
        }*/
        let mut instruction_byte = self.fetch_byte();

        let mut cycles: u8 = 0;

        if instruction_byte == 0xCB {
            instruction_byte = self.fetch_byte();
            if let Some(handler) = self.cb_opcode_handler.get(&instruction_byte) {
                handler(self);
                cycles = 8;
            } else {
                panic!("Prefixed opcode not implemented: {:02X}", instruction_byte);
            }
        } else {
            if let Some(handler) = self.opcode_handler.get(&instruction_byte) {
                cycles = handler(self);
            } else {
                panic!("Opcode not implemented: {:02X}", instruction_byte);
            }
        }

        self.bus.step(cycles);

        if self.bus.has_interrupt() {
            self.is_halted = false;
        }
        if !self.is_halted {

        }

        let mut interrupted = false;
        if self.interrupt_enabled {
            if self.bus.io.interrupt_enable.vblank && self.bus.io.interrupt_flag.vblank {
                interrupted = true;
                self.bus.io.interrupt_flag.vblank = false;
                self.interrupt(VBLANK_VECTOR)
            }
            if self.bus.io.interrupt_enable.lcdstat && self.bus.io.interrupt_flag.lcdstat {
                interrupted = true;
                self.bus.io.interrupt_flag.lcdstat = false;
                self.interrupt(LCDSTAT_VECTOR)
            }
            if self.bus.io.interrupt_enable.timer && self.bus.io.interrupt_flag.timer {
                interrupted = true;
                self.bus.io.interrupt_flag.timer = false;
                self.interrupt(TIMER_VECTOR)
            }
        }
        if interrupted {
            cycles += 12;
        }

        cycles
    }

    fn interrupt(&mut self, location: u16) {
        self.interrupt_enabled = false;
        self.push_stack_word(self.registers.pc);
        self.registers.pc = location;
        self.bus.step(12);
    }

    pub fn push_stack(&mut self, value: u8) {
        self.registers.sp -= 1;  // Decrement Stack Pointer
        self.bus.bus_write(self.registers.sp, value);
    }

    pub fn pop_stack(&mut self) -> u8 {
        let value = self.bus.bus_read(self.registers.sp);
        self.registers.sp += 1;  // Increment Stack Pointer
        value
    }

    pub fn push_stack_word(&mut self, value: u16) {
        let high_byte = ((value & 0xFF00) >> 8) as u8;
        let low_byte = (value & 0x00FF) as u8;

        self.push_stack(high_byte);
        self.push_stack(low_byte);
    }

    pub fn pop_stack_word(&mut self) -> u16 {
        let low_byte = self.pop_stack() as u16;
        let high_byte = self.pop_stack() as u16;

        (high_byte << 8) | low_byte
    }


    pub fn log_to_buffer(&mut self) {
        let pcmem = [
            self.bus.bus_read(self.registers.pc),
            self.bus.bus_read(self.registers.pc + 1),
            self.bus.bus_read(self.registers.pc + 2),
            self.bus.bus_read(self.registers.pc + 3),
        ];

        let log_message = format!(
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
            self.registers.a, self.registers.f, self.registers.b, self.registers.c, self.registers.d, self.registers.e, self.registers.h, self.registers.l, self.registers.sp, self.registers.pc, pcmem[0], pcmem[1], pcmem[2], pcmem[3]
        );

        //print!("{}", log_message);

        self.log_buffer.push(log_message);
    }


    fn flush_frequency_based_on_index(&self) -> usize {
        if self.log_index > 729720 {
            1000
        } else {
            1000
        }
    }

    fn flush_log_buffer(&mut self) {
        let mut file = OpenOptions::new()
            .append(true)
            .open("log.txt")
            .expect("Failed to open log.txt");

        for log_line in &self.log_buffer {
            file.write_all(log_line.as_bytes())
                .expect("Failed to write to log.txt");
        }

        self.log_buffer.clear();
    }
}

pub fn init_log() {
    File::create("log.txt").expect("Failed to create or clear log.txt");
}