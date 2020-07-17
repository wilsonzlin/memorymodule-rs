use std::os::raw::{c_void, c_char};
use libc::size_t;
use std::mem;
use std::ffi::CString;
use std::ptr::null;

type FARPROC = *const c_void;
type HMEMORYMODULE = *const c_void;
type LPCSTR = *const c_char;

extern "C" {
    fn MemoryLoadLibrary(data: *const c_void, len: size_t) -> HMEMORYMODULE;
    fn MemoryGetProcAddress(module: HMEMORYMODULE, name: LPCSTR) -> FARPROC;
    fn MemoryFreeLibrary(module: HMEMORYMODULE) -> ();
}

pub struct MemoryModule {
    src_data: *mut u8,
    src_data_len: usize,
    module: HMEMORYMODULE,
}

impl MemoryModule {
    pub fn new(mut data: Vec<u8>) -> MemoryModule {
        data.shrink_to_fit();


        let ptr = data.as_mut_ptr();
        let len = data.len();

        mem::forget(data);

        let module = unsafe {
            MemoryLoadLibrary(ptr as *const c_void, len as size_t)
        };
        if module == null() {
            panic!("loading library from in-memory data failed");
        };
        MemoryModule {
            src_data: ptr,
            src_data_len: len,
            module,
        }
    }

    pub fn get_function(&self, name: &str) -> FARPROC {
        let cname = CString::new(name).expect("create C string from name");
        unsafe {
            let res = MemoryGetProcAddress(self.module, cname.as_ptr());
            if res == null() {
                panic!("getting function failed");
            };
            res
        }
    }
}

impl Drop for MemoryModule {
    fn drop(&mut self) {
        unsafe {
            MemoryFreeLibrary(self.module);
            // Take ownership of original Vec<u8> so we can destroy it.
            Vec::from_raw_parts(self.src_data, self.src_data_len, self.src_data_len);
        };
    }
}
