#[allow(non_snake_case)]
pub mod bus;
pub mod lookup_table;
use bus::{Bus, BusRead, BusWrite};
use lookup_table::LookUpTable;
use std::{cell::RefCell, rc::Rc};

use crate::bus::RAM_SIZE;

pub struct Cpu {
    pub bus: Rc<RefCell<Bus>>,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub acc: u8,
    pub psr: u8,
    pub fetched: u8,
    pub opcode: u8,
    pub addr_rel: u16,
    pub addr_mode_name: String,
    pub addr_abs: u16,
    pub cycles: u8,
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
            sp: 0xFD,
            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            addr_mode_name: "".to_string(),
            cycles: 0,
            opcode: 0x00,
        };

        cpu
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.bus.borrow_mut().read(addr, false)
    }

    fn write(&mut self, addr: u16, data: u8) -> () {
        self.bus.borrow_mut().write(addr, data);
    }

    // Convenience methods
    pub fn get_flag(&self, f: u8) -> u8 {
        if self.psr & f > 0 {
            return 1;
        }
        0x00
    }
    fn set_flag(&mut self, f: u8, val: bool) {
        if val {
            self.psr |= f;
        }
        else {
            self.psr &= !f;
        }
        
    }

    // Addressing mode helpers
    //Accumulator
    pub fn ACC(cpu: &mut Cpu) -> u8 {
        0x00
    }
    //Immediate
    pub fn IMM(cpu: &mut Cpu) -> u8 {
        cpu.addr_abs = cpu.pc + 1;
        cpu.pc += 1;
        0x00
    }
    //Absolute
    pub fn ABS(cpu: &mut Cpu) -> u8 {
        let lo = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        let hi = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.addr_abs = (hi << 8) | lo;
        0x00
    }
    //Zero page
    pub fn ZP(cpu: &mut Cpu) -> u8 {
        cpu.addr_abs = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF;
        0x00
    }
    //Indirect zero page X
    pub fn ZPX(cpu: &mut Cpu) -> u8 {
        cpu.addr_abs = (cpu.read(cpu.pc) + cpu.x) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF;
        0x00
    }

    //Indirect zero page Y
    pub fn ZPY(cpu: &mut Cpu) -> u8 {
        cpu.addr_abs = (cpu.read(cpu.pc) + cpu.y) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= 0x00FF;
        0x00
    }
    //Indirect Absolute X
    pub fn ABSX(cpu: &mut Cpu) -> u8 {
        let lo = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        let hi = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.addr_abs = (hi << 8) | lo;
        cpu.addr_abs += cpu.x as u16;

        if (cpu.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        0x00
    }

    //Indirect Absolute Y
    pub fn ABSY(cpu: &mut Cpu) -> u8 {
        let lo = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        let hi = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        cpu.addr_abs = (hi << 8) | lo;
        cpu.addr_abs += cpu.y as u16;

        if (cpu.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        0x00
    }
    //Implied
    pub fn IMP(cpu: &mut Cpu) -> u8 {
        cpu.fetched = cpu.acc;
        0x00
    }
    //Relative
    pub fn REL(cpu: &mut Cpu) -> u8 {
        cpu.addr_rel = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        if cpu.addr_rel & 0x80 == 1 {
            cpu.addr_rel |= 0xFF00;
        }
        0x00
    }
    //Indirect indexed x
    pub fn INDX(cpu: &mut Cpu) -> u8 {
        let t = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;

        let lo = cpu.read((t + cpu.x as u16) & 0x00FF) as u16;
        let hi = cpu.read((t + cpu.x as u16 + 1) & 0x00FF) as u16;

        cpu.addr_abs = (hi << 8) | lo;
        0x00
    }
    //Indirect indexed y
    pub fn INDY(cpu: &mut Cpu) -> u8 {
        let t = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;

        let lo = cpu.read(t & 0x00FF) as u16;
        let hi = cpu.read((t + 1) & 0x00FF) as u16;

        cpu.addr_abs = (hi << 8) | lo;
        cpu.addr_abs += cpu.y as u16;

        if (cpu.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        }
        0x00
    }
    //Absolute indirect
    pub fn ABSIND(cpu: &mut Cpu) -> u8 {
        let lo = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;
        let hi = cpu.read(cpu.pc) as u16;
        cpu.pc += 1;

        let ptr = (hi << 8) | lo;
        if lo == 0x00FF {
            cpu.addr_abs =
                (((cpu.read(ptr & 0xFF00) as u16) << 8) | (cpu.read(ptr + 0) as u16)) as u16;
        } else {
            cpu.addr_abs = (((cpu.read(ptr + 1) as u16) << 8) | (cpu.read(ptr + 1) as u16)) as u16;
        }
        0x00
    }

    //Instructions

    pub fn ADC(cpu: &mut Cpu) -> u8 {
        cpu.fetch();

        let temp = cpu.acc as u16 + cpu.fetched as u16;
        cpu.set_flag(FLAGS::c(), temp > 255);

        let n = ((temp & 0x00FF) as u8) & 0x80;
        cpu.set_flag(FLAGS::n(), n == 1);
        cpu.set_flag(FLAGS::z(), (temp & 0x00FF) == 0);
        cpu.set_flag(
            FLAGS::v(),
            ((cpu.acc & 0x80) ^ n) & !((cpu.acc & 0x80) ^ (cpu.fetched & 0x80)) == 1,
        );
        cpu.acc = (temp & 0x00FF) as u8;
        0x01
    }

    pub fn AND(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        cpu.acc = cpu.acc & cpu.fetched;
        cpu.set_flag(FLAGS::n(), (cpu.acc & 0x80) == 1);
        cpu.set_flag(FLAGS::z(), cpu.acc == 0);
        0x01
    }

    pub fn ASL(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = (cpu.fetched as u16) << 1;
        cpu.set_flag(FLAGS::c(), (temp & 0xFF00) > 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        cpu.set_flag(FLAGS::z(), (temp & 0x00FF) == 0x00);
        if cpu.addr_mode_name == "IMP" {
            cpu.acc = (temp & 0x00FF) as u8;
        } else {
            cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
        }
        0x00
    }

    pub fn BCC(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(FLAGS::c()) == 0 {
            cpu.cycles += 1;

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs
        }
        0x00
    }

    pub fn BCS(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(FLAGS::c()) == 1 {
            cpu.cycles += 1;

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs
        }
        0x00
    }

    pub fn BEQ(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(FLAGS::z()) == 0 {
            cpu.cycles += 1;

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs
        }
        0x00
    }

    pub fn BIT(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = cpu.acc & cpu.fetched;
        cpu.set_flag(FLAGS::z(), (temp) == 0);
        cpu.set_flag(FLAGS::n(), cpu.fetched & FLAGS::n() == 1);
        cpu.set_flag(FLAGS::v(), cpu.fetched & FLAGS::v() == 1);
        0x00
    }

    pub fn BMI(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(FLAGS::n()) == 1 {
            cpu.cycles += 1;

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs
        }
        0x00
    }

    pub fn BNE(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(FLAGS::z()) == 0 {
            cpu.cycles += 1;

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs
        }
        0x00
    }

    pub fn BPL(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(FLAGS::n()) == 0 {
            cpu.cycles += 1;

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs
        }
        0x00
    }

    pub fn BRK(cpu: &mut Cpu) -> u8 {
        cpu.pc += 1;

        cpu.set_flag(FLAGS::i(), true);
        cpu.write(0x0100 + cpu.sp as u16, ((cpu.pc & 0xFF00) >> 8) as u8);
        cpu.sp -= 1;
        cpu.write(0x0100 + cpu.sp as u16, (cpu.pc & 0x00FF) as u8);
        cpu.sp -= 1;

        cpu.set_flag(FLAGS::b(), true);
        cpu.write(0x0100 + cpu.sp as u16, cpu.psr);
        cpu.sp -= 1;

        cpu.set_flag(FLAGS::b(), false);

        cpu.pc = (cpu.read(0xFFFE) as u16) | ((cpu.read(0xFFFF) as u16) << 8);

        0x00
    }

    pub fn BVC(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(FLAGS::v()) == 0 {
            cpu.cycles += 1;

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs
        }
        0x00
    }

    pub fn BVS(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(FLAGS::v()) == 1 {
            cpu.cycles += 1;

            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs
        }
        0x00
    }

    pub fn CLC(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(FLAGS::c(), false);
        0x00
    }

    pub fn CLD(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(FLAGS::d(), false);
        0x00
    }
    pub fn CLI(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(FLAGS::i(), false);
        0x00
    }

    pub fn CLV(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(FLAGS::v(), false);
        0x00
    }

    pub fn CMP(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = cpu.acc as u16 - cpu.fetched as u16;
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::c(), cpu.acc >= cpu.fetched);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn CPX(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = cpu.x as u16 - cpu.fetched as u16;
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::c(), cpu.x >= cpu.fetched);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn CPY(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = cpu.y as u16 - cpu.fetched as u16;
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::c(), cpu.y >= cpu.fetched);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn DEC(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = cpu.fetched as u16 - 1;
        cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn DEX(cpu: &mut Cpu) -> u8 {
        let temp = cpu.x as u16 - 1;
        cpu.x = (temp & 0x00FF) as u8;
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn DEY(cpu: &mut Cpu) -> u8 {
        let temp = cpu.y as u16 - 1;
        cpu.y = (temp & 0x00FF) as u8;
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn EOR(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = cpu.acc ^ cpu.fetched;
        cpu.acc = temp;
        cpu.set_flag(FLAGS::n(), (temp & 0x80) == 1);
        cpu.set_flag(FLAGS::z(), temp == 0x00);
        0x00
    }

    pub fn INC(cpu: &mut Cpu) -> u8 {
        cpu.fetch();

        let temp = cpu.fetched as u16 + 1;
        cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn INX(cpu: &mut Cpu) -> u8 {
        let temp = cpu.x as u16 + 1;
        cpu.x = (temp & 0x00FF) as u8;
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn INY(cpu: &mut Cpu) -> u8 {
        let temp = cpu.y as u16 + 1;
        cpu.y = (temp & 0x00FF) as u8;
        cpu.set_flag(FLAGS::z(), temp == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        0x00
    }

    pub fn JMP(cpu: &mut Cpu) -> u8 {
        cpu.pc = cpu.addr_abs;
        0x00
    }

    pub fn JSR(cpu: &mut Cpu) -> u8 {
        cpu.pc -= 1;
        cpu.write(0x0100 + cpu.sp as u16, ((cpu.pc >> 8) & 0x00FF) as u8);
        cpu.sp -= 1;
        cpu.write(0x0100 + cpu.sp as u16, (cpu.pc & 0x00FF) as u8);
        cpu.sp -= 1;

        cpu.pc = cpu.addr_abs;
        0x00
    }

    pub fn LDA(cpu: &mut Cpu) -> u8 {
        cpu.fetch();

        cpu.acc = cpu.fetched;
        cpu.set_flag(FLAGS::z(), cpu.acc == 0x00);
        cpu.set_flag(FLAGS::n(), (cpu.acc & 0x80) == 1);
        0x00
    }

    pub fn LDX(cpu: &mut Cpu) -> u8 {
        cpu.fetch();

        cpu.x = cpu.fetched;
        cpu.set_flag(FLAGS::z(), cpu.x == 0x00);
        cpu.set_flag(FLAGS::n(), (cpu.x & 0x80) == 1);
        0x00
    }

    pub fn LDY(cpu: &mut Cpu) -> u8 {
        cpu.fetch();

        cpu.y = cpu.fetched;
        cpu.set_flag(FLAGS::z(), cpu.y == 0x00);
        cpu.set_flag(FLAGS::n(), (cpu.y & 0x80) == 1);
        0x00
    }

    pub fn LSR(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        cpu.set_flag(FLAGS::c(), (cpu.fetched as u16 & 0x0001) == 1);
        let temp = (cpu.fetched as u16) >> 1;
        cpu.set_flag(FLAGS::z(), temp == 0x0000);
        cpu.set_flag(FLAGS::n(), (temp & 0x8000) == 1);

        if cpu.addr_mode_name == "IMP" {
            cpu.acc = (temp & 0x00FF) as u8;
        } else {
            cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
        }

        0x00
    }

    pub fn NOP(cpu: &mut Cpu) -> u8 {
        match cpu.opcode {
            0x01C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => return 0x01,
            _ => return 0x00,
        }
    }

    pub fn ORA(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        cpu.acc = cpu.acc | cpu.fetched;
        cpu.set_flag(FLAGS::z(), cpu.acc == 0);
        cpu.set_flag(FLAGS::n(), (cpu.acc & 0x80) == 1);
        0x00
    }

    pub fn PHA(cpu: &mut Cpu) -> u8 {
        cpu.write(0x0100 + cpu.sp as u16, cpu.acc);
        cpu.sp -= 1;
        0x00
    }

    pub fn PHP(cpu: &mut Cpu) -> u8 {
        cpu.write(0x0100 + cpu.sp as u16, cpu.psr | FLAGS::b() | FLAGS::u());
        cpu.sp -= 1;
        0x00
    }

    pub fn PLA(cpu: &mut Cpu) -> u8 {
        cpu.sp += 1;
        cpu.acc = cpu.read(0x0100 + cpu.sp as u16);
        cpu.set_flag(FLAGS::z(), cpu.acc == 0);
        cpu.set_flag(FLAGS::n(), (cpu.acc & 0x80) == 1);
        0x00
    }

    pub fn PLP(cpu: &mut Cpu) -> u8 {
        cpu.sp += 1;
        cpu.psr = cpu.read(0x0100 + cpu.sp as u16);
        cpu.set_flag(FLAGS::u(), true);
        0x00
    }

    pub fn ROL(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = ((cpu.fetched as u16) << 1) | cpu.get_flag(FLAGS::c()) as u16;
        cpu.set_flag(FLAGS::c(), (temp & 0xFF00) == 1);
        cpu.set_flag(FLAGS::z(), (temp & 0x00FF) == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);

        if cpu.addr_mode_name == "IMP" {
            cpu.acc = (temp & 0x00FF) as u8;
        } else {
            cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
        }

        0x00
    }

    pub fn ROR(cpu: &mut Cpu) -> u8 {
        cpu.fetch();
        let temp = ((cpu.fetched as u16) >> 1) | ((cpu.get_flag(FLAGS::c()) as u16) << 7);
        cpu.set_flag(FLAGS::c(), (cpu.fetched & 0x01) == 1);
        cpu.set_flag(FLAGS::z(), (temp & 0x00FF) == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);

        if cpu.addr_mode_name == "IMP" {
            cpu.acc = (temp & 0x00FF) as u8;
        } else {
            cpu.write(cpu.addr_abs, (temp & 0x00FF) as u8);
        }

        0x00
    }

    pub fn RTI(cpu: &mut Cpu) -> u8 {
        cpu.sp += 1;
        cpu.psr = cpu.read(0x0100 + cpu.sp as u16);
        cpu.psr &= !FLAGS::b();
        cpu.psr &= !FLAGS::u();
        cpu.sp += 1;

        cpu.pc = cpu.read(0x0100 + cpu.sp as u16) as u16;
        cpu.sp += 1;
        cpu.pc |= (cpu.read(0x0100 + cpu.sp as u16) as u16) << 8;
        0x00
    }

    pub fn RTS(cpu: &mut Cpu) -> u8 {
        cpu.sp += 1;

        cpu.pc = cpu.read(0x0100 + cpu.sp as u16) as u16;
        cpu.sp += 1;
        cpu.pc |= (cpu.read(0x0100 + cpu.sp as u16) as u16) << 8;
        cpu.pc += 1;
        0x00
    }

    pub fn SBC(cpu: &mut Cpu) -> u8 {
        cpu.fetch();

        let value = !cpu.fetched as u16;
        let temp = cpu.acc as u16 + value + cpu.get_flag(FLAGS::c()) as u16;
        cpu.set_flag(FLAGS::c(), (temp & 0xFF00) == 1);
        cpu.set_flag(FLAGS::z(), (temp & 0x00FF) == 0);
        cpu.set_flag(FLAGS::n(), (temp & 0x0080) == 1);
        cpu.set_flag(
            FLAGS::v(),
            ((temp ^ cpu.acc as u16) & (temp ^ value) & 0x0080) == 1,
        );
        cpu.acc = (temp & 0x00FF) as u8;
        0x01
    }

    pub fn SEC(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(FLAGS::c(), true);
        0x00
    }

    pub fn SED(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(FLAGS::d(), true);
        0x00
    }

    pub fn SEI(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(FLAGS::i(), true);
        0x00
    }

    pub fn STA(cpu: &mut Cpu) -> u8 {
        cpu.write(cpu.addr_abs, cpu.acc);
        0x00
    }

    pub fn STX(cpu: &mut Cpu) -> u8 {
        cpu.write(cpu.addr_abs, cpu.x);
        0x00
    }

    pub fn STY(cpu: &mut Cpu) -> u8 {
        cpu.write(cpu.addr_abs, cpu.y);
        0x00
    }

    pub fn TAX(cpu: &mut Cpu) -> u8 {
        cpu.x = cpu.acc;
        cpu.set_flag(FLAGS::z(), cpu.x == 0x00);
        cpu.set_flag(FLAGS::n(), (cpu.x & 0x80) == 1);
        0x00
    }

    pub fn TAY(cpu: &mut Cpu) -> u8 {
        cpu.y = cpu.acc;
        cpu.set_flag(FLAGS::z(), cpu.y == 0x00);
        cpu.set_flag(FLAGS::n(), (cpu.y & 0x80) == 1);
        0x00
    }

    pub fn TSX(cpu: &mut Cpu) -> u8 {
        cpu.x = cpu.sp;
        cpu.set_flag(FLAGS::z(), cpu.x == 0x00);
        cpu.set_flag(FLAGS::n(), (cpu.x & 0x80) == 1);
        0x00
    }

    pub fn TXA(cpu: &mut Cpu) -> u8 {
        cpu.acc = cpu.x;
        cpu.set_flag(FLAGS::z(), cpu.acc == 0x00);
        cpu.set_flag(FLAGS::n(), (cpu.acc & 0x80) == 1);
        0x00
    }

    pub fn TXS(cpu: &mut Cpu) -> u8 {
        cpu.sp = cpu.x;
        0x00
    }

    pub fn TYA(cpu: &mut Cpu) -> u8 {
        cpu.acc = cpu.y;
        cpu.set_flag(FLAGS::z(), cpu.acc == 0x00);
        cpu.set_flag(FLAGS::n(), (cpu.acc & 0x80) == 1);
        0x00
    }

    pub fn XXX(cpu: &mut Cpu) -> u8 {
        0x00
    }

    pub fn clock(&mut self, lookup: &mut LookUpTable) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.set_flag(FLAGS::u(), true);
            self.pc += 1;

            self.cycles = lookup.table[self.opcode as usize].cycles;

            self.addr_mode_name = lookup.table[self.opcode as usize].addr_name.to_string();

            let additional_cyles = { lookup.table[self.opcode as usize].address_mode }(self);

            let additional_cycles_2 = { lookup.table[self.opcode as usize].operation }(self);

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

    pub fn complete(&self) -> bool {
        self.cycles == 0
    }

    pub fn disassemble(&mut self, start: u16, stop: u16, lookup: &LookUpTable) -> Vec<String> {
        let mut map_lines: Vec<String> = vec![String::new();RAM_SIZE];
        let mut value = 0x00u8;
        let mut hi = 0x00u8;
        let mut lo = 0x00u8;
        let mut addr = start;
        let mut line_addr = 0u16;

        while addr < stop {
            line_addr = addr;
            let mut map_line = String::new();

            map_line.insert_str(map_line.len(), &format!("${:04x}: ", addr)[..]);
            let opcode = self.read(addr);
            addr += 1;
            map_line.insert_str(
                map_line.len(),
                &format!("{} ", lookup.table[opcode as usize].name)[..],
            );

            if lookup.table[opcode as usize].addr_name == "IMP" {
                map_line.insert_str(map_line.len(), " {IMP}");
            } else if lookup.table[opcode as usize].addr_name == "IMM" {
                value = self.read(addr);
                addr += 1;
                map_line.insert_str(map_line.len(), &format!(" ${:04x}, `{{`IMM`}}`", value)[..])
            } else if lookup.table[opcode as usize].addr_name == "ZP" {
                lo = self.read(addr);
                addr += 1;
                map_line.insert_str(map_line.len(), &format!(" ${:04x}, `{{`ZP`}}`", lo)[..])
            } else if lookup.table[opcode as usize].addr_name == "ZPX" {
                lo = self.read(addr);
                addr += 1;
                map_line.insert_str(map_line.len(), &format!(" ${:04x}, X `{{`ZPX`}}`", lo)[..])
            } else if lookup.table[opcode as usize].addr_name == "ZPY" {
                lo = self.read(addr);
                addr += 1;
                map_line.insert_str(map_line.len(), &format!(" ${:04x}, Y `{{`ZPY`}}`", lo)[..])
            } else if lookup.table[opcode as usize].addr_name == "INDX" {
                lo = self.read(addr);
                addr += 1;
                map_line.insert_str(
                    map_line.len(),
                    &format!(" ${:04x}, X) `{{`INDX`}}`", lo)[..],
                )
            } else if lookup.table[opcode as usize].addr_name == "INDY" {
                lo = self.read(addr);
                addr += 1;
                map_line.insert_str(
                    map_line.len(),
                    &format!(" ${:04x}, Y) `{{`INDX`}}`", lo)[..],
                )
            } else if lookup.table[opcode as usize].addr_name == "ABS" {
                lo = self.read(addr);
                addr += 1;
                hi = self.read(addr);
                addr += 1;
                map_line.insert_str(
                    map_line.len(),
                    &format!(" ${:04x}, `{{`ABS`}}`", ((hi as u16) << 8) | lo as u16)[..],
                )
            } else if lookup.table[opcode as usize].addr_name == "ABSX" {
                lo = self.read(addr);
                addr += 1;
                hi = self.read(addr);
                addr += 1;
                map_line.insert_str(
                    map_line.len(),
                    &format!(" ${:04x}, X `{{`ABS`}}`", ((hi as u16) << 8) | lo as u16)[..],
                )
            } else if lookup.table[opcode as usize].addr_name == "ABSY" {
                lo = self.read(addr);
                addr += 1;
                hi = self.read(addr);
                addr += 1;
                map_line.insert_str(
                    map_line.len(),
                    &format!(" ${:04x}, Y `{{`ABS`}}`", ((hi as u16) << 8) | lo as u16)[..],
                )
            } else if lookup.table[opcode as usize].addr_name == "ABSIND" {
                lo = self.read(addr);
                addr += 1;
                hi = self.read(addr);
                addr += 1;
                map_line.insert_str(
                    map_line.len(),
                    &format!(" ${:04x}, `{{`ABSIND`}}`", ((hi as u16) << 8) | lo as u16)[..],
                )
            } else if lookup.table[opcode as usize].addr_name == "REL" {
                value = self.read(addr);
                addr += 1;
                map_line.insert_str(
                    map_line.len(),
                    &format!(" ${:04x} [${:04x}] `{{`REL`}}`", value, addr)[..],
                )
            }
            
            map_lines[line_addr as usize] = map_line;
        }
        map_lines
    }
}

