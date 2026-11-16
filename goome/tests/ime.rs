use goome::{Event, Options, Rect, Ui};

struct Scene {
    ui: Ui,
    input: String,
    edit: String,
    flags: i32,
    ctl: String,
    hash: u32,
}

impl Scene {
    fn open(script: &str, input: &str, edit: &str, focus: i32) -> Scene {
        let mut ui = Options::new()
            .backend(goome::TEST)
            .mode(goome::MODE_WINDOW)
            .size(300, 200)
            .test_script(script)
            .open()
            .unwrap()
            .ui()
            .unwrap();
        ui.set_font(None);
        ui.theme().font_size = 14.0;
        ui.theme().anim_ms = 0.0;
        ui.config().blink_ms = 0;
        let mut k = Scene {
            ui,
            input: input.to_string(),
            edit: edit.to_string(),
            flags: 0,
            ctl: String::new(),
            hash: 0,
        };
        k.frame(focus);
        k.frame(0);
        k
    }

    fn frame(&mut self, focus: i32) {
        for e in self.ui.pump(0) {
            if e.kind == goome::CONTROL && e.msg.starts_with("ime ") {
                self.ctl = e.msg.clone();
            }
        }
        let Some(mut f) = self.ui.begin(0) else { return };
        if focus == 1 {
            f.set_focus();
        }
        f.input_ex_cap("in", &mut self.input, goome::INPUT_NO_SELECT_ON_FOCUS | self.flags, None, 64);
        if focus == 2 {
            f.set_focus();
        }
        f.text_edit("ed", &mut self.edit, 80.0, 0);
        drop(f);
        let last = self.ui.last_frame().unwrap();
        self.hash = goome_ref::fnv(last.pixels(), last.w(), last.h(), last.stride());
    }

    fn push(&mut self, e: Event) {
        self.ui.push(&e).unwrap();
        self.frame(0);
    }

    fn key(&mut self, key: char, mods: i32) {
        let text = if mods & goome::CTRL == 0 { key.to_string() } else { String::new() };
        self.push(Event::key(key as i32, mods, &text));
    }
}

#[test]
fn ime_events_on_the_test_backend() {
    let long = "q".repeat(80);
    let script = format!("1 text {};2 preedit \u{306b}\u{307b}\u{3093} 3;3 preedit abc;4 preedit;end 10", long);
    let g = Options::new()
        .backend(goome::TEST)
        .mode(goome::MODE_WINDOW)
        .size(100, 100)
        .test_script(&script)
        .open()
        .unwrap();
    g.push(&Event::text("hi")).unwrap();
    let e = g.poll(0).unwrap();
    assert_eq!((e.kind, e.msg.as_str(), g.event_text().as_str()), (goome::TEXT, "hi", "hi"));
    g.text_input(true, Some(Rect::new(10, 20, 1, 16))).unwrap();
    g.text_input(false, None).unwrap();
    let mut g = g;
    let (mut got, mut quit) = (0, false);
    for _ in 0..20 {
        if quit {
            break;
        }
        while let Some(e) = g.poll(0) {
            match e.kind {
                goome::QUIT => quit = true,
                goome::TEXT => {
                    got |= 1;
                    assert_eq!(e.msg, long);
                }
                goome::PREEDIT if e.msg == "\u{306b}\u{307b}\u{3093}" => {
                    got |= 2;
                    assert_eq!((e.id, g.preedit_cursor()), (3, 3));
                }
                goome::PREEDIT if e.msg == "abc" => {
                    got |= 4;
                    assert_eq!(g.preedit_cursor(), 3);
                }
                goome::PREEDIT if e.msg.is_empty() => {
                    got |= 8;
                    assert_eq!(g.preedit_cursor(), 0);
                }
                _ => {}
            }
        }
        if let Some(f) = g.begin(0) {
            f.end().unwrap();
        }
    }
    assert_eq!(got, 15);
}

#[test]
fn preedit_render_and_commit_match_c() {
    let (want, c_in) = goome_ref::ime();
    assert_eq!((want[1], want[2], want[4]), (0x2e20540f, 0xceafcbad, 0xc9c1add5));
    assert_eq!(c_in, "ab\u{65e5}\u{672c}");
    let mut k = Scene::open("end 100000", "ab", "", 1);
    let mut h = [0u32; 5];
    h[0] = k.hash;
    k.push(Event::preedit("xyz", 1));
    h[1] = k.hash;
    k.push(Event::preedit("xyz", 3));
    h[2] = k.hash;
    assert_eq!(k.input, "ab");
    k.push(Event::preedit("", 0));
    assert_eq!(k.hash, h[0]);
    k.push(Event::text("\u{65e5}\u{672c}"));
    assert_eq!(k.input, c_in);
    k.key('c', 0);
    assert_eq!(k.input, "ab\u{65e5}\u{672c}c");
    k.key('z', goome::CTRL);
    assert_eq!(k.input, "ab\u{65e5}\u{672c}");
    drop(k);

    let mut k = Scene::open("end 100000", "", "line", 2);
    h[3] = k.hash;
    k.push(Event::preedit("kana", 2));
    h[4] = k.hash;
    k.push(Event::preedit("", 0));
    k.push(Event::text("\u{304b}\u{306a}\n"));
    assert_eq!((k.edit.as_str(), k.input.as_str()), ("\u{304b}\u{306a}\nline", ""));
    drop(k);
    assert_eq!(h, want);

    let mut k = Scene::open("end 100000", "", "", 1);
    k.flags = goome::INPUT_NUMBER;
    k.push(Event::text("1a2.5"));
    assert_eq!(k.input, "12.5");
}

#[test]
fn long_commit_and_caret_control() {
    let long = "w".repeat(90);
    let mut k = Scene::open(&format!("4 text {};6 ime;end 100000", long), "", "", 1);
    for _ in 0..8 {
        k.frame(0);
    }
    assert_eq!(k.input, long[..63]);
    let p: Vec<&str> = k.ctl.split(|c| c == ' ' || c == ',' || c == 'x').collect();
    assert_eq!(p[..2], ["ime", "1"], "{}", k.ctl);
    let n: Vec<i32> = p[2..].iter().map(|s| s.parse().unwrap()).collect();
    assert!(n[0] > 0 && n[1] > 0 && n[3] >= 14, "{}", k.ctl);
}
