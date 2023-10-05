pub enum Frequency {
    F4096,
    F16384,
    F262144,
    F65536,
}

impl Frequency {
    fn cycles_per_tick(&self) -> usize {
        match self {
            Frequency::F4096 => 1024,
            Frequency::F16384 => 256,
            Frequency::F262144 => 16,
            Frequency::F65536 => 64,
        }
    }
}

pub struct Timer {
    div: u16,     // Using u16 to ensure the DIV overflow behaves correctly
    tima: u8,
    tma: u8,
    tac: u8,
    cycles: usize,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
            cycles: 0,
        }
    }

    pub fn timer_init(&mut self){
        self.div = 0xAC00;
    }

    pub fn step(&mut self, cpu_cycles: u8) -> bool {
        self.cycles += cpu_cycles as usize;

        while self.cycles >= 256 {
            self.cycles -= 256;
            self.div.wrapping_add(1);
        }

        if self.tac & 0x04 != 0 {
            let freq = match self.tac & 0x03 {
                0 => Frequency::F4096,
                1 => Frequency::F262144,
                2 => Frequency::F65536,
                3 => Frequency::F16384,
                _ => unreachable!(),
            };

            while self.cycles >= freq.cycles_per_tick() {
                self.cycles -= freq.cycles_per_tick();
                self.tima = self.tima.wrapping_add(1);

                if self.tima == 0 {
                    self.tima = self.tma;
                    return true; //Fire interrupt
                }
            }
        }
        return false;
    }

    pub fn timer_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div & 0xFF) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("Timer read address not implemented: {:04X}", address)
        }
    }

    pub fn timer_write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => panic!("Timer write address not implemented: {:04X}", address)
        }
    }
}
