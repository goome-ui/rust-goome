use goome::{Draw, Event, Goome, Options};

#[test]
fn combos() {
    assert_eq!(goome::parse_combo("alt+h").unwrap(), ('h' as i32, goome::ALT));
    assert_eq!(goome::parse_combo("Ctrl+Shift+F5").unwrap(), (goome::K_F1 + 4, goome::CTRL | goome::SHIFT));
    assert_eq!(goome::parse_combo("alt+?").unwrap(), ('/' as i32, goome::ALT | goome::SHIFT));
    assert_eq!(goome::parse_combo("super+space").unwrap(), (' ' as i32, goome::SUPER));
    assert_eq!(goome::parse_combo("cmd+left").unwrap(), (goome::K_LEFT, goome::SUPER));
    assert_eq!(goome::parse_combo("H").unwrap(), ('h' as i32, goome::SHIFT));
    for bad in ["alt+", "alt+f13", ""] {
        assert!(goome::parse_combo(bad).is_err(), "{}", bad);
    }
}

#[test]
fn pacer() {
    let mut p = goome::Pacer::new(50);
    assert_eq!(p.wait(), 0);
    let w = p.wait();
    assert!(w > 0 && w <= 20, "{}", w);
    let t = goome::now_ms();
    assert!(goome::now_ms() >= t);
    assert!(!goome::strerror(goome::ENOTSUP).is_empty());
}

const SCRIPT: &str = "screen 640x360 2; 1 click 20,20; 1 click 300,300; 2 hotkey alt+j; 3 key Hi\\b; 4 send ping; 5 scroll 30,30 -1; 6 hold 25,25 8; end 10";

#[test]
fn scripted_run() {
    let mut g = Goome::test(SCRIPT).unwrap();
    assert_eq!(g.backend_name().split_whitespace().next(), Some("test"));
    let mons = g.monitors();
    assert_eq!(mons.len(), 1);
    assert_eq!((mons[0].area.w, mons[0].scale, mons[0].pw), (640, 2.0, 1280));
    g.hotkey("alt+j", 7).unwrap();
    g.keyboard(true).unwrap();
    let (mut down, mut up, mut hot, mut keys, mut ctl, mut scroll, mut held) = (0, 0, 0, 0, 0, 0, 0);
    let mut text = String::new();
    let (mut quit, mut frames) = (false, 0);
    while !quit && frames < 100 {
        {
            let mut c = g.begin(0).expect("a canvas");
            assert_eq!((c.scale(), c.w()), (2.0, 1280));
            c.rect(10.0, 10.0, 40.0, 40.0, 0xFF336699);
            c.hit(10, 10, 40, 40);
        }
        frames += 1;
        if let Some((_, _, b)) = g.pointer() {
            if b != 0 {
                held += 1;
            }
        }
        for ev in g.events(1) {
            match ev.kind {
                goome::DOWN => down += 1,
                goome::UP => up += 1,
                goome::HOTKEY if ev.id == 7 => hot += 1,
                goome::KEY => {
                    keys += 1;
                    text += &ev.text;
                }
                goome::CONTROL if ev.msg == "ping" => ctl += 1,
                goome::SCROLL if ev.dy == -1 => scroll += 1,
                goome::QUIT => quit = true,
                _ => {}
            }
        }
    }
    assert_eq!((down, up), (2, 2));
    assert_eq!((hot, keys, text.as_str(), ctl, scroll), (1, 3, "Hi", 1, 1));
    assert!(quit);
    assert!(held >= 1);

    let img = g.capture(0, Some(goome::Rect::new(10, 10, 40, 40))).unwrap();
    assert_eq!((img.w, img.h, img.has_self), (80, 80, true));
    assert_eq!(img.pixel(0, 0), 0xFF336699);
    assert_eq!(img.row(79)[79], img.pixel(79, 79));
    assert_eq!(g.alpha_at(20.0, 20.0), 255);
    assert_eq!(g.alpha_at(200.0, 200.0), 0);
    let all = g.capture(0, None).unwrap();
    assert_eq!((all.w, all.h), (1280, 720));
    let dir = std::env::var("CARGO_TARGET_TMPDIR").unwrap_or_else(|_| ".".into());
    let p = format!("{}/grab.ppm", dir);
    goome::save_ppm(&p, &all.pixels, all.w, all.h, all.w).unwrap();
    assert_eq!(&std::fs::read(&p).unwrap()[..2], b"P6");
    let _ = std::fs::remove_file(&p);
    assert!(goome::save_ppm(&p, &all.pixels[..10], all.w, all.h, all.w).is_err());
}

#[test]
fn errors() {
    let e = Options::new().backend(goome::TEST).test_frames("no/such/frames").open().err().unwrap();
    assert_eq!(e.code, goome::ENODISPLAY);
    assert!(!e.to_string().is_empty());
    let e = Options::new().mode(7).open().err().unwrap();
    assert_eq!(e.code, goome::EARG);
    let g = Goome::test("end 3").unwrap();
    assert!(g.control_path().is_none());
    assert!(g.hotkey("alt+", 1).is_err());
}

#[test]
fn frame_end_sends_hits() {
    let mut g = Goome::test("end 3").unwrap();
    let mut c = g.begin(0).unwrap();
    c.rect(0.0, 0.0, 10.0, 10.0, 0xFFFF0000);
    c.hit_rect(goome::Rect::new(0, 0, 10, 10));
    c.end().unwrap();
    assert_eq!(g.alpha_at(5.0, 5.0), 255);
}

#[test]
fn offscreen() {
    let mut g = Options::new()
        .title("engine")
        .mode(goome::MODE_OFFSCREEN)
        .size(200, 100)
        .scale(2.0)
        .open()
        .unwrap();
    assert_eq!(g.backend_name(), "offscreen");
    assert_eq!(g.mode(), goome::MODE_OFFSCREEN);
    let m = &g.monitors()[0];
    assert_eq!((m.area.w, m.area.h, m.scale, m.pw, m.ph, m.name.as_str()), (200, 100, 2.0, 400, 200, "engine"));
    assert_eq!(g.last_frame().err().unwrap().code, goome::EAGAIN);
    {
        let mut c = g.begin(0).unwrap();
        assert_eq!((c.w(), c.h()), (400, 200));
        c.rect(10.0, 10.0, 20.0, 20.0, 0x80FF0000);
        c.hit(10, 10, 20, 20);
    }
    {
        let f = g.last_frame().unwrap();
        assert_eq!((f.w(), f.h(), f.stride()), (400, 200, 400));
        assert_eq!(f.pixels().len(), 400 * 200);
        assert_eq!(f.pixel(25, 25), 0x80800000);
        assert_eq!(f.row(0)[0], 0);
    }
    assert!(g.hit(15.0, 15.0) && !g.hit(30.0, 30.0));
    assert_eq!(g.frame_dirty(), Some(goome::Rect::new(0, 0, 400, 200)));
    let mut rgba = vec![0u8; 4 * 4 * 4];
    g.frame_copy(&mut rgba, 16, goome::FMT_RGBA, Some(goome::Rect::new(24, 24, 4, 4))).unwrap();
    assert_eq!(&rgba[..4], &[255, 0, 0, 128]);
    let mut small = vec![0u8; 8];
    assert!(g.frame_copy(&mut small, 16, goome::FMT_RGBA, Some(goome::Rect::new(0, 0, 4, 4))).is_err());
    assert!(g.frame_copy(&mut rgba, 16, goome::FMT_RGBA, Some(goome::Rect::new(398, 0, 4, 4))).is_err());
    assert!(g.frame_copy(&mut rgba, 16, goome::FMT_RGBA, Some(goome::Rect::new(i32::MAX, 0, 4, 4))).is_err());

    g.push(&Event::down(12.0, 13.0, 1)).unwrap();
    g.push(&Event::key('q' as i32, 0, "q")).unwrap();
    let ev = g.poll(0).unwrap();
    assert_eq!((ev.kind, ev.x, ev.y), (goome::DOWN, 12.0, 13.0));
    let ev = g.poll(0).unwrap();
    assert_eq!((ev.kind, ev.key, ev.text.as_str()), (goome::KEY, 'q' as i32, "q"));
    assert!(g.poll(0).is_none());

    g.resize(300, 150, 0.0).unwrap();
    assert_eq!(g.poll(0).unwrap().kind, goome::MONITORS);
    assert_eq!(g.monitors()[0].pw, 600);
    assert!(g.resize(0, 10, 1.0).is_err());
    g.window_title("renamed").unwrap();
    assert_eq!(g.monitors()[0].name, "renamed");
    assert!(g.window_pos().is_err());
    assert_eq!(g.clipboard_get().unwrap(), "");
    g.clipboard_set("hello clipboard").unwrap();
    assert_eq!(g.clipboard_get().unwrap(), "hello clipboard");
    let long = "x".repeat(10000);
    g.clipboard_set(&long).unwrap();
    assert_eq!(g.clipboard_get().unwrap(), long);
    g.cursor(goome::CURSOR_IBEAM).unwrap();
    assert_eq!(g.cursor_shape(), goome::CURSOR_IBEAM);
    assert!(g.cursor(99).is_err());
    assert!(g.hotkey("alt+h", 1).is_err());
    assert!(g.capture(0, None).is_err());
    while g.poll(0).is_some() {}
    g.push_text("Hé").unwrap();
    let a = g.poll(0).unwrap();
    let b = g.poll(0).unwrap();
    assert_eq!((a.key, a.mods, a.text.as_str(), b.text.as_str()), ('h' as i32, goome::SHIFT, "H", "é"));
}

#[test]
fn frame_matches_c() {
    let mut g = Options::new()
        .title("engine")
        .mode(goome::MODE_OFFSCREEN)
        .size(200, 200)
        .scale(2.0)
        .open()
        .unwrap();
    {
        let mut c = g.begin(0).unwrap();
        c.rect(10.0, 10.0, 20.0, 20.0, 0x80FF0000);
        c.circle(120.0, 50.0, 30.0, 0xFF3A7AFE);
        c.rrect(40.0, 60.0, 70.0, 30.0, 8.0, 0xCC20C060);
        c.text(44.0, 66.0, "offscreen", 0xFFFFFFFF);
        c.hit(10, 10, 20, 20);
    }
    let draw = {
        let f = g.last_frame().unwrap();
        goome_ref::fnv(f.pixels(), f.w(), f.h(), f.stride())
    };
    let mut ui = g.ui().unwrap();
    ui.theme().anim_ms = 0.0;
    ui.set_font(None);
    ui.theme().font_size = 14.0;
    let mut win = goome::WindowState::new(6.0, 6.0, 180.0, 0.0);
    let (mut check, mut val, mut name) = (false, 0.25, String::from("hi"));
    for frame in 0..6 {
        if frame == 1 {
            ui.push(&Event::motion(25.0, 64.0)).unwrap();
            ui.push(&Event::down(25.0, 64.0, 1)).unwrap();
            ui.push(&Event::up(25.0, 64.0, 1)).unwrap();
        }
        if frame == 2 {
            ui.push(&Event::down(90.0, 168.0, 1)).unwrap();
            ui.push(&Event::up(90.0, 168.0, 1)).unwrap();
        }
        if frame == 3 {
            ui.push(&Event::key('z' as i32, 0, "z")).unwrap();
        }
        ui.pump(0);
        let Some(mut f) = ui.begin(0) else { continue };
        let mut w = f.window("engine", &mut win, goome::WIN_MOVE);
        if w.shown() {
            w.checkbox("Check", &mut check);
            w.slider_fmt("Val", &mut val, 0.0, 1.0, Some("%.2f"));
            w.input_cap("Name", &mut name, 32);
        }
    }
    let f = ui.last_frame().unwrap();
    let mine = format!(
        "draw {:08x} ui {:08x} {} {} {}",
        draw,
        goome_ref::fnv(f.pixels(), f.w(), f.h(), f.stride()),
        check as i32,
        name,
        ui.hit(10.0, 10.0) as i32
    );
    assert_eq!(mine, goome_ref::frame());
}

#[test]
fn one_thread_and_one_test_backend() {
    let g = Goome::test("end 3").unwrap();
    assert_eq!(Goome::test("end 3").err().unwrap().code, goome::EAGAIN);
    let other = std::thread::spawn(|| (Goome::offscreen(10, 10, 1.0).is_err(), goome::Canvas::new(4, 4, 4, 1.0).is_err()))
        .join()
        .unwrap();
    assert_eq!(other, (true, true));
    drop(g);
    let again = std::thread::spawn(|| Goome::test("end 3").is_ok()).join().unwrap();
    assert!(again);
    assert!(Goome::test("end 3").is_ok());
}
