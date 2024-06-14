

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace CsBindgen
{

    internal unsafe partial struct RustGCHandle
    {
        public static unsafe RustGCHandle Allocate(object obj)
        {
            unsafe
            {
                return new RustGCHandle
                {
                    ptr = GCHandle.ToIntPtr(GCHandle.Alloc(obj)),
                    drop_callback = &DropGcHandle
                };
            }
        }

        [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
        private static void DropGcHandle(nint ptr)
        {
            GCHandle.FromIntPtr(ptr).Free();

        }
    }

    internal unsafe partial struct RustDelegate
    {
        public static unsafe RustDelegate Create(Action action)
        {
            [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
            static void ActionCallback(nint ptr)
            {
                (GCHandle.FromIntPtr(ptr).Target as Action)!.Invoke();
            }

            unsafe
            {
                return new RustDelegate
                {
                    handle = RustGCHandle.Allocate(action),
                    callback = (IntPtr)(delegate* unmanaged[Cdecl]<nint, void>)&ActionCallback,
                };
            }
        }

        public static unsafe RustDelegate Create(Action<int> action)
        {
            [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
            static void ActionCallback(nint ptr, int val)
            {
                (GCHandle.FromIntPtr(ptr).Target as Action<int>)!.Invoke(val);
            }

            unsafe
            {
                return new RustDelegate
                {
                    handle = RustGCHandle.Allocate(action),
                    callback = (IntPtr)(delegate* unmanaged[Cdecl]<nint, int, void>)&ActionCallback,
                };
            }
        }

        public static unsafe RustDelegate Create(Action<int, byte> action)
        {
            [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
            static void ActionCallback(nint ptr, int v1, byte v2)
            {
                (GCHandle.FromIntPtr(ptr).Target as Action<int, byte>)!.Invoke(v1, v2);
            }

            unsafe
            {
                return new RustDelegate
                {
                    handle = RustGCHandle.Allocate(action),
                    callback = (IntPtr)(delegate* unmanaged[Cdecl]<nint, int, byte, void>)&ActionCallback,
                };
            }
        }

        public static unsafe RustDelegate Create(Func<byte> func)
        {

            [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
            static byte ActionCallback(nint ptr)
            {
                return (GCHandle.FromIntPtr(ptr).Target as Func<byte>)!.Invoke();
            }

            unsafe
            {
                return new RustDelegate
                {
                    handle = RustGCHandle.Allocate(func),
                    callback = (IntPtr)(delegate* unmanaged[Cdecl]<nint, byte>)&ActionCallback,
                };
            }
        }
    }

    internal unsafe partial struct SuccessAction
    {
        public static implicit operator SuccessAction(Action action)
        {
            return new SuccessAction
            {
                rust_delegate = RustDelegate.Create(action)
            };
        }
    }

    internal unsafe partial struct FailureAction
    {
        public static implicit operator FailureAction(Action<int> action)
        {
            return new FailureAction
            {
                rust_delegate = RustDelegate.Create(action)
            };
        }
    }

    internal unsafe partial struct OtherAction
    {
        public static implicit operator OtherAction(Action<int, byte> action)
        {
            return new OtherAction
            {
                rust_delegate = RustDelegate.Create(action)
            };
        }
    }

    internal unsafe partial struct Function
    {
        public static implicit operator Function(Func<byte> action)
        {
            return new Function
            {
                rust_delegate = RustDelegate.Create(action)
            };
        }
    }

    public static class NativeBindings
    {
        public static void CallRust(Action success, Action<int> fail, Func<byte> fun, Action<int, byte> other)
        {
            unsafe
            {
                NativeMethods.fun_with_callbacks(success, fail, fun, other);
            }
        }
    }

}