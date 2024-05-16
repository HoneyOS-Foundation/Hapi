use std::os::raw::c_void;

/// Allocate a block of memory.
/// # Safety
/// No
pub unsafe fn alloc<T>() -> *mut T {
    let size = std::mem::size_of::<T>() as u32;
    unsafe { crate::ffi::hapi_mem_alloc(size) as *mut T }
}

/// Reallocate a block of memory.
/// # Safety
/// No
pub unsafe fn realloc<T>(old: *mut T) -> *mut T {
    let size = std::mem::size_of::<T>() as u32;
    unsafe { crate::ffi::hapi_mem_realloc(old as *mut c_void, size) as *mut T }
}

/// Free a block of memory
pub unsafe fn free<T>(ptr: *mut T) {
    unsafe { crate::ffi::hapi_mem_free(ptr as *mut c_void) }
}
