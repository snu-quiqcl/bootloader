#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(compiler_builtins)]
#![feature(rustc_private)]

global_asm!(include_str!("init.S"));

extern crate compiler_builtins;

mod uart;
mod elf;

use elf::*;
use uart::read;
use compiler_builtins::mem::memcpy;
use core::mem::transmute;

#[no_mangle]
pub unsafe extern "C" fn entry() -> ! {
    uart::uart_init();
    wait_magic();
    
    let mut file_len = read() as u32;
    file_len |= (read() as u32) << 8; 
    file_len |= (read() as u32) << 16;
    file_len |= (read() as u32) << 24;
    
    let elf_addr = 0x20000000 as *mut u8;

    println!("File length: {}", file_len);
    for i in 0..file_len {
        *elf_addr.offset(i as isize) = read();
    }
    println!("File transfer done.");

    let elf_hdr = &*(elf_addr as *const ElfHeader);
    if elf_hdr.e_ident[0] != 0x464c457f{
        panic!("Not a valid elf");
    }
    let prog_hdr = elf_addr.offset(elf_hdr.e_phoff as isize) as *const ProgramHeader;
    for ph_num in 0..elf_hdr.e_phnum {
        let ph = &*prog_hdr.offset(ph_num as isize);
        if ph.p_type != 1 {
            continue;
        }
        println!("Load to {:x} ({:x} bytes)", ph.p_paddr, ph.p_filesz);
        memcpy(ph.p_paddr as *mut u8, elf_addr.offset(ph.p_offset as isize), ph.p_filesz as usize);
    }

    println!("Load complete");
    println!("Jump to {:x}", elf_hdr.e_entry);

    let entry = transmute::<u32, fn() -> !>(elf_hdr.e_entry);
    entry()
}

/// Wait magic number 1111
fn wait_magic() {
    let mut count = 0;
    loop {
        if read() == 1 {
            count += 1;
        } else {
            print!("1");
            count = 0;
        }
        if count == 4 {
            break;
        }
    }
}



#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    unsafe { loop { asm!("wfe") } }
}