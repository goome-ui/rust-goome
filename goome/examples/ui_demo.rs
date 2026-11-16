use goome::{Bitmap, Options, Pacer, Ui, WindowState};

const FRUITS: [&str; 10] = ["Mango", "Banana", "Cherry", "Kiwi", "Apple", "Lemon", "Grape", "Fig", "Dragonfruit", "Pear"];
const TABS: [&str; 4] = ["Widgets", "Images", "Theme", "About"];

struct Demo {
    logo: Bitmap,
    main: WindowState,
    stats: WindowState,
    tab: i32,
    clicks: i32,
    grid: bool,
    wifi: bool,
    notify: bool,
    mode: i32,
    count: i32,
    fruit: i32,
    picked: i32,
    more: bool,
    light: i32,
    own_accent: bool,
    solid: bool,
    always: bool,
    img_clicks: i32,
    volume: f64,
    radius: f64,
    anim: f64,
    highlight: u32,
    accent: u32,
    name: String,
    search: String,
    fps: f64,
    frame_ms: f64,
    running: bool,
}

fn gradient() -> Bitmap {
    let mut px = vec![0u32; 128 * 128];
    for y in 0..128u32 {
        for x in 0..128u32 {
            let r = 0x2A + x * (0x6F - 0x2A) / 127;
            let g = 0x40 + y * (0xDC - 0x40) / 127;
            let b = 0x9A - (x + y) * 0x50 / 254;
            px[(y * 128 + x) as usize] = 0xFF000000 | r << 16 | g << 8 | b;
        }
    }
    Bitmap::from_argb(&px, 128, 128).expect("cant make the gradient")
}

fn plural(n: i32, none: &str, one: &str, many: &str) -> String {
    match n {
        0 => none.to_string(),
        1 => one.to_string(),
        _ => many.replace("{}", &n.to_string()),
    }
}

impl Demo {
    fn set_theme(&self, u: &mut Ui) {
        let t = u.theme();
        if self.light == 1 {
            t.set_light();
        } else {
            t.set_dark();
        }
        if self.own_accent {
            t.accent = self.accent;
        }
        t.radius = self.radius;
        t.anim_ms = self.anim;
        if self.solid {
            t.panel |= 0xFF000000;
        }
    }

    fn widgets(&mut self, u: &mut Ui) {
        u.heading("Buttons");
        u.row(3);
        if u.button("Click me") {
            self.clicks += 1;
        }
        if u.button("Reset") {
            self.clicks = 0;
        }
        u.button("Hover me");
        u.tooltip("hi :)");
        u.label(&plural(self.clicks, "not clicked yet", "clicked once", "clicked {} times"));
        u.separator();
        u.heading("Switches");
        u.checkbox("Show grid", &mut self.grid);
        u.same_line();
        u.checkbox("Notifications", &mut self.notify);
        u.toggle("Wi-Fi", &mut self.wifi);
        for (i, name) in ["Small", "Medium", "Large"].iter().enumerate() {
            if i > 0 {
                u.same_line();
            }
            u.radio(name, &mut self.mode, i as i32);
        }
        u.separator();
        u.heading("Values");
        u.slider_fmt("Volume", &mut self.volume, 0.0, 1.0, Some("%.2f"));
        u.slider_int("Count", &mut self.count, 0, 100);
        u.progress(self.volume, Some(&format!("{}%", (self.volume * 100.0 + 0.5) as i32)));
        u.progress(-1.0, None);
        u.row(2);
        u.input_cap("Name", &mut self.name, 64);
        u.input_cap("Search", &mut self.search, 64);
        u.combo("Fruit", &mut self.fruit, &FRUITS);
        u.color("Highlight", &mut self.highlight);
        u.collapsing("More stuff", &mut self.more);
        if self.more {
            u.wrapped("a long list so theres something to scroll. pick one, it shows up under it.");
            {
                let mut s = u.scroll("list", 150.0);
                for i in 0..30 {
                    if s.selectable(&format!("Item {}", i + 1), self.picked == i) {
                        self.picked = i;
                    }
                }
            }
            if self.picked >= 0 {
                u.label(&format!("picked item {}", self.picked + 1));
            } else {
                u.label("nothing picked");
            }
        }
    }

    fn images(&mut self, u: &mut Ui) {
        u.heading("Generated gradient");
        for (i, r) in [18.0, 60.0, 0.0].iter().enumerate() {
            if i > 0 {
                u.same_line();
            }
            u.image(&self.logo, 120.0, 120.0, *r);
        }
        u.label("image buttons");
        for i in 0..4 {
            if i > 0 {
                u.same_line();
            }
            if u.image_button(&format!("img{}", i), &self.logo, 56.0, 56.0) {
                self.img_clicks += 1;
            }
        }
        u.label(&plural(self.img_clicks, "no image clicks yet", "one image click", "{} image clicks"));
        u.image(&self.logo, 0.0, 90.0, 10.0);
    }

    fn theme(&mut self, u: &mut Ui) {
        u.heading("Theme");
        let mut ch = u.radio("Dark", &mut self.light, 0);
        u.same_line();
        ch |= u.radio("Light", &mut self.light, 1);
        if u.color("Accent", &mut self.accent) {
            self.own_accent = true;
            ch = true;
        }
        ch |= u.slider_fmt("Corner radius", &mut self.radius, 0.0, 20.0, Some("%.0f px"));
        ch |= u.slider_fmt("Animation", &mut self.anim, 0.0, 400.0, Some("%.0f ms"));
        ch |= u.toggle("Solid panels", &mut self.solid);
        if u.button("Reset theme") {
            self.light = 0;
            self.own_accent = false;
            self.radius = 10.0;
            self.anim = 120.0;
            self.solid = false;
            self.accent = 0xFF6FDC5F;
            ch = true;
        }
        if ch {
            self.set_theme(u);
        }
        if !self.own_accent {
            self.accent = u.theme().accent;
        }
    }

    fn about(&mut self, u: &mut Ui) {
        u.heading("About");
        u.wrapped("made with goome. try clicking next to this window, your desktop still gets it.");
        let name = u.backend_name();
        u.label(&format!("goome {} on {}", goome::VERSION, name));
        u.label(&format!("{:.0} fps, {:.2} ms a frame", self.fps, self.frame_ms));
        if !self.stats.open && u.button("Show stats") {
            self.stats.open = true;
        }
    }

    fn draw(&mut self, u: &mut Ui) {
        let flags = goome::WIN_MOVE | goome::WIN_RESIZE | goome::WIN_COLLAPSE;
        let mut main = self.main;
        {
            let mut w = u.window("goome ui", &mut main, flags);
            if w.shown() {
                w.tabs("tabs", &TABS, &mut self.tab);
                let mut id = w.id(TABS[self.tab as usize]);
                let mut page = id.scroll("page", 0.0);
                match self.tab {
                    0 => self.widgets(&mut page),
                    1 => self.images(&mut page),
                    2 => self.theme(&mut page),
                    _ => self.about(&mut page),
                }
            }
        }
        self.main = main;
        let mut stats = self.stats;
        {
            let mut w = u.window("Stats", &mut stats, flags | goome::WIN_CLOSE);
            if w.shown() {
                w.label(&format!("{:.0} fps", self.fps));
                w.label(&format!("{:.2} ms a frame", self.frame_ms));
                w.progress(
                    self.frame_ms / 16.7,
                    Some(&format!("{}% of a 60 fps frame", (self.frame_ms * 100.0 / 16.7 + 0.5) as i32)),
                );
                w.toggle("Redraw every frame", &mut self.always);
                if w.button("Quit") {
                    self.running = false;
                }
            }
        }
        self.stats = stats;
    }
}

fn main() {
    let g = match Options::new().title("goome ui").no_capture(true).open() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("cant open an overlay: {}", e);
            std::process::exit(1);
        }
    };
    let a = g.monitors().first().map(|m| m.area).unwrap_or(goome::Rect::new(0, 0, 1280, 720));
    let mut u = g.ui().expect("cant make the ui");
    let (ax, ay) = (a.x as f64, a.y as f64);
    let mut d = Demo {
        logo: gradient(),
        main: WindowState::new(ax + 40.0, ay + 40.0, 440.0, 620.0),
        stats: WindowState::new(ax + 500.0, ay + 40.0, 280.0, 0.0),
        tab: 0,
        clicks: 0,
        grid: true,
        wifi: true,
        notify: false,
        mode: 1,
        count: 42,
        fruit: 2,
        picked: -1,
        more: true,
        light: 0,
        own_accent: false,
        solid: false,
        always: false,
        img_clicks: 0,
        volume: 0.65,
        radius: 10.0,
        anim: 120.0,
        highlight: 0xFF4FA3FF,
        accent: 0xFF6FDC5F,
        name: "goome".into(),
        search: String::new(),
        fps: 0.0,
        frame_ms: 0.0,
        running: true,
    };
    let test = u.backend_name().starts_with("test");
    let (mut spent, mut frames, mut acc_ms, mut acc_n, mut fps_n) = (0i64, 0i64, 0.0, 0, 0);
    let mut fps_t = goome::now_ms();
    let mut pace = Pacer::new(60);
    let mut dirty = true;
    while d.running {
        let busy = dirty || d.always || test || u.animating();
        for ev in u.pump(if busy { pace.wait() } else { 500 }) {
            if ev.kind == goome::QUIT {
                d.running = false;
            }
            dirty = true;
        }
        if !d.running {
            break;
        }
        if !(dirty || d.always || test || u.animating()) {
            continue;
        }
        dirty = false;
        let t0 = goome::now_ms();
        let Some(mut f) = u.begin(0) else { continue };
        d.draw(&mut f);
        drop(f);
        let t1 = goome::now_ms();
        spent += t1 - t0;
        frames += 1;
        acc_ms += (t1 - t0) as f64;
        acc_n += 1;
        fps_n += 1;
        if t1 - fps_t >= 500 {
            d.fps = fps_n as f64 * 1000.0 / (t1 - fps_t) as f64;
            d.frame_ms = acc_ms / acc_n as f64;
            fps_t = t1;
            fps_n = 0;
            acc_ms = 0.0;
            acc_n = 0;
        }
    }
    if frames > 0 {
        println!("{} frames, {:.2} ms a frame", frames, spent as f64 / frames as f64);
    }
}
