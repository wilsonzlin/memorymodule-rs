use memorymodule_rs::*;
use std::os::raw::c_int;
use std::mem;

const MYDLL: &'static [u8] = include_bytes!("../../mydll/mydll.dll");

type AddFn = extern "C" fn(c_int, c_int) -> c_int;

fn main() {
    let mm = MemoryModule::new(MYDLL.to_vec());
    let add = unsafe {
        mem::transmute::<_, AddFn>(mm.get_function("add"))
    };
    println!("Add 3 5 = {}", add(3, 5));
}
