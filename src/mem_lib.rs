extern crate winapi;
extern crate wio;

use std::convert::TryInto;
use std::ffi::OsString;
use std::mem;
use std::os::windows::prelude::*;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::HANDLE;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32;
use winapi::um::winnt::PROCESS_VM_READ;

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
    return Process::new(unsafe { OpenProcess(PROCESS_VM_READ, 0, pid) });
}

pub fn get_proc_id_by_name(name: &str) -> u32 {
    let mut process: tlhelp32::PROCESSENTRY32W = unsafe { std::mem::zeroed() };

    // Если не инициализировать dwSize, первый вызов Process32FirstW() упадёт с ошибкой
    // https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/ns-tlhelp32-processentry32#members
    process.dwSize = mem::size_of::<tlhelp32::PROCESSENTRY32W>() as u32;

    // Создаём снепшот всех процессов
    // https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot
    let snapshot = unsafe { tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, 0) };

    // Получаем данные о первом процессе из снепшота, только так мы можем потом вызывать Process32NextW
    // https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw
    if unsafe { tlhelp32::Process32FirstW(snapshot, &mut process) } != 0 {
        // Получаем данные о следующем процессе в снепшоте, до тех пор пока функция не вернула 0
        // https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw
        while unsafe { tlhelp32::Process32NextW(snapshot, &mut process) } != 0 {
            let process_name = OsString::from_wide(&process.szExeFile);

            // process_name
            println!("check process: {}", process_name.to_string_lossy());

            match process_name.into_string() {
                Ok(s) => {
                    if s.contains(name) {
                        return process.th32ProcessID;
                    }
                }
                Err(_) => {
                    println!(
                        "Error converting process name for PID {}",
                        process.th32ProcessID
                    );
                }
            }
        }
    }

    return 0;
}
