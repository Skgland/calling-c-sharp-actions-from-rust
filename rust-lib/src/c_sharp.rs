type GCHandlePtr = isize;

#[repr(C)]
pub struct RustGCHandle {
    ptr: GCHandlePtr,
    drop_callback: extern "C" fn(GCHandlePtr),
}

impl Drop for RustGCHandle {
    fn drop(&mut self) {
        (self.drop_callback)(self.ptr);
    }
}

#[repr(C)]
pub struct SuccessAction {
    handle: RustGCHandle,
    callback: unsafe extern "C" fn(GCHandlePtr),
}

impl SuccessAction {
    pub fn invoke(&self) {
        unsafe { (self.callback)(self.handle.ptr) };
    }
}

#[repr(C)]
pub struct FailureAction {
    handle: RustGCHandle,
    callback: unsafe extern "C" fn(GCHandlePtr, i32),
}

impl FailureAction {
    pub fn invoke(&self, val: i32) {
        unsafe { (self.callback)(self.handle.ptr, val) };
    }
}

/*
success must be a GCHandle to an
 */
#[no_mangle]
pub unsafe extern "C" fn fun_with_callbacks(success: SuccessAction, failure: FailureAction) {
    super::normal_rust_fn(move || success.invoke(), move |val| failure.invoke(val))
}
