fn main() {
    // Add common search paths for macOS (Homebrew, etc.)
    println!("cargo:rustc-link-search=/opt/homebrew/lib");
    println!("cargo:rustc-link-search=/usr/local/lib");
    
    // We don't need to specify -lSDL2 here usually if the crate does it, 
    // but adding search paths helps.
}
