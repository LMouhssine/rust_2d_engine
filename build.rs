fn main() {
    println!("cargo:rustc-link-search=/opt/homebrew/Cellar/sdl2/2.30.9/lib");
    println!("cargo:rustc-link-lib=dylib=SDL2-2.0.0");
    
    // Frameworks système nécessaires
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=ForceFeedback");
    println!("cargo:rustc-link-lib=framework=Carbon");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=Foundation");
}
