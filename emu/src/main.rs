use std::{cell::RefCell, rc::{Rc}};

use cpu::{Cpu, bus::Bus, lookup_table::LookUpTable};
fn main() {
    let bus = Rc::new(RefCell::new(Bus::new()));

    let cpu = Rc::new(RefCell::new(Cpu::new(Rc::clone(&bus))));
    let cpu_clone = Rc::clone(&cpu);
    
    let mut look_up = LookUpTable::new(&cpu_clone);

    cpu.borrow_mut().clock(&mut look_up);
    //Cpu::run(cpu_clone);
}
