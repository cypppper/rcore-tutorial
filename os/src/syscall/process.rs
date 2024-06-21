use crate::task::{
    current_user_token, exit_current_and_run_next, suspend_current_and_run_next
    
};
use crate::timer::get_time_us;
use crate::mm::{PageTable, PhysAddr, VirtAddr, VirtPageNum};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let cur = current_user_token();
    let _us = get_time_us();
    let pt = PageTable::from_token(cur);
    let timeval_va = VirtAddr::from(_ts as usize);
    let timeval_vpn: VirtPageNum = timeval_va.floor().into();
    let ppn = pt
        .translate(timeval_vpn)
        .unwrap()
        .ppn();
    let pa: PhysAddr = ppn.clone().into();
    let timeval_usize = pa.0 + timeval_va.page_offset();
    let kernel_ts = timeval_usize as *mut TimeVal;
    unsafe { 
        *kernel_ts = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        }; 
    }
    // unsafe {
    //     *ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    0
}