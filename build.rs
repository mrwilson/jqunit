fn main() {
    println!("cargo:rustc-link-lib=jq");
    if let Some(_) = option_env!("DYLD_FALLBACK_LIBRARY_PATH") {
        println!("cargo:rustc-link-search=/opt/homebrew/lib")
    }
}
