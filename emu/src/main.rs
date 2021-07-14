use std::{cell::RefCell, rc::{Rc}};

use cpu::{Cpu, bus::Bus, lookup_table::LookUpTable};
fn main() {
    let bus = Rc::new(RefCell::new(Bus::new()));

    let mut cpu = Cpu::new(Rc::clone(&bus));
    
    
    let mut look_up = LookUpTable::new();

    cpu.clock(&mut look_up);
 
}
