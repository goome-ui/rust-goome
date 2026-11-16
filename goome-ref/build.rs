fn main() {
    let mut b = cc::Build::new();
    b.file("c/ref.c").include("../goome-sys/native");
    if !b.get_compiler().is_like_msvc() {
        b.flag("-std=c99");
    }
    b.compile("goome_ref");
    println!("cargo:rerun-if-changed=c/ref.c");
}
