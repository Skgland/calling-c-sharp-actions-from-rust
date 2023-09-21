

using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace CsBindgen
{

    public static class NativeBindings
    {
        [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
        private static void DropGcHandle(nint ptr)
        {
            GCHandle.FromIntPtr(ptr).Free();

        }

        [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
        private static void NoArgAction(nint ptr)
        {
            (GCHandle.FromIntPtr(ptr).Target as Action).Invoke();

        }

        [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
        private static void Int32ArgAction(nint ptr, int val)
        {
            (GCHandle.FromIntPtr(ptr).Target as Action<int>).Invoke(val);

        }

        public static void CallRust(Action success, Action<Int32> fail)
        {
            var successHandle = GCHandle.Alloc(success);
            var failiurHandle = GCHandle.Alloc(fail);

            unsafe
            {
                var rSuccess = new SuccessAction()
                {
                    handle = new RustGCHandle()
                    {
                        ptr = GCHandle.ToIntPtr(successHandle),
                        drop_callback = &DropGcHandle
                    },
                    callback = &NoArgAction,

                };

                var rFail = new FailureAction()
                {
                    handle = new RustGCHandle()
                    {
                        ptr = GCHandle.ToIntPtr(failiurHandle),
                        drop_callback = &DropGcHandle
                    },
                    callback = &Int32ArgAction,
                };

                NativeMethods.fun_with_callbacks(rSuccess, rFail);
            }
        }
    }

}