
use std::mem;
use std::ptr;
use std::os::raw::c_void;

use internal::js::{JsFunction, JsNull, JsValue};
use internal::mem::{Handle, Managed};

use neon_runtime::raw::{Local};
use neon_runtime::async;

pub fn run_async<'a, 'b, T, X, R>(callback: Handle<'a, JsFunction>, execute: X, result: R) 
where 
    X: FnOnce() -> T,
    R: FnOnce(&mut T) -> Result<Handle<'b, JsValue>, Handle<'b, JsValue>>
{
    let mut complete_cb = callback.to_raw();
    let execute_cb = Box::new(execute);
    let result_cb = Box::new(result);

    let execute_kernel: *mut c_void = Box::into_raw(execute_cb) as *mut c_void;
    let result_kernel: *mut c_void = Box::into_raw(result_cb) as *mut c_void;

    unsafe { async::queue(&mut complete_cb, execute_wrapper::<T, X>, execute_kernel, result_wrapper::<'b, T, R>, result_kernel, drop_wrapper::<T>) };
}

extern "C" fn execute_wrapper<T, F>(arg: *mut c_void, state: *mut *mut c_void) 
    where F: FnOnce() -> T
{
    let closure = unsafe { Box::from_raw(arg as *mut F) };
    let result = Box::new(closure());
    let raw = Box::into_raw(result) as *mut c_void; 
    unsafe { ptr::write(state, raw) };
}


extern "C" fn result_wrapper<'a, T, F>(res: &mut Local, err: &mut Local, arg: *mut c_void, state: *mut c_void) 
    where F: FnOnce(&mut T) -> Result<Handle<'a, JsValue>, Handle<'a, JsValue>>
{
    let closure = unsafe { Box::from_raw(arg as *mut F) };

    // avoid droping the value in this function
    let mut value = unsafe { Box::from_raw(state as *mut T) };
    let result = closure(value.as_mut());
    mem::forget(value);

    *res = result.ok().unwrap_or_else(|| JsNull::new().upcast()).to_raw();
    *err = result.err().unwrap_or_else(|| JsNull::new().upcast()).to_raw();
}

extern "C" fn drop_wrapper<T>(state: *mut c_void) {
    // Deallocate the memory
    let _: Box<T> = unsafe { Box::from_raw(state as *mut T) };
}