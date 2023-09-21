use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    csbindgen::Builder::default()
        .input_extern_file("src/c_sharp.rs")
        .csharp_dll_name("rust_lib")
        .generate_csharp_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/dotnet/NativeMethods.g.cs"
        ))?;
    std::thread::sleep(Duration::from_secs(1));
    Ok(())
}
