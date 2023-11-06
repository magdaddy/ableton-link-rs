use cmake::Config;
use bindgen::builder;

#[cfg(target_os = "macos")]
fn link_cpp() {
    // IMPORTANT!!! otherwise linker errors, apparently only on macOS
    println!("cargo:rustc-link-lib=c++");  
}

#[cfg(not(target_os = "macos"))]
fn link_cpp() {
    println!("cargo:rustc-link-lib=stdc++");  
}

fn main() {
    // cmake
    // Builds the project in the directory located in `libfoo`, installing it
    // into $OUT_DIR
    let dst = Config::new("c-wrapper")
                // .cxxflag("-fno-rtti")
                // .no_build_target(true)
                .build_target("linkrs")
                .build();
    let builddir = dst.join("build");
    println!("cargo:rustc-link-search=native={}", builddir.display());
    println!("cargo:rustc-link-lib=static=linkrs");
    link_cpp();

    // bindgen
    let bindings = builder()
                    .header("c-wrapper/link_rs.h")
                    .allowlist_function("Link_.*")
                    .allowlist_function("SessionState_.*")
                    .allowlist_function("Clock_.*")
                    .generate()
                    .expect("generate bindings");
    let outfile = dst.join("link_rs.rs");
    bindings.write_to_file(outfile).expect("write bindings to file");

}