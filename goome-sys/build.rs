fn main() {
    println!("cargo:rerun-if-changed=native/goome.h");
    println!("cargo:rerun-if-changed=native/goome_lib.c");
    let mut b = cc::Build::new();
    b.file("native/goome_lib.c").include("native");
    let c = b.get_compiler();
    if c.is_like_msvc() {
        b.flag("/W3");
    } else {
        b.flag("-std=c99").flag("-Wno-unused-parameter").flag("-Wno-missing-field-initializers");
    }
    b.compile("goome");
    println!("cargo:include={}/native", std::env::var("CARGO_MANIFEST_DIR").unwrap());
}
