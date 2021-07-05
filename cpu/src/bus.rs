const RAM_SIZE: usize = 64 * 1024;

#[derive(Debug, Clone)]
pub struct Bus {
    ram: [u8; RAM_SIZE]
}
impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; RAM_SIZE]
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.ram.len() {
            self.ram[i] = 0x00;
        }
    }
}


pub trait BusWrite {
    fn write(&mut self, addr: u16, data: u8) -> ();
}

pub trait BusRead {
    fn read(&mut self, addr: u16, read_only: bool) -> u8;
}

impl BusWrite for Bus {
    fn write(&mut self, addr: u16, data: u8) -> () {
            self.ram[addr as usize] = data;
    }
}

impl BusRead for Bus {
    fn read(&mut self, addr: u16, _read_only: bool) -> u8 {
        self.ram[addr as usize]
    }
}