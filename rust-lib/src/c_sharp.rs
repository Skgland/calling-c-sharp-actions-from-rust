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

// csbindgen doesn't translate this as a generic struct, we cheat by naming the generic parameter IntPtr to get the C# IntPtr type,
// as we can't use generics there anyways due to https://github.com/dotnet/runtime/issues/13627 which makes this somewhat akward
#[repr(C)]
pub struct RustAction<IntPtr> {
    handle: RustGCHandle,
    callback: IntPtr,
}

macro_rules! impl_invoke {
    ($param:ident : $arg:ident $(,$params:ident : $args:ident)* ) => {
        impl_invoke!($($params: $args),*);

        #[allow(dead_code)]
        impl<$($args: FfiSafe,)* $arg: FfiSafe> RustAction<unsafe extern "C" fn(GCHandlePtr, $($args,)* $arg)> {
            pub fn invoke(&self, $($params : $args,)* $param : $arg) {
                unsafe { (self.callback)(self.handle.ptr, $($params,)* $param) };
            }
        }
    };
    () => {
        #[allow(dead_code)]
        impl RustAction<unsafe extern "C" fn(GCHandlePtr)> {
            pub fn invoke(&self) {
                unsafe { (self.callback)(self.handle.ptr) };
            }
        }
    };
}

// implement invoke method for 0 to 4 FfiSafe argument
impl_invoke!(p1: P1, p2: P2, p3: P3, p4: P4);

pub trait FfiSafe {}

macro_rules! impl_ffi_safe {
    ($($ty:ty),*) => {
        $(impl FfiSafe for $ty {})*
    };
}

impl_ffi_safe!(u8, i8, u16, i16, u32, i32, u64, i64);

type SuccessFunction = unsafe extern "C" fn(GCHandlePtr);
type FailureFunction = unsafe extern "C" fn(GCHandlePtr, i32);
type OtherFunction = unsafe extern "C" fn(GCHandlePtr, i32, u8);

type SuccessAction = RustAction<SuccessFunction>;
type FailureAction = RustAction<FailureFunction>;
type OtherAction = RustAction<OtherFunction>;

/*
success must be a GCHandle to an
 */
#[no_mangle]
pub unsafe extern "C" fn fun_with_callbacks(
    success: SuccessAction,
    failure: FailureAction,
    other: OtherAction,
) {
    super::normal_rust_fn(
        move || success.invoke(),
        move |val| failure.invoke(val),
        move |v1, v2| other.invoke(v1, v2),
    )
}
