#[allow(non_snake_case)]
pub mod bus;
pub mod lookup_table;
use bus::{Bus, BusRead, BusWrite};
use lookup_table::LookUpTable;
use std::{cell::RefCell, rc::Rc};

pub struct Cpu {
    bus: Rc<RefCell<Bus>>,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    acc: u8,
    psr: u8,
    fetched: u8,
    opcode: u8,
    addr_rel: u16,
    addr_mode_name: String,
    addr_abs: u16,
    cycles: u8,
}
pub enum FLAGS {
    C(u8), //Carry bit 1 = true
    Z(u8), //Zero 1 = result zero
    I(u8), //IRQ Disable 1 = true
    D(u8), //Decimal mode 1 = true
    B(u8), //Break command 1 = BRK
    U(u8), //Unused
    V(u8), //Overflow 1 = true
    N(u8), //Negative 1 = true
}

impl FLAGS {
    pub fn c() -> u8 {
        FLAGS::match_flags(FLAGS::C(1 << 0))
    }
    pub fn z() -> u8 {
        FLAGS::match_flags(FLAGS::Z(1 << 1))
    }
    pub fn i() -> u8 {
        FLAGS::match_flags(FLAGS::I(1 << 2))
    }
    pub fn d() -> u8 {
        FLAGS::match_flags(FLAGS::D(1 << 3))
    }
    pub fn b() -> u8 {
        FLAGS::match_flags(FLAGS::B(1 << 4))
    }
    pub fn u() -> u8 {
        FLAGS::match_flags(FLAGS::U(1 << 5))
    }
    pub fn v() -> u8 {
        FLAGS::match_flags(FLAGS::V(1 << 6))
    }
    pub fn n() -> u8 {
        FLAGS::match_flags(FLAGS::N(1 << 7))
    }
    pub fn match_flags(f: Self) -> u8 {
        match f {
            FLAGS::B(v)
            | FLAGS::C(v)
            | FLAGS::D(v)
            | FLAGS::I(v)
            | FLAGS::N(v)
            | FLAGS::U(v)
            | FLAGS::V(v)
            | FLAGS::Z(v) => v,
        }
    }
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        let cpu = Cpu {
            bus: Rc::clone(&bus),
            x: 0x00,
            y: 0x00,
            acc: 0x00,
            pc: 0x0000,
            psr: 0x00,
            sp: 0x00,
            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            addr_mode_name: "".to_string(),
            cycles: 0,
            opcode: 0x00,
        };

        cpu
    }
  
    fn read(&mut self, addr: u16) -> u8 {
        self.bus.borrow_mut().read(addr, false)
    }

    fn write(&mut self, addr: u16, data: u8) -> () {
        self.bus.borrow_mut().write(addr, data);
    }

    // Convenience methods
    fn get_flag(&mut self, f: u8) -> u8 {
        if self.psr & f > 0 {
            return 1;
        }
        0x00
    }
    fn set_flag(&mut self, f: u8, val: bool) {
        if val {
            self.psr |= f;
        }
        self.psr &= !f;
    }

    // Addressing mode helpers
    //Accumulator
    pub fn ACC(&mut self) -> u8 {
        0x00
    }
    //Immediate
    pub fn IMM(&mut self) -> u8 {
        self.addr_abs = self.pc + 1;
        self.pc+=1;
        0x00
    }
    //Absolute
    pub fn ABS(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc+=1;
        let hi = self.read(self.pc) as u16;
        self.pc+=1;
        self.addr_abs = (hi << 8) | lo;
        0x00
    }
    //Zero page
    pub fn ZP(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc) as u16;
        self.pc+=1;
        self.addr_abs &= 0x00FF;
        0x00
    }
    //Indirect zero page X
    pub fn ZPX(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc) + self.x) as u16;
        self.pc+=1;
        self.addr_abs &= 0x00FF;
        0x00
    }

    //Indirect zero page Y
    pub fn ZPY(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc) + self.y) as u16;
        self.pc+=1;
        self.addr_abs &= 0x00FF;
        0x00
    }
    //Indirect Absolute X
    pub fn ABSX(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc+=1;
        let hi = self.read(self.pc) as u16;
        self.pc+=1;
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        0x00
    }

    //Indirect Absolute Y
    pub fn ABSY(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc+=1;
        let hi = self.read(self.pc) as u16;
        self.pc+=1;
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        0x00
    }
    //Implied
    pub fn IMP(&mut self) -> u8 {
        self.fetched = self.acc;
        0x00
    }
    //Relative
    pub fn REL(&mut self) -> u8 {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc+=1;
        if self.addr_rel & 0x80 == 1 {
            self.addr_rel |= 0xFF00;
        }
        0x00
    }
    //Indirect indexed x
    pub fn INDX(&mut self) -> u8 {
        let t = self.read(self.pc) as u16;
        self.pc+=1;

        let lo = self.read((t + self.x as u16) & 0x00FF) as u16;
        let hi = self.read((t + self.x as u16 + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        0x00
    }
    //Indirect indexed y
    pub fn INDY(&mut self) -> u8 {
        let t = self.read(self.pc) as u16;
        self.pc+=1;

        let lo = self.read(t & 0x00FF) as u16;
        let hi = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1
        }
        0x00
    }
    //Absolute indirect
    pub fn ABSIND(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc +=1;
        let hi = self.read(self.pc) as u16;
        self.pc+=1;

        let ptr = (hi << 8) | lo;
        if lo == 0xFF {
            self.addr_abs = ((self.read(ptr & 0xFF00) << 8) | self.read(ptr + 0)) as u16;
        } 
        else {
            self.addr_abs = (((self.read(ptr + 1)) << 8) | self.read(ptr + 1)) as u16;
        }
        0x00
    }

    //Opcodes

    pub fn ADC(&mut self) -> u8 {
        self.fetch();

        let temp = self.acc as u16 + self.fetched as u16;
        self.set_flag(FLAGS::c(), temp > 255);

        let n = ((temp & 0x00FF) as u8) & 0x80;
        self.set_flag(FLAGS::n(), n == 1);
        self.set_flag(FLAGS::z(), (temp & 0x00FF) == 0);
        self.set_flag(FLAGS::v(), ((self.acc & 0x80) ^ n) & !((self.acc & 0x80) ^ (self.fetched & 0x80)) == 1);
        self.acc = (temp & 0x00FF) as u8;
        0x01
    }

    pub fn AND(&mut self) -> u8 {
        self.fetch();
        self.acc = self.acc & self.fetched;
        self.set_flag(FLAGS::n(), (self.acc & 0x80) == 1);
        self.set_flag(FLAGS::z(), self.acc == 0);
        0x01
    }

    pub fn ASL(&mut self) -> u8 {
        0x00
    }

    pub fn BCC(&mut self) -> u8 {
        0x00
    }

    pub fn BCS(&mut self) -> u8 {
        0x00
    }

    pub fn BEQ(&mut self) -> u8 {
        0x00
    }

    pub fn BIT(&mut self) -> u8 {
        0x00
    }

    pub fn BMI(&mut self) -> u8 {
        0x00
    }

    pub fn BNE(&mut self) -> u8 {
        0x00
    }

    pub fn BPL(&mut self) -> u8 {
        0x00
    }

    pub fn BRK(&mut self) -> u8 {
        0x00
    }

    pub fn BVC(&mut self) -> u8 {
        0x00
    }

    pub fn BVS(&mut self) -> u8 {
        0x00
    }

    pub fn CLC(&mut self) -> u8 {
        0x00
    }

    pub fn CLD(&mut self) -> u8 {
        0x00
    }
    pub fn CLI(&mut self) -> u8 {
        0x00
    }

    pub fn CLV(&mut self) -> u8 {
        0x00
    }

    pub fn CMP(&mut self) -> u8 {
        0x00
    }

    pub fn CPX(&mut self) -> u8 {
        0x00
    }

    pub fn CPY(&mut self) -> u8 {
        0x00
    }

    pub fn DEC(&mut self) -> u8 {
        0x00
    }

    pub fn DEX(&mut self) -> u8 {
        0x00
    }

    pub fn DEY(&mut self) -> u8 {
        0x00
    }

    pub fn EOR(&mut self) -> u8 {
        0x00
    }

    pub fn INC(&mut self) -> u8 {
        0x00
    }

    pub fn INX(&mut self) -> u8 {
        0x00
    }

    pub fn INY(&mut self) -> u8 {
        0x00
    }

    pub fn JMP(&mut self) -> u8 {
        0x00
    }

    pub fn JSR(&mut self) -> u8 {
        0x00
    }

    pub fn LDA(&mut self) -> u8 {
        0x00
    }

    pub fn LDX(&mut self) -> u8 {
        0x00
    }

    pub fn LDY(&mut self) -> u8 {
        0x00
    }

    pub fn LSR(&mut self) -> u8 {
        0x00
    }

    pub fn NOP(&mut self) -> u8 {
        0x00
    }

    pub fn ORA(&mut self) -> u8 {
        0x00
    }

    pub fn PHA(&mut self) -> u8 {
        0x00
    }

    pub fn PHP(&mut self) -> u8 {
        0x00
    }

    pub fn PLA(&mut self) -> u8 {
        0x00
    }

    pub fn PLP(&mut self) -> u8 {
        0x00
    }

    pub fn ROL(&mut self) -> u8 {
        0x00
    }

    pub fn ROR(&mut self) -> u8 {
        0x00
    }

    pub fn RTI(&mut self) -> u8 {
        0x00
    }

    pub fn RTS(&mut self) -> u8 {
        0x00
    }

    pub fn SBC(&mut self) -> u8 {
        self.fetch();

        let value = !self.fetched as u16;
        let temp = self.acc as u16 + value + self.get_flag(FLAGS::c()) as u16; 
        self.set_flag(FLAGS::c(), (temp & 0x00FF) == 1);
        self.set_flag(FLAGS::z(), (temp & 0x00FF) == 0);
        self.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        self.set_flag(FLAGS::v(), ((temp ^ self.acc as u16) & (temp ^ value) & 0x0080) == 1);
        self.acc = (temp & 0x00FF) as u8;
        0x01
    }

    pub fn SEC(&mut self) -> u8 {
        0x00
    }

    pub fn SED(&mut self) -> u8 {
        0x00
    }

    pub fn SEI(&mut self) -> u8 {
        0x00
    }

    pub fn STA(&mut self) -> u8 {
        0x00
    }

    pub fn STX(&mut self) -> u8 {
        0x00
    }

    pub fn STY(&mut self) -> u8 {
        0x00
    }

    pub fn TAX(&mut self) -> u8 {
        0x00
    }

    pub fn TAY(&mut self) -> u8 {
        0x00
    }

    pub fn TSX(&mut self) -> u8 {
        0x00
    }

    pub fn TXA(&mut self) -> u8 {
        0x00
    }

    pub fn TXS(&mut self) -> u8 {
        0x00
    }

    pub fn TYA(&mut self) -> u8 {
        0x00
    }

    pub fn XXX(&mut self) -> u8 {
        0x00
    }

    pub fn clock(&mut self, lookup: &mut LookUpTable) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.set_flag(FLAGS::u(), true);
            self.pc += 1;

            self.cycles = lookup.table[self.opcode as usize].cycles;

            self.addr_mode_name = lookup.table[self.opcode as usize].addr_name.to_string();

            let additional_cyles = lookup.table[self.opcode as usize].address_mode.as_mut()();

            let additional_cycles_2 = lookup.table[self.opcode as usize].operation.as_mut()();

            self.cycles += additional_cycles_2 + additional_cyles;

            self.set_flag(FLAGS::u(), true);
        }

        self.cycles -= 1
    }

    pub fn reset(&mut self) {
        self.addr_abs = 0xFFFC;

        let lo = self.read(self.addr_abs + 0) as u16;
        let hi = self.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.acc = 0x00;
        self.y = 0x00;
        self.x = 0x00;
        self.sp = 0xFD;

        self.psr = 0x00 | FLAGS::u();

        self.fetched = 0x00;
        self.addr_rel = 0x00;
        self.addr_abs = 0x00;

        self.cycles = 8;
    }

    pub fn irq(&mut self) {
        if self.get_flag(FLAGS::i()) == 0 {
            self.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.sp -= 1;
            self.write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
            self.sp -= 1;

            self.set_flag(FLAGS::b(), false);
            self.set_flag(FLAGS::u(), true);
            self.set_flag(FLAGS::i(), true);

            self.write(0x0100 + self.sp as u16, self.psr);
            self.sp -= 1;

            self.addr_abs = 0xFFFE;
            let lo = self.read(self.addr_abs + 0) as u16;
            let hi = self.read(self.addr_abs + 1) as u16;

            self.pc = (hi << 8) | lo;

            self.cycles = 7;
        }
    }
    pub fn nmi(&mut self) {
        self.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp -= 1;
        self.write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp -= 1;

        self.set_flag(FLAGS::b(), false);
        self.set_flag(FLAGS::u(), true);
        self.set_flag(FLAGS::i(), true);

        self.write(0x0100 + self.sp as u16, self.psr);
        self.sp -= 1;

        self.addr_abs = 0xFFFA;
        let lo = self.read(self.addr_abs + 0) as u16;
        let hi = self.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.cycles = 8;
    }

    pub fn fetch(&mut self) -> u8 {
        if self.addr_mode_name != "IMP" {
            self.fetched = self.read(self.addr_abs);
        }
        self.fetched
    }
}
