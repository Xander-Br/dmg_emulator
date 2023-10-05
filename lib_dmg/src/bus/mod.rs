use crate::cart::Cartridge;
use crate::gpu::{GPU, InterruptRequest};
use crate::io::IO;
use crate::ram::RAM;

pub const VBLANK_VECTOR: u16 = 0x40;
pub const LCDSTAT_VECTOR: u16 = 0x48;
pub const TIMER_VECTOR: u16 = 0x50;



pub struct Bus {
    cart: Cartridge,
    ram: RAM,
    pub io: IO,
    pub gpu: GPU,
}

impl Bus {
    pub fn new(data: &[u8]) -> Bus {
        Bus {
            cart: Cartridge::new(data),
            ram: RAM::new(),
            io: IO::new(),
            gpu: GPU::new(),
        }
    }

    pub fn step(&mut self, cycles: u8){
        if self.io.timer.step(cycles) {
            self.io.interrupt_flag.timer = true;
        }

        let (vblank, lcd) = match self.gpu.step(cycles) {
            InterruptRequest::Both => (true, true),
            InterruptRequest::VBlank => (true, false),
            InterruptRequest::LCDStat => (false, true),
            InterruptRequest::None => (false, false),
        };

        if vblank {
            self.io.interrupt_flag.vblank = true;
        }
        if lcd {
            self.io.interrupt_flag.lcdstat = true;
        }
    }

    pub fn bus_read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.cart.cart_read(address), // ROM Bank 00
            0x4000..=0x7FFF => self.cart.cart_read(address), // ROM Bank 01->NN
            0x8000..=0x9FFF => self.gpu.gpu_read(address), // GPU VRAM
            0xA000..=0xBFFF => self.ram.ram_read(address), // 8 KiB External RAM
            0xC000..=0xCFFF => self.ram.ram_read(address), // 4 KiB Work RAM (WRAM)
            0xD000..=0xDFFF => self.ram.ram_read(address), // 4 KiB Work RAM (WRAM)
            0xFE00..=0xFE9F => self.gpu.gpu_read(address), // Object attribute memory (OAM)
            0xFF40..=0xFF4B => self.gpu.gpu_read(address), // GPU LCD
            0xFF00..=0xFF7F => self.io.io_read(address), // I/O Registers
            0xFF80..=0xFFFE => self.ram.ram_read(address), // High ram
            0xFFFF => self.io.io_read(address), // Interrupt master
            _ => panic!("Bus read address not implemented: {:04X}", address)
        }
    }

    pub fn has_interrupt(&self) -> bool {
        (self.io.interrupt_enable.vblank && self.io.interrupt_flag.vblank)
            || (self.io.interrupt_enable.lcdstat && self.io.interrupt_flag.lcdstat)
            || (self.io.interrupt_enable.timer && self.io.interrupt_flag.timer)
            || (self.io.interrupt_enable.serial && self.io.interrupt_flag.serial)
            || (self.io.interrupt_enable.joypad && self.io.interrupt_flag.joypad)
    }

    pub fn bus_read_word(&mut self, address: u16) -> u16 {
        let lsb = self.bus_read(address) as u16;
        let msb = self.bus_read(address + 1) as u16;
        (msb << 8) | lsb
    }

    pub fn bus_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => self.cart.cart_write(address, value),// ROM Bank 00
            0x4000..=0x7FFF => self.cart.cart_write(address, value),// ROM Bank 01->NN
            0x8000..=0x9FFF => self.gpu.gpu_write(address, value), // GPU VRAM
            0xA000..=0xBFFF => self.ram.ram_write(address, value), // 8 KiB External RAM
            0xC000..=0xCFFF => self.ram.ram_write(address, value), // 4 KiB Work RAM (WRAM)
            0xD000..=0xDFFF => self.ram.ram_write(address, value), // 4 KiB Work RAM (WRAM)
            0xE000..=0xFDFF => (), //Not Usable
            0xFE00..=0xFE9F => self.gpu.gpu_write(address, value), // Object attribute memory (OAM)
            0xFEA0..=0xFEFF => (), //Not Usable
            0xFF46 => { //GPU LCD
                // TODO: account for the fact this takes 160 microseconds
                let dma_source = (value as u16) << 8;
                let dma_destination = 0xFE00;
                for offset in 0..150 {
                    let addr = dma_destination + offset;
                    let val = self.bus_read(dma_source + offset);
                    self.bus_write(
                        addr,
                        val
                    )
                }
            },
            0xFF40..=0xFF4B => self.gpu.gpu_write(address, value), // GPU LCD
            0xFF50 => self.cart.boot_rom = None,
            0xFF00..=0xFF7F => self.io.io_write(address, value), // I/O Registers
            0xFF80..=0xFFFE => self.ram.ram_write(address, value), // High ram
            0xFFFF => self.io.io_write(address, value), // Interrupt master
            _ => panic!("Bus write address not implemented: {:04X}", address)
        }
    }

    pub fn bus_write_word(&mut self, mut address: u16, value: u16) {
        let lsb = (value & 0xFF) as u8;          // Extract the LSB
        let msb = ((value >> 8) & 0xFF) as u8;   // Extract the MSB

        self.bus_write(address, lsb);
        let (addr, res) = address.overflowing_add(1);
        self.bus_write(addr, msb);
    }
}