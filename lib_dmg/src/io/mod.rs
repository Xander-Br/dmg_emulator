use crate::io::interrupt::InterruptFlags;
use crate::io::joypad::Joypad;
use crate::io::timer::Timer;

mod timer;
mod interrupt;
mod joypad;

pub struct IO {
    pub timer: Timer,
    pub interrupt_flag: InterruptFlags,
    pub interrupt_enable: InterruptFlags,
    pub joypad: Joypad,
    io: [u8; 0x80]
}

impl IO {
    pub fn new() -> IO {
        IO {
            timer: Timer::new(),
            interrupt_flag: InterruptFlags::new(),
            interrupt_enable: InterruptFlags::new(),
            joypad: Joypad::new(),
            io: [0;0x80]
        }
    }

    pub fn io_read(&mut self, mut address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.to_byte(),
            0xFF04..=0xFF07 => self.timer.timer_read(address),
            0xFF10..=0xFF26 => 0,
            0xFF0F => self.interrupt_flag.to_byte(),
            0xFFFF => self.interrupt_enable.to_byte(),
            _ => {
                address-=0xFF00;
                self.io[address as usize]
            },
        }
    }

    pub fn io_write(&mut self, mut address: u16, value: u8) {
        match address {
            0xFF00 => { //Joypad
                self.joypad.column = if (value & 0x20) == 0 {
                    joypad::Column::One
                } else {
                    joypad::Column::Zero
                };
            }
            0xFF01..=0xFF02 => {/* SERIAL */}

            0xFF04..=0xFF07 => self.timer.timer_write(address, value),

            0xFF0F => self.interrupt_flag.from_byte(value),
            0xFF10..=0xFF3F => { /* Normally for sound */ }
            0xFF51..=0xFF7F => { /* Gameboy color */ }

            0xFFFF => self.interrupt_enable.from_byte(value),
            _ => {
                address-=0xFF00;
                self.io[address as usize] = value;
            },
        }
    }
}
