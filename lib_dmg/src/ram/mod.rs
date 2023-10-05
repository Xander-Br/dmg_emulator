pub struct RAM {
    wram: Vec<u8>,
    hram: Vec<u8>,
    eram: Vec<u8>,
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            wram: vec![0; 0x2000],
            hram: vec![0; 0x80],
            eram: vec![0; 0x2000],
        }
    }

    pub fn ram_read(&mut self, mut address: u16) -> u8 {
        match address {
            0xA000..=0xBFFF => {
                address -= 0xA000;
                self.eram[address as usize]
            },
            0xC000..=0xDFFF => {
                address -= 0xC000;
                self.wram[address as usize]
            }
            0xFF80..=0xFFFE => {
                address -= 0xFF80;
                self.hram[address as usize]
            }
            _ => panic!("RAM read address not implemented: {:04X}", address)
        }
    }

    pub fn ram_write(&mut self, mut address: u16, value: u8) {
        match address {
            0xA000..=0xBFFF => {
                address -= 0xA000;
                self.eram[address as usize] = value
            },
            0xC000..=0xDFFF => {
                address -= 0xC000;
                self.wram[address as usize] = value
            }
            0xFF80..=0xFFFE => {
                address -= 0xFF80;
                self.hram[address as usize] = value
            }
            _ => panic!("RAM read address not implemented: {:04X}", address)
        }
    }
}