# goome

Safe Rust for [goome](https://github.com/goome-ui/c-goome): a see-through overlay on top of everything where only the parts you draw take clicks, normal desktop app windows, or offscreen frames you hand to your own engine. Plus drawing, images, fonts and a whole immediate mode ui kit.

```rust,no_run
use goome::{Draw, Goome, QUIT};

let mut g = Goome::open(&goome::Options::new().title("my overlay")).unwrap();
let mut running = true;
while running {
    for ev in g.events(16) {
        if ev.kind == QUIT {
            running = false;
        }
    }
    if let Some(mut c) = g.begin(0) {
        c.round_rect(100.0, 100.0, 220.0, 60.0, 12.0, 0xE0202838);
        c.text(118.0, 120.0, "im on top of everything", 0xFFFFFFFF);
        c.hit(100, 100, 220, 60);
    }
}
```

The C library gets built from the vendored goome.h by [goome-sys](https://github.com/goome-ui/rust-goome/tree/main/goome-sys), so all you need is a C compiler. More in the [repo](https://github.com/goome-ui/rust-goome) and the [docs](https://goome-ui.github.io/docs/).

GPL-3.0, same as goome.
