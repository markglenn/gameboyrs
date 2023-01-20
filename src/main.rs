use crate::hardware::cartridge::Cartridge;

pub mod hardware;

fn main() {
    let cartridge = Cartridge::load("priv/roms/cpu_instrs.gb").unwrap();

    println!("Hello, world!: {}", cartridge.title());
}
