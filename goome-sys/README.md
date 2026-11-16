# goome-sys

Raw `extern "C"` bindings to [goome](https://github.com/goome-ui/c-goome), plus a build script that compiles the vendored `goome.h` with the `cc` crate. You probably want the safe [goome](https://github.com/goome-ui/rust-goome/tree/main/goome) crate instead.

Everything in `native/goome.h` is declared here with the same names. `native/VERSION` has the c-goome commit it came from.

GPL-3.0, same as goome.
