#[allow(non_snake_case)]
pub mod bus;
pub mod lookup_table;
use lookup_table::{LookUpTable};
use bus::{Bus, BusRead, BusWrite};
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
    pub fn C() -> u8 {
        FLAGS::match_flags(FLAGS::C(1 << 0))
    }
    pub fn Z() -> u8 {
        FLAGS::match_flags(FLAGS::Z(1 << 1))
    }
    pub fn I() -> u8 {
        FLAGS::match_flags(FLAGS::I(1 << 2))
    }
    pub fn D() -> u8 {
        FLAGS::match_flags(FLAGS::D(1 << 3))
    }
    pub fn B() -> u8 {
        FLAGS::match_flags(FLAGS::B(1 << 4))
    }
    pub fn U() -> u8 {
        FLAGS::match_flags(FLAGS::U(1 << 5))
    }
    pub fn V() -> u8 {
        FLAGS::match_flags(FLAGS::V(1 << 6))
    }
    pub fn N() -> u8 {
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
        Cpu {
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
            cycles: 0,
            opcode: 0x00,
        }
    }
    fn read(&mut self, addr: u16) -> u8 {
        self.bus.borrow_mut().read(addr, false)
    }

    fn write(&mut self, addr: u16, data: u8) -> () {
        self.bus.borrow_mut().write(addr, data);
    }

    // Convenience methods
    fn get_flag(&mut self, f: FLAGS) -> u8 {
        0x00
    }
    fn set_flag(&mut self, f: FLAGS, val: bool) {}

    // Addressing mode helpers
    //Accumulator
    pub fn ACC(&mut self) -> u8 {
        0x00
    }
    //Immediate
    pub fn IMM(&mut self) -> u8 {
        0x00
    }
    //Absolute
    pub fn ABS(&mut self) -> u8 {
        0x00
    }
    //Zero page
    pub fn ZP(&mut self) -> u8 {
        0x00
    }
    //Indirect zero page X
    pub fn ZPX(&mut self) -> u8 {
        0x00
    }

    //Indirect zero page Y
    pub fn ZPY(&mut self) -> u8 {
        0x00
    }
    //Indirect Absolute X
    pub fn ABSX(&mut self) -> u8 {
        0x00
    }

    //Indirect Absolute Y
    pub fn ABSY(&mut self) -> u8 {
        0x00
    }
    //Implied
    pub fn IMP(&mut self) -> u8 {
        0x00
    }
    //Relative
    pub fn REL(&mut self) -> u8 {
        0x00
    }
    //Indirect indexed x
    pub fn INDX(&mut self) -> u8 {
        0x00
    }
    //Indirect indexed y
    pub fn INDY(&mut self) -> u8 {
        0x00
    }
    //Absolute indirect
    pub fn ABSIND(&mut self) -> u8 {
        0x00
    }

    //Opcodes

    pub fn ADC(&mut self) -> u8 {
        0x00
    }
    pub fn AND(&mut self) -> u8 {
        0x00
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
        0x00
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

    pub fn clock(&mut self) {}

    pub fn reset(&mut self) {}

    pub fn irq(&mut self) {}
    pub fn nmi(&mut self) {}

    pub fn fetch(&self) -> u8 {
        0x00
    }

    pub fn run(cpu: Rc<RefCell<Cpu>>) {
        let look_up_table = LookUpTable::new(Rc::clone(&cpu));
    }
}

