

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

    internal unsafe partial struct RustAction
    {
        public static unsafe RustAction Create(Action action)
        {

            unsafe
            {
                return new RustAction
                {
                    handle = RustGCHandle.Allocate(action),
                    callback = (IntPtr)GetCallback(action),
                };
            }
        }

        public static unsafe RustAction Create<T>(Action<T> action)
        {
            unsafe
            {
                return new RustAction
                {
                    handle = RustGCHandle.Allocate(action),
                    callback = (IntPtr)GetCallback((dynamic)action),
                };
            }
        }

        public static unsafe RustAction Create<T1, T2>(Action<T1, T2> action)
        {
            unsafe
            {
                return new RustAction
                {
                    handle = RustGCHandle.Allocate(action),
                    callback = (IntPtr)GetCallback((dynamic)action),
                };
            }
        }

        private static unsafe delegate* unmanaged[Cdecl]<nint, void> GetCallback(Action action)
        {
            [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
            static void ActionCallback(nint ptr)
            {
                (GCHandle.FromIntPtr(ptr).Target as Action)!.Invoke();
            }

            return &ActionCallback;
        }

        private static unsafe delegate* unmanaged[Cdecl]<nint, int, void> GetCallback(Action<int> action)
        {
            [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
            static void ActionCallback(nint ptr, int val)
            {
                (GCHandle.FromIntPtr(ptr).Target as Action<int>)!.Invoke(val);
            }

            return &ActionCallback;
        }

        private static unsafe delegate* unmanaged[Cdecl]<nint, int, byte, void> GetCallback(Action<int, byte> action)
        {
            [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
            static void ActionCallback(nint ptr, int v1, byte v2)
            {
                (GCHandle.FromIntPtr(ptr).Target as Action<int, byte>)!.Invoke(v1, v2);
            }

            return &ActionCallback;
        }
    }

    public static class NativeBindings
    {
        public static void CallRust(Action success, Action<int> fail, Action<int, byte> other)
        {
            unsafe
            {
                NativeMethods.fun_with_callbacks(RustAction.Create(success), RustAction.Create(fail), RustAction.Create(other));
            }
        }
    }

}