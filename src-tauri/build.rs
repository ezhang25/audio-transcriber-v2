fn main() {
    println!(
        "cargo:rustc-link-arg-bin=audiotranscriber=-Wl,-rpath,/usr/lib/swift"
    );

    println!(
        "cargo:rustc-link-arg-bin=audiotranscriber=-Wl,-rpath,@executable_path/../Resources/swift"
    );

    tauri_build::build()
}