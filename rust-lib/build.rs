use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    csbindgen::Builder::default()
        .input_extern_file("src/c_sharp.rs")
        .csharp_dll_name("rust_lib")
        .generate_csharp_file("../c-sharp/CSharpBin/NativeMethods.g.cs")?;
    Ok(())
}
