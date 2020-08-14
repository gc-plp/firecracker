//
extern crate vm_memory;

use vm_memory::{Bytes, GuestAddress, GuestMemory, GuestMemoryMmap, GuestMemoryRegion};

pub fn insert_sw_breakpoint(mem: &GuestMemoryMmap, addr: GuestAddress) {
    println!("[GDB] Setting software breakpoint at {:x}", addr.0);

    // Declare the debug interrupt instruction
    // See: https://en.wikipedia.org/wiki/INT_(x86_instruction)#INT3
    let int3: u8 = 0xcc;

    // Save the instruction at the given address
    // TODO

    // Get the physical address
    //let reg = mem.find_region(addr).unwrap();
    //println!("asd {}", reg.start_addr().0);

    // Write the interrupt exception to the given address
    mem.write_obj(int3, addr).unwrap();
}
