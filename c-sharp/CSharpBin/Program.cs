// See https://aka.ms/new-console-template for more information
using CsBindgen;

NativeBindings.CallRust(() => Console.WriteLine("Hello, World!"), (val) => Console.WriteLine($"Failed with {val}"));