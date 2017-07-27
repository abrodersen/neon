//! Facilities for working with `nan::AsyncWorker`s.

use raw::Local;
use std::os::raw::c_void;

// Suppress a spurious rustc warning about the use of CMutSlice.
#[allow(improper_ctypes)]
extern "C" {

    /// Mutates the `out` argument provided to refer to a newly created `nan::AsyncWorker` object.
    /// The `callback` argument must be a callback function that will be invoked upon completion.
    /// Returns `false` if the value couldn't be created.
    #[link_name = "Neon_Async_QueueWork"]
    pub fn queue(callback: &mut Local, 
                execCallback: extern fn(*mut c_void, *mut *mut c_void),
                execKernel: *mut c_void,
                resultCallback: extern fn(&mut Local, &mut Local, *mut c_void, *mut c_void),
                resultKernel: *mut c_void,
                dropCallback: extern fn(*mut c_void));
}