use cpu::bus::Bus;
use cpu::lookup_table::LookUpTable;
use cpu::{Cpu, FLAGS};
use find_folder;
use piston_window::{Context, Event, EventLoop, G2d, PistonWindow, WindowSettings, *};
use std::cell::RefCell;
use std::rc::Rc;

const WHITE: [f32; 4] = [255.0, 255.0, 255.0, 1.0];
const RED: [f32; 4] = [255.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 255.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 255.0, 1.0];
const CYAN: [f32; 4] = [50.0, 100.0, 255.0, 1.0];

struct Game<'a> {
    font: Glyphs,
    cpu: &'a mut Cpu,
    map_asm: Vec<String>,
}

impl<'a> Game<'a> {
    fn new(font: Glyphs, cpu: &'a mut Cpu, map_asm: Vec<String>) -> Self {
        Game { font, cpu, map_asm }
    }
    pub fn draw_string(
        &mut self,
        c: Context,
        g: &mut G2d,
        d: &mut GfxDevice,
        x: f64,
        y: f64,
        value: &str,
        color: [f32; 4],
    ) {
        let transform = c.transform.trans(x, y);
        text::Text::new_color(color, 16)
            .draw(value, &mut self.font, &c.draw_state, transform, g)
            .unwrap();

        self.font.factory.encoder.flush(d);
    }

    fn render(&mut self, c: Context, g: &mut G2d, d: &mut GfxDevice) {
        self.draw_string(
            c,
            g,
            d,
            10.0,
            25.0,
            "Hello world",
            [255.0, 255.0, 255.0, 1.0],
        )
    }

    fn update(&mut self, args: UpdateArgs) {}

    fn draw_ram(
        &mut self,
        c: Context,
        g: &mut G2d,
        d: &mut GfxDevice,
        x: f64,
        y: f64,
        mut addr: u16,
        rows: u32,
        cols: u32,
    ) {
        let ram_x = x;
        let mut ram_y = y;

        for _ in 0..rows {
            let mut s_offset = format!("${:04x}:", addr);
            for _ in 0..cols {
                s_offset.insert_str(s_offset.len(), &format!(" {:04x}", self.cpu.read(addr))[..]);
                addr += 1
            }

            self.draw_string(c, g, d, ram_x, ram_y, &s_offset, [255.0, 255.0, 255.0, 1.0]);
            ram_y += 16.0
        }
    }

    fn draw_cpu(&mut self, c: Context, g: &mut G2d, d: &mut GfxDevice, x: f64, y: f64) {
        self.draw_string(c, g, d, x, y, "STATUS:  ", WHITE);
        self.draw_string(
            c,
            g,
            d,
            x + 72.0,
            y,
            "N",
            if (self.cpu.get_flag(FLAGS::n())) == 1 {
                GREEN
            } else {
                RED
            },
        );
        self.draw_string(
            c,
            g,
            d,
            x + 88.0,
            y,
            "V",
            if (self.cpu.get_flag(FLAGS::v())) == 1 {
                GREEN
            } else {
                RED
            },
        );
        self.draw_string(
            c,
            g,
            d,
            x + 104.0,
            y,
            "-",
            if (self.cpu.get_flag(FLAGS::u())) == 1 {
                GREEN
            } else {
                RED
            },
        );
        self.draw_string(
            c,
            g,
            d,
            x + 120.0,
            y,
            "B",
            if (self.cpu.get_flag(FLAGS::b())) == 1 {
                GREEN
            } else {
                RED
            },
        );
        self.draw_string(
            c,
            g,
            d,
            x + 136.0,
            y,
            "D",
            if (self.cpu.get_flag(FLAGS::d())) == 1 {
                GREEN
            } else {
                RED
            },
        );
        self.draw_string(
            c,
            g,
            d,
            x + 152.0,
            y,
            "I",
            if (self.cpu.get_flag(FLAGS::i())) == 1 {
                GREEN
            } else {
                RED
            },
        );
        self.draw_string(
            c,
            g,
            d,
            x + 168.0,
            y,
            "Z",
            if (self.cpu.get_flag(FLAGS::z())) == 1 {
                GREEN
            } else {
                RED
            },
        );
        self.draw_string(
            c,
            g,
            d,
            x + 184.0,
            y,
            "C",
            if (self.cpu.get_flag(FLAGS::c())) == 1 {
                GREEN
            } else {
                RED
            },
        );
        self.draw_string(
            c,
            g,
            d,
            x,
            y + 16.0,
            &format!("PC: ${:04x}", self.cpu.pc)[..],
            WHITE,
        );
        self.draw_string(
            c,
            g,
            d,
            x,
            y + 32.0,
            &format!("A: ${:04x} [{}]", self.cpu.acc, self.cpu.acc)[..],
            WHITE,
        );
        self.draw_string(
            c,
            g,
            d,
            x,
            y + 48.0,
            &format!("X: ${:04x} [{}]", self.cpu.x, self.cpu.x)[..],
            WHITE,
        );
        self.draw_string(
            c,
            g,
            d,
            x,
            y + 64.0,
            &format!("Y: ${:04x} [{}]", self.cpu.y, self.cpu.y)[..],
            WHITE,
        );
        self.draw_string(
            c,
            g,
            d,
            x,
            y + 80.0,
            &format!("Stack: ${:04x}", self.cpu.sp)[..],
            WHITE,
        );
    }

    fn draw_code(
        &mut self,
        c: Context,
        g: &mut G2d,
        d: &mut GfxDevice,
        x: f64,
        y: f64,
        n_lines: u32,
    ) {
        let it_a = &self.map_asm.get(self.cpu.pc as usize).unwrap().clone();
        let mut n_line_y = (n_lines >> 1) * 10 + y as u32;
        let end = &self.map_asm[self.map_asm.len() - 1].clone();
        if it_a != end {
            self.draw_string(c, g, d, x, n_line_y as f64, it_a, CYAN);
            while n_line_y < (n_lines * 10) + y as u32 {
                n_line_y += 32;
                let it_a = &self
                    .map_asm
                    .get((self.cpu.pc + 1) as usize)
                    .unwrap()
                    .clone();
                if it_a != end {
                    self.draw_string(c, g, d, x, n_line_y as f64, it_a, WHITE);
                }
            }
        }

        let it_a = &self.map_asm.get(self.cpu.pc as usize).unwrap().clone();
        let mut n_line_y = (n_lines >> 1) * 10 + y as u32;
        if it_a != end {
            self.draw_string(c, g, d, x, n_line_y as f64, it_a, CYAN);
            while n_line_y > y as u32 {
                n_line_y -= 32;
                let it_a = &self
                    .map_asm
                    .get((self.cpu.pc - 1) as usize)
                    .unwrap()
                    .clone();
                if it_a != end {
                    self.draw_string(c, g, d, x, n_line_y as f64, it_a, WHITE);
                }
            }
        }
    }
}

fn main() {
    let bus = Rc::new(RefCell::new(Bus::new()));

    let mut cpu = Cpu::new(Rc::clone(&bus));

    let mut look_up = LookUpTable::new();
    let data = [
        0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9, 0x00,
        0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA,
    ];

    let mut n_offset = 0x8000;
    for i in 0..data.len() {
        cpu.bus.borrow_mut().ram[n_offset] = data[i] as u8;
        n_offset += 1;
    }

    cpu.bus.borrow_mut().ram[0xFFFC] = 0x00;
	cpu.bus.borrow_mut().ram[0xFFFD] = 0x80;


    cpu.reset();

    let map_asm = cpu.disassemble(0x0000, 0xFFFF, &look_up);

    let mut window: PistonWindow = WindowSettings::new("NES 6502 TEST", (1024, 768))
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(60);
    

    let assets = find_folder::Search::Kids(1).for_folder("assets").unwrap();
    let glyphs = window.load_font(assets.join("Roboto-Regular.ttf")).unwrap();

    let mut app = Game::new(glyphs, &mut cpu, map_asm);
    while let Some(e) = window.next() {
        match e {
            Event::Loop(l) => match l {
                Loop::Render(args) => {
                    window.draw_2d(&e, |c, g, d| {
                        clear(BLUE, g);
                        app.draw_ram(c,g,d, 2.0, 48.0, 0x0000, 16, 16);
		                app.draw_ram(c,g,d, 2.0, 332.0, 0x8000, 16, 16);
		                app.draw_cpu(c,g,d,800.0, 24.0);
		                //app.draw_code(c,g,d,800.0, 160.0, 26);


		                app.draw_string(c,g,d, 10.0, 600.0, "SPACE = Step Instruction    R = RESET    I = IRQ    N = NMI", WHITE);
                    });
                }
                _ => {}
            },
            Event::Custom(_, _, _) => {}
            Event::Input(input, dt) => {
                match input {
                    Input::Button(args) => {
                        match args.state {
                            ButtonState::Press => {
                                match args.button {
                                    Button::Keyboard(Key::Space) => {
                                        loop {
                                            app.cpu.clock(&mut look_up);
                                            if app.cpu.complete()  {
                                                break;
                                            }
                                        }
                                    },
                                    Button::Keyboard(Key::R) => {
                                        
                                        app.cpu.reset();
                                    },
                                    Button::Keyboard(Key::I) => {
                                        
                                        app.cpu.irq();
                                    },
                                    Button::Keyboard(Key::N) => {
                                        
                                        app.cpu.nmi()
                                    },
                                    _ => {}
                                }
                            }
                            _ => {}
                        }

                    }
                    _ => {}
                    
                }
            }
        }
    }
}
