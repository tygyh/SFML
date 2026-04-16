fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("channel_remap.cpp")
        .std("c++17")
        .compile("channel_remap");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=channel_remap.h");
    println!("cargo:rerun-if-changed=channel_remap.cpp");
}