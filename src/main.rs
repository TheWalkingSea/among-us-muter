
mod memory;
mod cheat;

use memory::Memory;

use cheat::AmongUsCheat;

fn main()  {
    unsafe {
        let mem = Memory::build("Among Us.exe");

        println!("The process id of `Among Us.exe` is {}", mem.process_id);

        let base = mem.get_module_base("GameAssembly.dll");

        println!("The module base for `GameAssembly.dll` is {base:?}");

        mem.cheat_loop(base)
    }
}
