

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