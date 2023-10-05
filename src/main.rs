use std::fs;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};
use minifb::{Key, Window, WindowOptions};
use lib_dmg::cpu::CPU;

const ENLARGEMENT_FACTOR: usize = 2;
const WINDOW_DIMENSIONS: [usize; 2] = [(160 * ENLARGEMENT_FACTOR), (144 * ENLARGEMENT_FACTOR)];

fn main() {
    let mut data = Vec::new();
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\01-special.gb").expect("Failed to open file");         //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\02-interrupts.gb").expect("Failed to open file");
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\03-op sp,hl.gb").expect("Failed to open file");        //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\04-op r,imm.gb").expect("Failed to open file");        //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\05-op rp.gb").expect("Failed to open file");           //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\06-ld r,r.gb").expect("Failed to open file");          //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\07-jr,jp,call,ret,rst.gb").expect("Failed to open file"); //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\08-misc instrs.gb").expect("Failed to open file");     //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\09-op r,r.gb").expect("Failed to open file");          //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\10-bit ops.gb").expect("Failed to open file");         //PASSED
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\cpu_instrs\\individual\\11-op a,(hl).gb").expect("Failed to open file");       //PASSED
    //let mut file = fs::File::open("C:\\Dev\\dmg_emulator\\src\\CRASH.gb").expect("Failed to open file");
    let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\tetris\\tetris.gb").expect("Failed to open file");
    //let mut file = fs::File::open("C:\\Dev\\gb_emulator\\roms\\supermario\\supermario.gb").expect("Failed to open file");
    file.read_to_end(&mut data).expect("Failed to read file");


    let mut cpu: CPU = CPU::new(&data);
    let window = Window::new(
        "DMG-01",
        WINDOW_DIMENSIONS[0],
        WINDOW_DIMENSIONS[1],
        WindowOptions::default(),
    )
        .unwrap();

    run(cpu, window)

}

const ONE_SECOND_IN_MICROS: usize = 1000000000;
const ONE_SECOND_IN_CYCLES: usize = 4190000;
const ONE_FRAME_IN_CYCLES: usize = 70224;
const NUMBER_OF_PIXELS: usize = 23040;

fn run(mut cpu: CPU, mut window: Window) {
    let mut buffer = [0; NUMBER_OF_PIXELS];
    let mut cycles_elapsed_in_frame = 0usize;
    let mut now = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_down(Key::A) {
            cpu.bus.io.joypad.a = true;
            cpu.bus.io.interrupt_enable.joypad = true;
        }else {
            cpu.bus.io.joypad.a = false;
            cpu.bus.io.interrupt_enable.joypad = false;
        }
        if window.is_key_down(Key::S) {
            cpu.bus.io.joypad.start = true;
            cpu.bus.io.interrupt_enable.joypad = true;
        } else {
            cpu.bus.io.joypad.start = false;
            cpu.bus.io.interrupt_enable.joypad = false;
        }
        if window.is_key_down(Key::Right) {
            cpu.bus.io.joypad.right = true;
            cpu.bus.io.interrupt_enable.joypad = true;
        } else {
            cpu.bus.io.joypad.right = false;
            cpu.bus.io.interrupt_enable.joypad = false;
        }
        if window.is_key_down(Key::Left) {
            cpu.bus.io.joypad.left = true;
            cpu.bus.io.interrupt_enable.joypad = true;
        } else {
            cpu.bus.io.joypad.left = false;
            cpu.bus.io.interrupt_enable.joypad = false;
        }
        if window.is_key_down(Key::Up) {
            cpu.bus.io.joypad.up = true;
            cpu.bus.io.interrupt_enable.joypad = true;
        } else {
            cpu.bus.io.joypad.up = false;
            cpu.bus.io.interrupt_enable.joypad = false;
        }
        if window.is_key_down(Key::Down) {
            cpu.bus.io.joypad.down = true;
            cpu.bus.io.interrupt_enable.joypad = true;
        } else {
            cpu.bus.io.joypad.down = false;
            cpu.bus.io.interrupt_enable.joypad = false;
        }

        let time_delta = now.elapsed().subsec_nanos();
        now = Instant::now();
        let delta = time_delta as f64 / ONE_SECOND_IN_MICROS as f64;
        let cycles_to_run = delta * ONE_SECOND_IN_CYCLES as f64;

        let mut cycles_elapsed = 0;
        while cycles_elapsed <= cycles_to_run as usize {
            cycles_elapsed += cpu.step() as usize;

        }
        cycles_elapsed_in_frame += cycles_elapsed;



        // TODO: Consider updating buffer after every line is rendered.
        if cycles_elapsed_in_frame >= ONE_FRAME_IN_CYCLES {
            for (i, pixel) in cpu.bus.gpu.canvas_buffer.chunks(4).enumerate() {
                buffer[i] = (pixel[3] as u32) << 24
                    | (pixel[2] as u32) << 16
                    | (pixel[1] as u32) << 8
                    | (pixel[0] as u32)
            }
            window.update_with_buffer(&buffer, 160, 144).unwrap();
            cycles_elapsed_in_frame = 0;
        } else {
            sleep(Duration::from_nanos(2))
        }
    }
}
