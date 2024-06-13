// See https://aka.ms/new-console-template for more information
using CsBindgen;

NativeBindings.CallRust(() => Console.WriteLine("Hello, World!"), (val) => Console.WriteLine($"Failed with {val}"), (v1, v2) => Console.WriteLine($"v1 = {v1}, v2 = {v2}"));