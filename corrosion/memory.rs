extern {
    fn exit(__status : i32);
    fn free(__ptr : *mut std::os::raw::c_void);
    fn malloc(__size : usize) -> *mut std::os::raw::c_void;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut std::os::raw::c_void, usize) -> *mut std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void),
    pub opaque : *mut std::os::raw::c_void,
}

unsafe extern fn DefaultAllocFunc(
    mut opaque : *mut std::os::raw::c_void, mut size : usize
) -> *mut std::os::raw::c_void {
    opaque;
    malloc(size)
}

unsafe extern fn DefaultFreeFunc(
    mut opaque : *mut std::os::raw::c_void,
    mut address : *mut std::os::raw::c_void
) {
    opaque;
    free(address);
}

#[no_mangle]
pub unsafe extern fn BrotliInitMemoryManager(
    mut m : *mut MemoryManager,
    mut
    alloc_func
    :
    unsafe extern fn(*mut std::os::raw::c_void, usize) -> *mut std::os::raw::c_void,
    mut
    free_func
    :
    unsafe extern fn(*mut std::os::raw::c_void, *mut std::os::raw::c_void),
    mut opaque : *mut std::os::raw::c_void
) { if alloc_func == 0 {
        (*m).alloc_func = DefaultAllocFunc;
        (*m).free_func = DefaultFreeFunc;
        (*m).opaque = 0i32 as (*mut std::os::raw::c_void);
    } else {
        (*m).alloc_func = alloc_func;
        (*m).free_func = free_func;
        (*m).opaque = opaque;
    }
}

#[no_mangle]
pub unsafe extern fn BrotliAllocate(
    mut m : *mut MemoryManager, mut n : usize
) -> *mut std::os::raw::c_void {
    let mut result
        : *mut std::os::raw::c_void
        = ((*m).alloc_func)((*m).opaque,n);
    if result.is_null() {
        exit(1i32);
    }
    result
}

#[no_mangle]
pub unsafe extern fn BrotliFree(
    mut m : *mut MemoryManager, mut p : *mut std::os::raw::c_void
) {
    ((*m).free_func)((*m).opaque,p);
}

#[no_mangle]
pub unsafe extern fn BrotliWipeOutMemoryManager(
    mut m : *mut MemoryManager
) {
    m;
}
