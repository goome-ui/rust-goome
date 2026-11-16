use goome::{Draw, Options, Ui};

#[derive(Default)]
struct Scene {
    on: bool,
    dropped: bool,
    pos: f64,
    spring: f64,
    last: f64,
    col: u32,
    hash: u32,
}

fn open() -> Ui {
    Options::new()
        .backend(goome::TEST)
        .mode(goome::MODE_WINDOW)
        .size(300, 200)
        .test_script("end 100000")
        .open()
        .unwrap()
        .ui()
        .unwrap()
}

fn frame(ui: &mut Ui, k: &mut Scene) {
    ui.pump(0);
    let Some(mut f) = ui.begin(0) else { return };
    k.pos = f.anim("box", if k.on { 200.0 } else { 10.0 }, 160.0, goome::EASE_OUT);
    k.spring = f.anim("spring", if k.on { 1.0 } else { 0.0 }, 320.0, goome::EASE_SPRING);
    k.col = f.anim_color("col", if k.on { 0xFF20D060 } else { 0 }, 200.0, goome::EASE_LINEAR);
    if !k.dropped {
        k.last = f.anim("gone", if k.on { 5.0 } else { 0.0 }, 100.0, goome::EASE_LINEAR);
    }
    let (pos, spring, col) = (k.pos, k.spring, k.col);
    if let Some(mut c) = f.canvas() {
        c.rect(pos, 20.0, 60.0, 40.0, 0xFFE0A030);
        c.rrect(20.0 + spring * 180.0, 90.0, 50.0, 30.0, 15.0, 0xFF4FB3F7);
        c.rect(20.0, 140.0, 260.0, 40.0, col);
    }
    drop(f);
    let last = ui.last_frame().unwrap();
    k.hash = goome_ref::fnv(last.pixels(), last.w(), last.h(), last.stride());
}

#[test]
fn ease_curves() {
    assert_eq!(goome::ease(goome::EASE_LINEAR, 0.25), 0.25);
    assert_eq!(goome::ease(goome::EASE_IN, 0.5), 0.125);
    assert_eq!(goome::ease(goome::EASE_OUT, 0.5), 0.875);
    assert_eq!(goome::ease(goome::EASE_IN_OUT, 0.25), 0.0625);
    assert_eq!(goome::ease(goome::EASE_IN_OUT, 0.75), 0.9375);
    assert_eq!(goome::ease(goome::EASE_SMOOTH, 0.5), 0.5);
    assert_eq!(goome::ease(goome::EASE_BOUNCE, 0.25), 0.47265625);
    assert_eq!(goome::ease(goome::EASE_OUT, -3.0), 0.0);
    assert_eq!(goome::ease(goome::EASE_IN, 7.0), 1.0);
    let peak = (1..100).map(|i| goome::ease(goome::EASE_SPRING, i as f64 / 100.0)).fold(0.0, f64::max);
    assert!(peak > 1.1 && peak < 1.25, "{}", peak);
}

#[test]
fn anim_frames_match_c() {
    let (want, done) = goome_ref::anim();
    assert!(done);
    assert_eq!(want, [0x0226f2f5, 0x906d3ce5, 0xdb5848ad, 0x12bea72d]);
    let mut ui = open();
    let mut k = Scene::default();
    frame(&mut ui, &mut k);
    assert_eq!((k.pos, k.spring, k.col, k.last), (10.0, 0.0, 0, 0.0));
    frame(&mut ui, &mut k);
    assert!(!ui.animating());
    k.on = true;
    let mut hs = [0u32; 4];
    let (mut prev, mut over, mut moving) = (k.pos, 0.0f64, 0);
    for i in 0..60 {
        frame(&mut ui, &mut k);
        match i {
            1 => hs[0] = k.hash,
            4 => hs[1] = k.hash,
            8 => hs[2] = k.hash,
            _ => {}
        }
        assert!(k.pos >= prev);
        prev = k.pos;
        over = over.max(k.spring);
        if ui.animating() {
            moving = i + 1;
        }
    }
    hs[3] = k.hash;
    assert_eq!((k.pos, k.spring, k.col, k.last), (200.0, 1.0, 0xFF20D060, 5.0));
    assert!(over > 1.05 && (19..=22).contains(&moving), "{} {}", over, moving);
    assert_eq!(hs, want);
    k.on = false;
    frame(&mut ui, &mut k);
    frame(&mut ui, &mut k);
    assert!(k.pos < 200.0 && k.pos > 10.0);
    k.dropped = true;
    for _ in 0..1600 {
        frame(&mut ui, &mut k);
    }
    k.dropped = false;
    frame(&mut ui, &mut k);
    assert_eq!(k.last, 0.0);
}
