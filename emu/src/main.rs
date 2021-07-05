use std::{cell::RefCell, rc::Rc};

use cpu::{bus::Bus, Cpu};
fn main() {
    let bus = Rc::new(RefCell::new(Bus::new()));

    let cpu = Rc::new(RefCell::new(Cpu::new(Rc::clone(&bus))));
    let cpu_clone = Rc::clone(&cpu);
    Cpu::run(cpu_clone);
}
