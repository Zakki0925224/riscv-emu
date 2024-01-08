mod cpu;
mod emulator;
mod ram;

use emulator::Emulator;

fn main() {
    let ram_size = 1024; // 1KB
    let emulator = Emulator::new(ram_size);
    println!("{:?}", emulator);
}
