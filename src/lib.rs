use std::os::raw::{c_void, c_char};
use libc::size_t;
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

pub struct MemoryModule<'d> {
    // We keep this here just so it lasts for the lifetime of module.
    _src_data: &'d [u8],
    module: HMEMORYMODULE,
}

unsafe impl <'d> Send for MemoryModule<'d> {}
unsafe impl <'d> Sync for MemoryModule<'d> {}

impl <'d> MemoryModule<'d> {
    pub fn new(data: &'d [u8]) -> MemoryModule<'d> {
        let ptr = data.as_ptr();
        let len = data.len();

        let module = unsafe {
            MemoryLoadLibrary(ptr as *const c_void, len as size_t)
        };
        if module == null() {
            panic!("loading library from in-memory data failed");
        };
        MemoryModule {
            _src_data: data,
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

impl <'d> Drop for MemoryModule<'d> {
    fn drop(&mut self) {
        unsafe {
            MemoryFreeLibrary(self.module);
        };
    }
}
