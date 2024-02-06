mod cpu;
mod emulator;
mod ram;
mod registers;

use emulator::Emulator;

fn main() {
    let ram_size = 1024; // 1KB
    let mut emulator = Emulator::new(ram_size);
    emulator.run();
}
