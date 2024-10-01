use avm::processor::Cpu;

fn main() {
    let cpu = Cpu::with_memory_capacity(1024);

    println!("Init: {}", &cpu);
}
