use goome::{Draw, Event, Goome, Skin, Theme, Ui, WindowState};

fn fnv(px: &[u32], w: usize, h: usize, stride: usize) -> u32 {
    goome_ref::fnv(px, w, h, stride)
}

#[test]
fn scripted_widgets_match_c() {
    let script = "screen 400x360 1; 1 click 35,71; 2 click 29,106; 3 hold 40,157 6; 4 hover 120,157; \
                  7 click 100,213; 8 key ab; 9 key \\n; 10 key zz; 11 click 120,273; 13 click 120,206; \
                  15 click 300,20; 16 hold 100,20 20; 18 hover 150,40; end 22";
    let mut ui = Goome::test(script).unwrap().ui().unwrap();
    ui.set_font(None);
    ui.theme().font_size = 14.0;
    let mut win = WindowState::new(10.0, 10.0, 220.0, 0.0);
    let (mut val, mut check, mut pick, mut text) = (0.0, false, 0, String::new());
    let (mut quit, mut frames, mut clicks, mut downs, mut keys_off, mut kb_seen) = (false, 0, 0, 0, 0, false);
    let mut hs = [0u32; 3];
    while !quit && frames < 100 {
        while let Some(ev) = ui.poll(0) {
            match ev.kind {
                goome::QUIT => quit = true,
                goome::DOWN => downs += 1,
                goome::KEY if frames > 10 => keys_off += 1,
                _ => {}
            }
            ui.feed(&ev);
        }
        let mut f = ui.begin(0).expect("a ui frame");
        {
            let mut w = f.window("test", &mut win, goome::WIN_MOVE);
            if w.shown() {
                if w.button("Go") {
                    clicks += 1;
                }
                w.checkbox("Check", &mut check);
                w.slider("Val", &mut val, 0.0, 1.0);
                w.input_cap("Text", &mut text, 16);
                w.combo("Pick", &mut pick, &["one", "two", "three"]);
            }
        }
        drop(f);
        let last = ui.last_frame().unwrap();
        let (w, h, stride) = (last.w(), last.h(), last.stride());
        let px = last.pixels().to_vec();
        if frames == 8 {
            kb_seen = !text.is_empty();
        }
        if frames == 0 || frames == 12 || frames == 21 {
            hs[if frames == 0 {
                0
            } else if frames == 12 {
                1
            } else {
                2
            }] = fnv(&px, w, h, stride);
        }
        frames += 1;
    }
    assert_eq!((clicks, check), (1, true));
    assert!(val > 0.45 && val < 0.55, "{}", val);
    assert!(kb_seen && text == "ab" && keys_off == 0, "{:?}", text);
    assert_eq!(pick, 1);
    assert_eq!((win.x, win.y), (60.0, 30.0));
    assert_eq!(downs, 7);
    assert_eq!(hs, [0x51713294, 0xb52f16c5, 0xea01692c]);
}

#[test]
fn style_custom_icons_match_c() {
    let mut ui = goome::Options::new()
        .title("style")
        .mode(goome::MODE_OFFSCREEN)
        .size(320, 260)
        .scale(1.5)
        .open()
        .unwrap()
        .ui()
        .unwrap();
    ui.theme().set_preset(goome::PRESET_MINT).unwrap();
    ui.set_font(None);
    ui.theme().font_size = 14.0;
    ui.theme().anim_ms = 0.0;
    let argb: Vec<u32> = (0..64).map(|i| if i % 9 != 0 { 0xFF20A0E0 } else { 0x80FFFFFF }).collect();
    let bm = goome::Bitmap::from_argb(&argb, 8, 8).unwrap();
    let mut win = WindowState::new(10.0, 10.0, 280.0, 0.0);
    let (mut drag, mut on, mut sel) = (0.0, true, 1);
    let plot = [1.0f32, 3.0, 2.0, 5.0, 4.0, 6.0];
    for frame in 0..4 {
        if frame == 1 {
            ui.push(&Event::motion(60.0, 185.0)).unwrap();
            ui.push(&Event::down(60.0, 185.0, 1)).unwrap();
        }
        if frame == 2 {
            ui.push(&Event::motion(90.0, 185.0)).unwrap();
        }
        if frame == 3 {
            ui.push(&Event::up(90.0, 185.0, 1)).unwrap();
        }
        ui.pump(0);
        let Some(mut f) = ui.begin(0) else { continue };
        let mut w = f.window("style", &mut win, goome::WIN_MOVE);
        if w.shown() {
            {
                let mut s = w.style();
                s.color(goome::S_ACCENT, 0xFFFF6B8B).num(goome::S_RADIUS, 2.0);
                s.button("Styled");
            }
            {
                let mut s = w.style();
                s.icon(goome::ICON_CHECK, Some(&bm));
                s.checkbox("Icon", &mut on);
            }
            w.tabs("tabs", &["One", "Two"], &mut sel);
            let it = w.custom("knob", 200.0, 30.0, goome::ITEM_DRAG_X);
            drag += it.dx;

            let mut c = w.canvas().unwrap();
            c.rrect(it.x, it.y, it.w, it.h, 6.0, if it.held { 0xFF6FDC5F } else { 0xFF404858 });
            c.circle(it.x + 15.0 + drag, it.y + 15.0, 10.0, 0xFFFFFFFF);
            w.plot_bars("bars", &plot, 0.0, 6.0, 40.0);
            w.badge("new", 0xFFE04040);
            w.progress(0.4, None);
        }
    }
    let f = ui.last_frame().unwrap();
    let mine = fnv(f.pixels(), f.w(), f.h(), f.stride()) ^ (drag as i32 as u32);
    assert!(drag > 20.0, "{}", drag);
    assert_eq!(mine, goome_ref::style());
}

fn everything(ui: &mut Ui, st: &mut State) {
    let mut f = match ui.begin(0) {
        Some(f) => f,
        None => return,
    };
    let mut a = f.app(Some("everything"), goome::APP_MENUBAR | goome::APP_STATUSBAR);
    if let Some(mut bar) = a.menubar() {
        if let Some(mut m) = bar.menu("File") {
            if m.menu_item("Quit", Some("ctrl+q"), None) {
                st.quit = true;
            }
            m.menu_separator();
            m.menu_item("Grid", None, Some(&mut st.grid));
        }
    }
    {
        let mut p = a.split("main", false, &mut st.split);
        p.label("left");
        p.heading("heading");
        p.wrapped("some long wrapped text that goes on and on");
        p.link("a link");
        p.spinner(16.0);
        p.radio("A", &mut st.radio, 0);
        p.radio("B", &mut st.radio, 1);
        p.toggle("Toggle", &mut st.toggle);
        p.slider_int("Int", &mut st.int, 0, 10);
        p.drag_float_fmt("Drag", &mut st.drag, 0.1, 0.0, 10.0, Some("%.1f"));
        p.drag_int("DragI", &mut st.int, 1.0, 0, 10);
        p.number("Num", &mut st.num, 0.5);
        p.color("Color", &mut st.color);
        p.selectable("Pick me", st.toggle);
        p.collapsing("More", &mut st.more);
        p.input_ex("Secret", &mut st.secret, goome::INPUT_PASSWORD, Some("hint"));
        p.plot_lines("lines", &[1.0, 2.0, 1.5], 0.0, 3.0, 30.0);
        if let Some(mut t) = p.tree("Tree", &mut st.tree) {
            t.label("leaf");
        }
        p.split_next();
        if let Some(mut t) = p.table("files", 2, 120.0, goome::TABLE_SORTABLE | goome::TABLE_STRIPES) {
            t.table_column("Name", 0.0);
            t.table_column("Size", 60.0);
            t.table_sort(&mut st.col, &mut st.desc);
            for i in 0..3 {
                t.table_row();
                t.label(&format!("file {}", i));
                t.table_next();
                t.label(&format!("{}", i * 10));
            }
        }
        {
            let mut s = p.scroll("log", 80.0);
            for i in s.list_clip(50, 0.0) {
                let mut id = s.id(&i.to_string());
                id.label("row");
            }
        }
        p.text_edit("body", &mut st.body, 80.0, goome::EDIT_LINE_NUMBERS);
        p.button("drag me");
        p.drag_source("thing", b"hello");
        p.button("drop here");
        if let Some(got) = p.drop_target("thing", 64) {
            st.dropped = got;
        }
        if p.button("open") {
            p.popup_open("pop");
        }
        if let Some(mut pop) = p.popup("pop") {
            pop.label("in a popup");
            pop.popup_close();
        }
        if let Some(mut c) = p.context("ctx") {
            c.label("ctx");
        }
        if let Some(mut m) = p.modal("Sure?", &mut st.modal) {
            m.label("yes");
        }
        assert!(p.store("count", (1 << 24) + 1).is_empty());
        let store = p.store("count", 2);
        store[0] += 1.0;
        st.stored = store[0];
        p.pos(4.0, 4.0);
        p.size(30.0, 30.0);
        p.width(0.0);
        p.same_line();
        p.space(2.0);
        p.separator();
        p.row(2);
        p.tooltip("tip");
        let _ = p.hovered() || p.focused();
        p.set_focus();
        p.add_hit(0.0, 0.0, 1.0, 1.0);
    }
    a.statusbar("ready");
    a.toast("hi", 100);
}

#[derive(Default)]
struct State {
    quit: bool,
    grid: bool,
    split: f64,
    radio: i32,
    toggle: bool,
    int: i32,
    drag: f64,
    num: f64,
    color: u32,
    more: bool,
    secret: String,
    tree: bool,
    col: i32,
    desc: bool,
    body: String,
    dropped: Vec<u8>,
    modal: bool,
    stored: f64,
}

#[test]
fn every_widget_runs() {
    let mut ui = Goome::offscreen(900, 700, 1.0).unwrap().ui().unwrap();
    let mut st = State {
        split: 0.4,
        tree: true,
        more: true,
        modal: true,
        body: "line one\nline two".into(),
        color: 0xFF336699,
        ..State::default()
    };
    ui.bind(goome::ACT_COPY, "ctrl+c,ctrl+insert").unwrap();
    assert!(ui.bind(goome::ACT_COPY, "ctrl+").is_err());
    ui.config().blink_ms = 300;
    ui.set_mask(Some("*"));
    ui.auto_grab(goome::WANT_MOUSE);
    ui.scale(1.0);
    for _ in 0..5 {
        ui.pump(0);
        everything(&mut ui, &mut st);
    }
    assert_eq!(st.stored, 5.0);
    assert!(ui.last_frame().unwrap().pixels().iter().any(|&p| p != 0));
    let _ = ui.wants();
    let _ = ui.animating();
}

#[test]
fn themes_and_skins() {
    let mut ui = Goome::offscreen(300, 200, 1.0).unwrap().ui().unwrap();
    let dark = Theme::dark();
    let light = Theme::light();
    assert_ne!(dark.bg, light.bg);
    let made = Theme::make(0xFF336699, true, 7.0);
    assert_eq!((made.accent, made.radius), (0xFF336699, 7.0));
    assert!(Theme::preset(999).is_err());
    let mut t = *ui.theme();
    t.set_light();
    ui.set_theme(&t);
    assert_eq!(ui.theme().bg, light.bg);
    let bm = goome::Bitmap::from_argb(&[0xFFFFFFFF; 16], 4, 4).unwrap();
    let skin = Skin::even(&bm, 1.0);
    ui.set_skin(goome::SKIN_BUTTON, Some(&skin));
    ui.set_icon(goome::ICON_CLOSE, Some(&bm));
    drop(bm);
    drop(skin);
    for _ in 0..2 {
        let Some(mut f) = ui.begin(0) else { continue };
        let mut w = f.window("w", &mut WindowState::new(5.0, 5.0, 200.0, 0.0), goome::WIN_CLOSE);
        let mut s = w.style();
        s.theme(&dark).radii((1.0, 2.0, 3.0, 4.0)).font(None).skin(goome::SKIN_PANEL, None);
        s.button("skinned");
        s.push_color(goome::S_TEXT, 0xFFFF0000);
        s.label("red");
        s.pop(1);
    }
    ui.set_skin(goome::SKIN_BUTTON, None);
    ui.set_icon(goome::ICON_CLOSE, None);
}

#[test]
fn window_close_and_drop_order() {
    let mut ui = Goome::test("screen 400x300 1; end 4").unwrap().ui().unwrap();
    let mut win = WindowState::new(10.0, 10.0, 200.0, 120.0);
    win.open = false;
    let shown = {
        let mut f = ui.begin(0).unwrap();
        let w = f.window("hidden", &mut win, goome::WIN_CLOSE);
        w.shown()
    };
    assert!(!shown);
    let font = goome::Font::from_bytes(vec![0; 4]);
    assert!(font.is_err());
    let g = ui.goome();
    assert!(g.monitors().len() == 1);
    drop(ui);
}

#[test]
fn bad_formats_are_ignored() {
    let mut ui = Goome::offscreen(300, 200, 1.0).unwrap().ui().unwrap();
    let mut v = 0.5;
    let mut f = ui.begin(0).unwrap();
    let mut w = f.window("fmt", &mut WindowState::new(0.0, 0.0, 200.0, 0.0), 0);
    w.slider_fmt("a", &mut v, 0.0, 1.0, Some("%s %n"));
    w.slider_fmt("b", &mut v, 0.0, 1.0, Some("%d"));
    w.slider_fmt("c", &mut v, 0.0, 1.0, Some("%5.1f%%"));
    w.label("100% safe %s %n");
}

#[test]
fn app_loop_on_the_test_backend() {
    let g = goome::Options::new()
        .backend(goome::TEST)
        .mode(goome::MODE_WINDOW)
        .test_script("screen 1x1 1; 3 resize 700x500; 5 close; end 10")
        .open()
        .unwrap();
    let mut app = goome::App::from_goome(g, "app").unwrap();
    app.flags = goome::APP_STATUSBAR;
    let mut clicks = 0;
    while let Some(mut ui) = app.frame() {
        assert!(ui.shown());
        if ui.button("hi") {
            clicks += 1;
        }
        ui.statusbar("ready");
    }
    assert!(app.frames >= 3, "{}", app.frames);
    assert_eq!(clicks, 0);
    assert!(app.close_asked());
}
