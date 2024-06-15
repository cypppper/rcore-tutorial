use log::info;

use crate::batch::{USER_STACK, USER_STACK_SIZE, APP_BASE_ADDRESS, APP_SIZE_LIMIT};

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            info!("write addr is {:#x}, len is {}", buf as usize, len);
            if (((buf as usize)  >= USER_STACK.get_sp() - USER_STACK_SIZE) && ((buf as usize) + len <= USER_STACK.get_sp())) 
            || (((buf as usize) + len <= APP_SIZE_LIMIT + APP_BASE_ADDRESS) && ((buf as usize) >= APP_BASE_ADDRESS)){
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            }else{
                -1 as isize
            }
        },
        _ => {
            -1
            // panic!("Unsupported fd in sys_write!");
        }
    }
}