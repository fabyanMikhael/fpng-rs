fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src"); // include path
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path]).build()?;
    // This assumes all your C++ bindings are in main.rs
    b.files(&["src/fpng-rs.cpp", "src/fpng.cpp"]) // add any other C++ files you need
        .flag_if_supported("-std=c++20")
        .flag_if_supported("-Ofast")
        .flag_if_supported("-march=native")
        .flag_if_supported("-mfpmath=sse")
        .flag_if_supported("-msse4.1")
        .flag_if_supported("-mpclmul")
        .flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-w")
        .compile("fpng-rs"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/fpng-rs.h");
    println!("cargo:rerun-if-changed=src/fpng-rs.cpp");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
