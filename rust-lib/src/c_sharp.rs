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
pub struct RustDelegate<IntPtr> {
    handle: RustGCHandle,
    callback: IntPtr,
}

macro_rules! impl_invoke {
    ($param:ident : $arg:ident $(,$params:ident : $args:ident)* -> $ret:ident) => {
        impl_invoke!($($params: $args),* -> $ret);

        #[allow(dead_code)]
        impl<$($args: FfiSafe,)* $arg: FfiSafe> RustDelegate<unsafe extern "C" fn(GCHandlePtr, $($args,)* $arg)> {
            pub fn invoke(&self, $($params : $args,)* $param : $arg) {
                unsafe { (self.callback)(self.handle.ptr, $($params,)* $param) }
            }
        }

        #[allow(dead_code)]
        impl<$($args: FfiSafe,)* $arg: FfiSafe, $ret: FfiSafe> RustDelegate<unsafe extern "C" fn(GCHandlePtr, $($args,)* $arg) ->  $ret> {
            pub fn invoke(&self, $($params : $args,)* $param : $arg) -> $ret {
                unsafe { (self.callback)(self.handle.ptr, $($params,)* $param) }
            }
        }
    };
    (-> $ret:ident) => {
        #[allow(dead_code)]
        impl RustDelegate<unsafe extern "C" fn(GCHandlePtr)> {
            pub fn invoke(&self) {
                unsafe { (self.callback)(self.handle.ptr) }
            }
        }
        #[allow(dead_code)]
        impl<$ret: FfiSafe> RustDelegate<unsafe extern "C" fn(GCHandlePtr) -> $ret> {
            pub fn invoke(&self) -> $ret {
                unsafe { (self.callback)(self.handle.ptr) }
            }
        }
    };
}

// implement invoke method for 0 to 16 FfiSafe argument and an optional return type
// C# only implements Func/Argument for up to 16 Arguments so this should be (more than) enough
impl_invoke!(p1: P1, p2: P2, p3: P3, p4: P4, p5: P5, p6: P6, p7: P7, p8: P8, p9: P9, p10: P10, p11: P11, p12: P12, p13: P13, p14: P14, p15: P15, p16: P16 -> R);

pub trait FfiSafe {}

macro_rules! impl_ffi_safe {
    ($($ty:ty),*) => {
        $(impl FfiSafe for $ty {})*
    };
}

impl_ffi_safe!(u8, i8, u16, i16, u32, i32, u64, i64);

// can't use the macro to generate the struct as well, as then csbindgen won't find the struct definition and skip the definition in C#
macro_rules! delegate {
    ($name:ident ($($tys:ty),*) $(->  $ret:ty)?) => {
        impl std::ops::Deref for $name {
            type Target = RustDelegate<unsafe extern "C" fn(GCHandlePtr, $($tys),*) $(-> $ret)?>;
            fn deref(&self) -> &Self::Target {
                &self.rust_delegate
            }
        }
    };
}

delegate!(SuccessAction());
#[repr(transparent)]
pub struct SuccessAction {
    rust_delegate: RustDelegate<unsafe extern "C" fn(GCHandlePtr)>,
}

delegate!(FailureAction(i32));
#[repr(transparent)]
pub struct FailureAction {
    rust_delegate: RustDelegate<unsafe extern "C" fn(GCHandlePtr, i32)>,
}

delegate!(Function()->u8);
#[repr(transparent)]
pub struct Function {
    rust_delegate: RustDelegate<unsafe extern "C" fn(GCHandlePtr) -> u8>,
}

delegate!(OtherAction(i32, u8));
#[repr(transparent)]
pub struct OtherAction {
    rust_delegate: RustDelegate<unsafe extern "C" fn(GCHandlePtr, i32, u8)>,
}

/*
success must be a GCHandle to an
 */
#[no_mangle]
pub unsafe extern "C" fn fun_with_callbacks(
    success: SuccessAction,
    failure: FailureAction,
    fun: Function,
    other: OtherAction,
) {
    super::normal_rust_fn(
        move || success.invoke(),
        move |val| failure.invoke(val),
        move || fun.invoke(),
        move |v1, v2| other.invoke(v1, v2),
    )
}
