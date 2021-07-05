use crate::Cpu;
use std::{cell::RefCell, rc::Rc};

pub struct Instruction<'a> {
    pub name: &'a str,
    pub operation: Box<dyn FnMut() -> u8 + 'a>,
    pub address_mode: Box<dyn FnMut() -> u8 + 'a>,
    pub cycles: u8,
}

// This is a 16 * 16 matrix representing the processor opcodes for the 6502 cpu
pub struct LookUpTable<'a> {
    pub table: Vec<Instruction<'a>>,
}

impl<'a> LookUpTable<'a> {
    pub fn new(cpu: Rc<RefCell<Cpu>>) -> LookUpTable<'a> {
        LookUpTable {
            table: vec![
                //ROW 0
                Instruction {
                    name: "BRK",
                    cycles: 7,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BRK())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "ORA",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ORA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "ORA",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ORA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "ASL",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ASL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "PHP",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().PHP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "ORA",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ORA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "ASL",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ASL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ACC())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "ORA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ORA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "ASL",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ASL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 1
                Instruction {
                    name: "BPL",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BPL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().REL())
                    },
                },
                Instruction {
                    name: "ORA",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ORA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "ORA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ORA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "ASL",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ASL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CLC",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CLC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "ORA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ORA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "ORA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ORA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "ASL",
                    cycles: 7,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ASL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 2
                Instruction {
                    name: "JSR",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().JSR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "AND",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().AND())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "BIT",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BIT())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "AND",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().AND())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "ROL",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "PLP",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().PLP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "AND",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().AND())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "ROL",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ACC())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "BIT",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BIT())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "AND",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().AND())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "ROL",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 3
                Instruction {
                    name: "BMI",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BMI())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().REL())
                    },
                },
                Instruction {
                    name: "AND",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().AND())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "AND",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().AND())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "ROL",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "SEC",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SEC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "AND",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().AND())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "AND",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().AND())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "ROL",
                    cycles: 7,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROL())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 4
                Instruction {
                    name: "RTI",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().RTI())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "EOR",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().EOR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "EOR",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().EOR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "LSR",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LSR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "PHA",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().PHA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "EOR",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().EOR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "LSR",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LSR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ACC())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "JMP",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().JMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "EOR",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().EOR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "LSR",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LSR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 5
                Instruction {
                    name: "BVC",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BVC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().REL())
                    },
                },
                Instruction {
                    name: "EOR",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().EOR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "EOR",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().EOR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "LSR",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LSR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CLI",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CLI())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "EOR",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().EOR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "EOR",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().EOR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "LSR",
                    cycles: 7,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LSR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 6
                Instruction {
                    name: "RTS",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().RTS())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "ADC",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ADC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "ADC",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ADC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "ROR",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "PLA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().PLA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "ADC",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ADC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "ROR",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ACC())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "JMP",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().JMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSIND())
                    },
                },
                Instruction {
                    name: "ADC",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ADC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "ROR",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 7
                Instruction {
                    name: "BVS",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BVS())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().REL())
                    },
                },
                Instruction {
                    name: "ADC",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ADC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "ADC",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ADC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "ROR",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "SEI",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SEI())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "ADC",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ADC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "ADC",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ADC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "ROR",
                    cycles: 7,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ROR())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 8
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "STA",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "STY",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "STA",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "STX",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "DEY",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().DEY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "TXA",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().TXA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "STY",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "STA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "STX",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW 9
                Instruction {
                    name: "BCC",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BCC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().REL())
                    },
                },
                Instruction {
                    name: "STA",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "STY",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "STA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "STX",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "TYA",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().TYA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "STA",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "TXS",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().TXS())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "STA",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().STA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW A
                Instruction {
                    name: "LDY",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "LDA",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDX())
                    },
                },
                Instruction {
                    name: "LDX",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "LDY",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "LDA",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "LDX",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "TAY",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().TAY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "LDA",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "TAX",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().TAX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "LDY",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "LDA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "LDX",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW B
                Instruction {
                    name: "BCS",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BCS())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().REL())
                    },
                },
                Instruction {
                    name: "LDA",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "LDY",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "LDA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "LDX",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CLV",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CLV())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "LDA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "TSX",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().TSX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "LDY",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "LDA",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDA())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "LDX",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().LDX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                // ROW C
                Instruction {
                    name: "CPY",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CPY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "CMP",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CPY",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CPY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "CMP",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "DEC",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().DEC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "INY",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "CMP",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "DEX",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().DEX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CPY",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CPY())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "CMP",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "DEC",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().DEC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                //ROW D
                Instruction {
                    name: "BNE",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BNE())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().REL())
                    },
                },
                Instruction {
                    name: "CMP",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CMP",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "DEC",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().DEC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CLD",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CLD())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "CMP",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CMP",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CMP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "DEC",
                    cycles: 7,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().DEC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                //ROW E
                Instruction {
                    name: "CPX",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CPX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "SBC",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SBC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CPX",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CPX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "SBC",
                    cycles: 3,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SBC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "INC",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "INX",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "SBC",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SBC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMM())
                    },
                },
                Instruction {
                    name: "NOP",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().NOP())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "CPX",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().CPX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "SBC",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SBC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "INC",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABS())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                //ROW F
                Instruction {
                    name: "BEQ",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().BEQ())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().REL())
                    },
                },
                Instruction {
                    name: "SBC",
                    cycles: 5,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SBC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INDY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "SBC",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SBC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "INC",
                    cycles: 6,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ZPX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "SED",
                    cycles: 2,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SED())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().IMP())
                    },
                },
                Instruction {
                    name: "SBC",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SBC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSY())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
                Instruction {
                    name: "SBC",
                    cycles: 4,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().SBC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "INC",
                    cycles: 7,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().INC())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().ABSX())
                    },
                },
                Instruction {
                    name: "???",
                    cycles: 0,
                    operation: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                    address_mode: {
                        let cpu_clone = Rc::clone(&cpu);
                        Box::new(move || cpu_clone.borrow_mut().XXX())
                    },
                },
            ],
        }
    }
}
