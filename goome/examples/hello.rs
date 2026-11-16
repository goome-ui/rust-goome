use goome::{Draw, Options, Pacer};

fn main() {
    let mut g = match Options::new().title("goome hello").open() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("cant open an overlay: {}", e);
            std::process::exit(1);
        }
    };
    let mons = g.monitors();
    println!("backend: {}", g.backend_name());
    for (i, m) in mons.iter().enumerate() {
        let a = m.area;
        println!("monitor {}: {} {}x{} at {},{} scale {:.2}", i, m.name, a.w, a.h, a.x, a.y, m.scale);
    }
    if g.hotkey("alt+h", 1).is_err() {
        println!("no global hotkeys here: {}", g.error());
    }
    let (bx, mut by) = (mons[0].area.x as f64 + 80.0, mons[0].area.y as f64 + 80.0);
    let (mut clicks, mut hidden, mut typing, mut running) = (0, false, false, true);
    let mut typed = String::new();
    let mut pace = Pacer::new(30);
    while running {
        let wait = pace.wait();
        for ev in g.events(wait) {
            match ev.kind {
                goome::QUIT => running = false,
                goome::DOWN => {
                    println!("click {:.0},{:.0}", ev.x, ev.y);
                    if ev.x > bx + 250.0 && ev.y < by + 30.0 {
                        running = false;
                    } else if ev.y > by + 80.0 {
                        typing = !typing;
                        let _ = g.keyboard(typing);
                    } else {
                        clicks += 1;
                    }
                }
                goome::KEY => {
                    println!("key {} mods {:x} text '{}'", ev.key, ev.mods, ev.text);
                    if ev.key == goome::K_ESC {
                        running = false;
                    } else if ev.key == goome::K_BACKSPACE {
                        typed.pop();
                    } else if !ev.text.is_empty() && typed.len() + ev.text.len() < 40 {
                        typed += &ev.text;
                    }
                }
                goome::HOTKEY => hidden = !hidden,
                goome::CONTROL if ev.msg == "hide" => hidden = !hidden,
                goome::SCROLL => by += ev.dy as f64 * 10.0,
                _ => {}
            }
        }
        let Some(mut c) = g.begin(0) else { continue };
        if hidden {
            continue;
        }
        c.round_rect(bx, by, 280.0, 120.0, 14.0, 0xE0202838);
        c.frame(bx, by, 280.0, 120.0, 2.0, if clicks & 1 == 1 { 0xFF6FDC5F } else { 0xFF8090A0 });
        c.text_big(bx + 16.0, by + 12.0, "hello from goome", 0xFFFFFFFF);
        c.line(bx + 256.0, by + 10.0, bx + 268.0, by + 22.0, 2.0, 0xFFFF6B8B);
        c.line(bx + 268.0, by + 10.0, bx + 256.0, by + 22.0, 2.0, 0xFFFF6B8B);
        c.text(bx + 16.0, by + 46.0, &format!("clicks: {}", clicks), 0xFFC0C8D0);
        c.round_rect(bx + 12.0, by + 80.0, 256.0, 28.0, 8.0, if typing { 0xFF2E4A30 } else { 0xFF303848 });
        let line = if !typing {
            "click here to type"
        } else if typed.is_empty() {
            "type something..."
        } else {
            &typed
        };
        c.text(bx + 22.0, by + 86.0, line, 0xFFFFFFFF);
        c.hit(bx as i32, by as i32, 280, 120);
    }
}
