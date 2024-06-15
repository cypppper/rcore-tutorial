#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
pub mod batch;
mod sync;
mod logging;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as usize as *mut u8, 
            ebss as usize - sbss as usize,
        ).fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}