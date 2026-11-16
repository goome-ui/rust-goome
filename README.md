<p align="center">
  <img src="https://raw.githubusercontent.com/goome-ui/.github/main/profile/logo.png" alt="goome" width="150">
</p>

<h1 align="center">🦀 rust-goome 🦀</h1>

<p align="center">
<b>Overlays on top of everything, normal desktop apps, or ui inside your game, from safe Rust.</b>
</p>

<p align="center">
  <img src="https://goome-ui.github.io/docs/assets/badges/rust-177.svg" alt="Rust 1.77+">
  <img src="https://goome-ui.github.io/docs/assets/badges/crate-goome-sys.svg" alt="crate goome + goome-sys">
  <img src="https://img.shields.io/badge/Linux-X11%20%2B%20Wayland-1b1d22?style=flat-square&logo=linux&logoColor=white" alt="Linux">
  <img src="https://img.shields.io/badge/Windows-10%20%2F%2011-1b1d22?style=flat-square&logo=windows11&logoColor=white" alt="Windows">
  <img src="https://img.shields.io/badge/macOS-14%2B-1b1d22?style=flat-square&logo=apple&logoColor=white" alt="macOS">
  <img src="https://goome-ui.github.io/docs/assets/badges/license-gpl-3.svg" alt="GPL-3.0">
</p>

<p align="center">
  <a href="https://goome-ui.github.io/docs/">Docs</a> &nbsp;·&nbsp;
  <a href="https://goome-ui.github.io/docs/builder/">Builder</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/examples">Examples</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/c-goome">C</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/csharp-goome">C#</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/python-goome">Python</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/java-goome">Java</a> &nbsp;·&nbsp;
  <a href="https://github.com/goome-ui/godot-goome">Godot</a>
</p>

<p align="center">
  This is <a href="https://github.com/goome-ui/c-goome">goome</a> for Rust. Draw over the whole screen and clicks only stop where you drew something.<br>
  It does app windows and offscreen frames for game engines too. <code>goome-sys</code> builds the C library from the vendored goome.h with the <code>cc</code> crate, <code>goome</code> wraps it so you never touch a raw pointer.<br>
  Questions go in <a href="https://github.com/goome-ui/c-goome/discussions">Discussions</a>. If you want to help, the <a href="https://github.com/goome-ui/rust-goome/labels/good%20first%20issue">good first issues</a> are small ones to start with.<br> <br>
  Do you like this project? If so, please let me know!
  <a href="https://sevvy-wevvy.com/discord">Discord</a>
</p>

<h3 align="center">🫥 An overlay on top of everything 🫥</h3>

```rust
use goome::{Draw, Options, QUIT};

let mut g = Options::new().title("my overlay").open()?;
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

<h3 align="center">🪟 A normal app 🪟</h3>

```rust
let mut app = goome::App::new("My app", 960, 640)?;
let mut name = String::new();
while let Some(mut ui) = app.frame() {
    ui.input("Name", &mut name);
    if ui.button("Say hi") {
        ui.toast(&format!("hi {}", name), 2000);
    }
}
```

<h3 align="center">🎮 Inside a game engine 🎮</h3>

```rust
let mut ui = goome::Goome::offscreen(800, 600, 1.0)?.ui()?;
let mut win = goome::WindowState::new(20.0, 20.0, 260.0, 0.0);
let mut volume = 0.5;
loop {
    ui.push(&goome::Event::motion(mouse_x, mouse_y))?;   // your engines input goes in
    ui.pump(0);
    if let Some(mut f) = ui.begin(0) {
        let mut w = f.window("Settings", &mut win, goome::WIN_MOVE);
        if w.shown() {
            w.slider("Volume", &mut volume, 0.0, 1.0);
        }
    }
    let frame = ui.last_frame()?;                       // premultiplied BGRA pixels come out, no copy
    upload_texture(frame.pixels(), frame.w(), frame.h());
}
```

<p align="center">
Overlay is still the default, nothing changes unless you ask for an app or offscreen.<br>
For mods inside a game the other bindings have more. <a href="https://github.com/goome-ui/c-goome">c-goome</a> has the DirectX 9/11/12 and Vulkan backends plus Metal and GL, <a href="https://github.com/goome-ui/csharp-goome">csharp-goome</a> has Unity mod templates, <a href="https://github.com/goome-ui/java-goome">java-goome</a> does Fabric and NeoForge and <a href="https://github.com/goome-ui/godot-goome">godot-goome</a> is a GDExtension.
</p>

<details>
<summary align="center"><b>⬇️ Install ⬇️</b></summary>

<br>

<p align="center">
It isnt on crates.io yet, so take it straight from git:
</p>

```toml
[dependencies]
goome = { git = "https://github.com/goome-ui/rust-goome" }
```

<p align="center">
The first build compiles goome.h with your C compiler, which takes a few seconds once. Cargo already needs one for a lot of crates so you probably have it. If not, on Debian/Ubuntu its <code>sudo apt install build-essential</code>, on Mac <code>xcode-select --install</code> and on Windows the Visual Studio build tools (the same ones rustup asks for).<br> <br>
No system libraries get linked, goome loads what it needs (X11, Wayland, dbus, the Windows and Mac ones) at runtime.
</p>

<h3 align="center">📦 Other Versions</h3>

<p align="center">
Every release gets a tag, pin one like this:
</p>

```toml
goome = { git = "https://github.com/goome-ui/rust-goome", tag = "v0.5.0" }
```

<p align="center">
Working from a clone instead:
</p>

```sh
git clone https://github.com/goome-ui/rust-goome
cd rust-goome
cargo test
cargo run --example ui_demo
```
</details>

<details>
<summary align="center"><b>🎛️ UI Kit 🎛️</b></summary>

<br>

<p align="center">
The immediate mode ui works like in C. You keep your own variables, call the widgets every frame and they change them through <code>&amp;mut</code>. Every widget returns <code>true</code> when it changed something this frame, like the int the C ones return.
</p>

```rust
use goome::{Bitmap, Options, WindowState};

let mut ui = Options::new().title("settings").no_capture(true).open()?.ui()?;
let logo = Bitmap::load("logo.png")?;
let mut win = WindowState::new(40.0, 40.0, 300.0, 0.0);   // h 0 = fit the content
let (mut volume, mut muted, mut name) = (0.5, false, String::from("goome"));
let mut running = true;
while running {
    for ev in ui.pump(16) {
        if ev.kind == goome::QUIT {
            running = false;
        }
    }
    let Some(mut f) = ui.begin(0) else { continue };
    let mut w = f.window("Settings", &mut win, goome::WIN_MOVE | goome::WIN_COLLAPSE);
    if w.shown() {
        w.image(&logo, 64.0, 64.0, 12.0);
        if w.button("Say hi") {
            println!("hi");
        }
        w.slider("Volume", &mut volume, 0.0, 1.0);
        w.toggle("Muted", &mut muted);
        w.input("Name", &mut name);
    }
}
```

<p align="center">
<code>ui.begin()</code> and <code>ui.window()</code> hand back guards. When they drop, <code>goome_ui_end</code> / <code>goome_ui_window_end</code> get called for you, so you cant forget one. The guards deref to the <code>Ui</code>, so you just keep calling widgets on them.<br> <br>
Things C only wants ended when they opened (<code>menubar</code>, <code>menu</code>, <code>context</code>, <code>popup</code>, <code>modal</code>, <code>tree</code>, <code>table</code>) return an <code>Option</code>, so <code>if let Some(mut m) = ui.menu("File")</code> does the right thing. <code>scroll</code>, <code>split</code>, <code>app</code> and <code>id</code> always end, like <code>window</code>.<br>
<code>Ui</code> owns the <code>Goome</code> and derefs to it, so <code>ui.poll()</code>, <code>ui.monitors()</code> and the rest still work.
</p>

```rust
if let Some(mut t) = ui.table("files", 3, 200.0, goome::TABLE_SORTABLE | goome::TABLE_STRIPES) {
    t.table_column("Name", 0.0);
    t.table_column("Size", 80.0);
    t.table_column("Kind", 0.0);
    t.table_sort(&mut col, &mut desc);
    for f in &files {
        t.table_row();
        t.label(&f.name);
        t.table_next();
        t.label(&f.size.to_string());
        t.table_next();
        t.label(&f.kind);
    }
}

{
    let mut panes = ui.split("main", false, &mut side);
    panes.label("left");
    panes.split_next();
    panes.text_edit("body", &mut text, 0.0, goome::EDIT_LINE_NUMBERS | goome::EDIT_WRAP);
}

let mut log = ui.scroll("log", 200.0);
for i in log.list_clip(lines.len(), 0.0) {
    log.label(&lines[i]);
}
```
</details>

<details>
<summary align="center"><b>🎨 Your Own Look 🎨</b></summary>

<br>

<p align="center">
<code>ui.theme()</code> gives you the theme as a <code>&amp;mut Theme</code> with every color and size, plus presets (<code>set_preset(goome::PRESET_MINT)</code>), <code>set_light()</code> and <code>Theme::make(accent, dark, radius)</code>. Fonts, skins and icons go in through <code>ui.set_font</code>, <code>ui.set_skin</code> and <code>ui.set_icon</code> so the ui keeps them alive.<br> <br>
For one widget, <code>ui.style()</code> pushes onto the style stack and pops it all again when it drops:
</p>

```rust
{
    let mut s = ui.style();
    s.color(goome::S_ACCENT, 0xFFFF6B8B).num(goome::S_RADIUS, 2.0).icon(goome::ICON_CHECK, Some(&my_check));
    s.button("Styled");
}

let it = ui.custom("knob", 200.0, 30.0, goome::ITEM_DRAG_X);
knob += it.dx;
if let Some(mut c) = ui.canvas() {
    c.rrect(it.x, it.y, it.w, it.h, 6.0, if it.held { 0xFF6FDC5F } else { 0xFF404858 });
}
```

<p align="center">
<code>ui.config()</code> has the timings and cursors, <code>ui.bind(goome::ACT_COPY, "ctrl+c,ctrl+insert")</code> changes the keys and <code>ui.store(id, n)</code> keeps a few numbers per widget between frames.
</p>
</details>

<details>
<summary align="center"><b>🪟 App Mode 🪟</b></summary>

<br>

<p align="center">
<code>App::new(title, w, h)</code> opens a normal resizable window and <code>app.frame()</code> runs the loop for you without any closures. It waits for input when nothing moves, feeds the ui, and gives you <code>None</code> once the window closes. <code>APP_MENUBAR</code>, <code>APP_STATUSBAR</code> and <code>APP_NO_PAD</code> go in <code>app.flags</code>, the events of that frame are in <code>ui.events()</code> and <code>app.keep_open_on_close(true)</code> lets you ask before closing.<br> <br>
Want your own loop? <code>Goome::app(title, w, h)</code> is the plain window, then <code>ui.app(title, flags)</code> inside a frame.
</p>
</details>

<details>
<summary align="center"><b>🎮 Engines And Offscreen 🎮</b></summary>

<br>

<p align="center">
<code>Goome::offscreen(w, h, scale)</code> never opens a window. You push your engines input in, goome draws into a buffer it owns, and you upload that. It never blocks and never starts a thread.
</p>

| | |
|:--|:--|
| `g.push(&Event::down(x, y, 1))` | feed it input, any mode. `push_text("hi")` types a string |
| `g.last_frame()` | the frame from the last end, `.pixels()` is a `&[u32]` straight on goomes memory. The borrow checker stops you from calling `begin` while you hold it |
| `g.frame_dirty()` | the `Rect` (real px) that changed since you last asked, `None` if nothing did |
| `g.frame_copy(&mut buf, stride, fmt, rect)` | copies that part into your buffer as `FMT_BGRA_PREMUL`, `FMT_RGBA_PREMUL`, `FMT_BGRA` or `FMT_RGBA` |
| `g.resize(w, h, scale)` | new size, sends `MONITORS` |
| `g.hit(x, y)` | true when that spot is on the ui, send clicks to goome only then |
| `g.cursor_shape()` | what cursor the ui wants |

<p align="center">
<code>examples/offscreen.rs</code> writes a frame to a PNG without any image crate.
</p>
</details>

<details>
<summary align="center"><b>🎞️ Animations 🎞️</b></summary>

<br>

<p align="center">
<code>ui.anim(id, target, ms, ease)</code> gives you a number that slides toward <code>target</code>. Call it every frame with whatever the target is right now, goome keeps the state per id and keeps redrawing until its there (<code>ui.animating()</code>). <code>ui.anim_color</code> does the same for colors.<br>
The eases are <code>EASE_SMOOTH</code>, <code>EASE_LINEAR</code>, <code>EASE_IN</code>, <code>EASE_OUT</code>, <code>EASE_IN_OUT</code>, <code>EASE_BACK</code>, <code>EASE_SPRING</code> and <code>EASE_BOUNCE</code>, and <code>goome::ease(kind, t)</code> gives you the raw curve.
</p>

```rust
let x = ui.anim("drawer", if open { 1.0 } else { 0.0 }, 300.0, goome::EASE_SPRING);
let tint = ui.anim_color("drawer tint", if open { 0xFF6FDC5F } else { 0xFF404858 }, 200.0, goome::EASE_LINEAR);
```
</details>

<details>
<summary align="center"><b>🌍 Emoji, RTL, IME And Screen Readers 🌍</b></summary>

<br>

<p align="center">
<code>Font::system(None)</code> already pulls in the system emoji font. For your own fonts, <code>font.fallback(&amp;other)</code> adds one to try for missing chars. The font keeps a clone of it, so dropping yours is fine.<br>
Hebrew and Arabic just work. <code>goome::text_dir(goome::DIR_RTL)</code> forces the direction for all text, <code>DIR_AUTO</code> (the default) goes by the first strong letter of each line.<br> <br>
Inputs and text edits turn the IME on by themselves while focused and draw what youre composing at the caret. If you do your own text box, <code>g.text_input(true, Some(caret))</code> starts it, then you get <code>TEXT</code> events with the finished text in <code>ev.msg</code> (the whole thing, not cut at 63 bytes) and <code>PREEDIT</code> ones while composing, with the caret in <code>ev.id</code>. From an engine you push them with <code>Event::text("日本")</code> and <code>Event::preedit("にほ", 3)</code>.<br> <br>
The ui kit builds an accessibility tree every frame and goome hands it to the screen reader in window mode. Its off in overlay and offscreen mode, <code>ui.a11y(true)</code> turns it on. <code>ui.a11y_tree()</code> gives you a <code>Vec&lt;A11yNode&gt;</code> with role, label, value, box, <code>parent</code> as an index and <code>focused()</code> / <code>checked()</code> and friends, so an engine can bridge it to its own. For a <code>ui.custom</code> widget call <code>ui.a11y_role(goome::ROLE_BUTTON, "Knob")</code> and <code>ui.a11y_value("11")</code> right before it.
</p>
</details>

<details>
<summary align="center"><b>📚 The API 📚</b></summary>

<br>

<p align="center">
Everything from <a href="https://github.com/goome-ui/c-goome/blob/main/include/goome.h">goome.h</a> is in here. <a href="https://goome-ui.github.io/docs/">The docs</a> explain every part, the names are the C ones without <code>goome_</code>. Constants lose the <code>GOOME_</code> too (<code>goome::DOWN</code>, <code>goome::WIN_MOVE</code>). Anything that can fail returns <code>goome::Result</code> with the C error code in <code>.code</code>.
</p>

```rust
let mut g = Goome::test("screen 800x600 1; end 5")?;   // or Options::new()...open(), Goome::app, Goome::offscreen
g.monitors();                                          // areas, scale, real px
g.events(16);                                          // an iterator, waits up to 16 ms for the first one
g.hotkey("alt+h", 1)?;                                 // comes back as a HOTKEY event
let img = g.capture(0, Some(Rect::new(0, 0, 200, 100)))?;
let bmp = Bitmap::load("logo.png")?;                   // freed when the last clone drops
let font = Font::system(None)?;                        // font.measure(16.0, "hi") gives (w, h)
let mut c = Canvas::new(160, 100, 160, 1.0)?;          // draw without any goome open
```

<p align="center">
Drawing lives on the <code>Draw</code> trait, so the overlay frame, <code>ui.canvas()</code> and your own <code>Canvas</code> all have <code>rect</code>, <code>rrect</code>, <code>shadow</code>, <code>circle</code>, <code>arc</code>, <code>poly</code>, <code>bitmap</code>, <code>text</code>, clipping and the rest.<br>
<code>goome::sys</code> is the raw FFI if you ever need it.<br> <br>
goome itself isnt thread safe, so the crate keeps it on one thread: the types arent <code>Send</code>, and opening goome on a second thread while the first still has stuff alive gives you an error instead of a crash. On Mac stay on the main thread.
</p>
</details>

<details>
<summary align="center"><b>🧩 Examples 🧩</b></summary>

<br>

- `examples/hello.rs` - a box you can click and type into, alt+h hides it, esc or the x quits
- `examples/ui_demo.rs` - the ui kit demo with tabs, images, a theme editor and a stats window
- `examples/app.rs` - a notes app in a normal window with menus, a sidebar in a split pane, a multiline editor and dialogs
- `examples/offscreen.rs` - draws the ui with no window and saves it as a PNG
- `examples/anim.rs` - a ball that slides over with whatever ease you pick, plus the curve
- `examples/text.rs` - emoji, hebrew and arabic with a switch for the text direction. Give it a font file to add as a fallback
- `examples/a11y.rs` - prints the accessibility tree of a small settings window

```sh
cargo run --example ui_demo
cargo run --example offscreen -- out.png
```

<p align="center">
Theyre ports of the C and Python ones. Way more small projects live in <a href="https://github.com/goome-ui/examples">goome-ui/examples</a>.
</p>
</details>

<details>
<summary align="center"><b>🖥️ Where It Works 🖥️</b></summary>

<br>

<p align="center">
Linux (X11 and Wayland), Windows 10 / 11 and macOS 14+, same as goome. Not every desktop can do everything, the <a href="https://github.com/goome-ui/c-goome#readme">c-goome README</a> has the table and <code>g.caps() &amp; goome::CAP_HOTKEYS</code> tells you at runtime.
</p>
</details>

<details>
<summary align="center"><b>🧪 Testing Without A Screen 🧪</b></summary>

<br>

<p align="center">
goome has a <b>test</b> backend that fakes a screen and plays a script of what the "user" does. All the tests here run on it, no display needed:
</p>

```sh
cargo test
```

<p align="center">
You can test your own app the same way, with env vars and no code changes:
</p>

```sh
GOOME_BACKEND=test GOOME_TEST_SCRIPT="1 click 400,300; 2 key hi\n; end 10" cargo run
```

<p align="center">
or from Rust with <code>Goome::test("screen 800x600 1; 1 click 120,90; end 5")</code>. The script format is in the <a href="https://github.com/goome-ui/c-goome#readme">c-goome README</a>.<br> <br>
The tests check the struct layouts and every constant against C, and draw the same scenes from Rust and from C in the same process, so the pixel hashes have to match on every OS. A few of them are the exact numbers from c-goome's own tests.
</p>
</details>

<details>
<summary align="center"><b>🔧 Updating goome 🔧</b></summary>

<br>

```sh
tools/update_native.sh ../c-goome      # or a path to any goome.h, or nothing to clone main
cargo test
```

<p align="center">
<code>goome-sys/native/VERSION</code> keeps the commit it came from. If the layout test fails after that, a struct, constant or function changed and <code>goome-sys/src/lib.rs</code> needs the same change.
</p>
</details>

<details>
<summary align="center"><b>🛠 Pull Requests 🛠</b></summary>

<br>

<p align="center">
Want to help with rust-goome? Make A pull request to <b>dev</b>!<br>
Small fixes are the easiest to merge, for anything bigger open an issue first so we can talk about it. Bugs in the C side go to <a href="https://github.com/goome-ui/c-goome">c-goome</a>.<br> <br>
As long as it works and doesn't add a dependency its very likely to get accepted!<br>
Read <a href="CONTRIBUTING.md">CONTRIBUTING.md</a> before you send one.
</p>
</details>

<details>
<summary align="center"><b>⚖️ License ⚖️</b></summary>

<br>

<p align="center">
rust-goome is under the <a href="LICENSE">GPL-3.0</a>.<br> <br>
<sub>
• You can use it, change it and share it<br>
• If you share something made with it, you have to share your source too, under the same license<br>
• Keep the credits
</sub><br> <br>
In short: play fair, keep it open, and give credit where it's due.
</p>
</details>

<details>
<summary align="center"><b>🩷 Donate 🩷</b></summary>

<br>

<p align="center">
<a href="https://sevvy-wevvy.com/kofi">Ko-fi</a> (Preferred)<br>
<a href="https://sevvy-wevvy.com/patreon">Patreon</a><br>
<a href="https://sevvy-wevvy.com/throne">Throne</a>
</p>
</details>

<p align="center">
<b>🫶 Contributors 🫶</b><br>
Thanks to everyone who helped out!<br> <br>
<a href="https://github.com/goome-ui/rust-goome/graphs/contributors"><img src="https://contrib.rocks/image?repo=goome-ui/rust-goome" alt="contributors"></a>
</p>
