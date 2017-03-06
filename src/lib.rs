#![no_std]


extern crate memory_area;
extern crate elf_section;


pub mod x86;
pub mod x86_64 {
    pub use x86::*;
}


#[cfg(target_arch = "x86")]
pub use x86::*;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
