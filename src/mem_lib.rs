extern crate winapi;
extern crate wio;

use winapi::shared::ntdef::HANDLE;
use winapi::ctypes::{c_void};
use winapi::um::winnt::{PROCESS_VM_READ};
use winapi::um::{tlhelp32};
use std::mem;
use std::ffi::OsString;
use std::os::windows::prelude::*;
use winapi::um::memoryapi::{ReadProcessMemory};
use winapi::um::processthreadsapi::{OpenProcess};
use std::convert::TryInto;

pub struct Process {
    handler: HANDLE,
}

impl Process {
    fn new(m_handler: HANDLE) -> Process {
        return Process { handler: m_handler };
    }

    pub fn read_address(self, addres: &str) -> u64 {
        let num = unsafe { std::mem::zeroed() };
        let addr = u64::from_str_radix(&addres[2..], 16).unwrap();
        unsafe {
            ReadProcessMemory(
                self.handler,
                addr as *mut c_void,
                num,
                u64::max_value().try_into().unwrap(),
                0 as *mut usize,
            )
        };

        return num as u64;
    }
}

pub fn get_proc(pid: u32) -> Process {
    return Process::new(unsafe{OpenProcess(PROCESS_VM_READ, 0, pid)});
}

pub fn get_proc_id_by_name(name: &str) -> u32 {

    let mut process: tlhelp32::PROCESSENTRY32W = unsafe{ std::mem::zeroed() };
    process.dwSize = mem::size_of::<tlhelp32::PROCESSENTRY32W>() as u32;

    //Make a Snanshot of all the current proccess.
    let snapshot = unsafe{tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, 0)};

    //Get the first proccess and store it in proccess variable.
    if unsafe{tlhelp32::Process32FirstW(snapshot, &mut process)} != 0{

        //Take the next procces if posible.
        while unsafe{tlhelp32::Process32NextW(snapshot, &mut process)} != 0 {

            let process_name = OsString::from_wide(&process.szExeFile);

            match process_name.into_string() {
                Ok(s) => {
                    if s.contains(name) {
                        return process.th32ProcessID;
                    }
                },
                Err(_) => {
                    println!("Error converting process name for PID {}", process.th32ProcessID);
                }
            }
        }
    }

    return 0;
}
