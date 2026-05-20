use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::*;
use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
use std::mem::size_of;
use std::os::raw::c_void;
use std::mem::MaybeUninit;
// use std::ffi::CStr;
// use std::os::raw::c_char;

pub struct Memory {
    pub process_id: u32,
    pub handle: HANDLE
}


impl Memory {
    pub unsafe fn build(process_name: &str) -> Self {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

            let mut entry: PROCESSENTRY32 = PROCESSENTRY32::default();
            entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

            while Process32Next(
                snapshot,
                &mut entry
            ) > 0 {
                // dbg!(CStr::from_ptr(entry.szExeFile.as_ptr() as *const c_char).to_string_lossy());

                if &entry.szExeFile.map(|x| x as u8)[..process_name.len()] == process_name.as_bytes() {
                    let process_id = entry.th32ProcessID;
                    let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);

                    return Memory {
                        process_id,
                        handle,
                    }
                }
            }

            panic!("Could not open a handle to `{process_name}`");
        }
    }
    
    pub unsafe fn get_module_base(&self, module_name: &str) -> *const c_void {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.process_id);

            let mut entry: MODULEENTRY32 = MODULEENTRY32::default();
            entry.dwSize = size_of::<MODULEENTRY32>() as u32;

            while Module32Next(
                snapshot,
                &mut entry
            ) > 0 {
                // dbg!(CStr::from_ptr(entry.szModule.as_ptr() as *const c_char).to_string_lossy());

                if &entry.szModule.map(|x| x as u8)[..module_name.len()] == module_name.as_bytes() {
                    return entry.modBaseAddr as *const c_void;
                }
            }
            
            panic!("Could not find the base address for the module `{module_name}`");
        }
    }

    pub unsafe fn read<T>(&self, address: *const c_void) -> T {
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();

            ReadProcessMemory(
                self.handle,
                address,
                value.as_mut_ptr() as *mut c_void,
                size_of::<T>(),
                std::ptr::null_mut(),
            );

            value.assume_init()
        }
    }

    pub unsafe fn read_ptr(&self, address: *const c_void) -> *const c_void {
        unsafe {
            self.read::<u32>(address) as *const c_void
        }
    }

    pub unsafe fn read_il2cpp_string(&self, string_ptr: *const c_void) -> String {
      unsafe {
          if string_ptr.is_null() {
              return String::new();
          }
          let str__length: i32 = self.read::<i32>(string_ptr.add(0x08));
          if str__length <= 0 || str__length > 256 {
              return String::new(); // null/garbage guard
          }
          let mut utf16 = Vec::with_capacity(str__length as usize);
          for i in 0..str__length as usize {
              utf16.push(self.read::<u16>(string_ptr.add(0x0C + i * 2)));
          }
          String::from_utf16_lossy(&utf16)
      }
  }

}

impl Drop for Memory {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut() {
            unsafe {
                CloseHandle(self.handle);
            }
        }
    }
}