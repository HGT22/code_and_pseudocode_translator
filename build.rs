fn main() {
    // Set library name based on target platform
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    match target_os.as_str() {
        "android" => {
            println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libcode_translator_android.so");
        }
        "linux" => {
            println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libcode_translator_linux.so");
        }
        _ => {}
    }
}
