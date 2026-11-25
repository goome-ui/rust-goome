#![doc = include_str!("../README.md")]

pub use goome_sys as sys;

mod app;
mod consts;
mod draw;
mod ui;

pub use app::{App, AppFrame};
pub use consts::*;
pub use draw::{builtin_font_count, builtin_measure, set_builtin_font, text_dir, Bitmap, Canvas, Draw, Font, Frame, Radii, UiCanvas};
#[doc(hidden)]
pub use ui::mirror_layout;
pub use ui::{ease, A11yNode, Config, Item, Scope, Skin, Style, Theme, Ui, UiFrame, WindowState};

use std::ffi::{c_char, c_int, CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

/// The goome version this crate was built against.
pub const VERSION: &str = sys::GOOME_VERSION;

struct Owner {
    thread: Option<std::thread::ThreadId>,
    users: usize,
    test: bool,
}

static OWNER: std::sync::Mutex<Owner> = std::sync::Mutex::new(Owner {
    thread: None,
    users: 0,
    test: false,
});

pub(crate) struct Claim {
    test: bool,
}

pub(crate) fn claim() -> Result<Claim> {
    let mut o = OWNER.lock().unwrap_or_else(|e| e.into_inner());
    let me = std::thread::current().id();
    if o.users > 0 && o.thread != Some(me) {
        return Err(Error::new(EAGAIN, "goome is already in use on another thread, keep it all on one thread"));
    }
    o.thread = Some(me);
    o.users += 1;
    Ok(Claim { test: false })
}

impl Claim {
    fn take_test(&mut self) -> Result<()> {
        let mut o = OWNER.lock().unwrap_or_else(|e| e.into_inner());
        if o.test {
            return Err(Error::new(EAGAIN, "theres already a goome open on the test backend, close it first"));
        }
        o.test = true;
        self.test = true;
        Ok(())
    }
}

impl Drop for Claim {
    fn drop(&mut self) {
        let mut o = OWNER.lock().unwrap_or_else(|e| e.into_inner());
        o.users -= 1;
        if self.test {
            o.test = false;
        }
        if o.users == 0 {
            o.thread = None;
        }
    }
}

fn wants_test(backend: i32) -> bool {
    match std::env::var("GOOME_BACKEND") {
        Ok(v) if !v.is_empty() => v.eq_ignore_ascii_case("test"),
        _ => backend == TEST,
    }
}

/// What goome gives back when something fails: a `code` (one of the `E*` consts) and a message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    /// One of the `E*` consts.
    pub code: i32,
    /// Plain text for what went wrong.
    pub message: String,
}

impl Error {
    pub(crate) fn new(code: i32, message: impl Into<String>) -> Error {
        Error { code, message: message.into() }
    }

    pub(crate) fn code(code: i32) -> Error {
        Error::new(code, strerror(code))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "goome error {}", self.code)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for Error {}

/// `Result` with goome's [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn cstr(s: &str) -> CString {
    match CString::new(s) {
        Ok(c) => c,
        Err(e) => {
            let mut b = e.into_vec();
            b.retain(|&c| c != 0);
            CString::new(b).unwrap_or_default()
        }
    }
}

pub(crate) fn opt_cstr(s: Option<&str>) -> Option<CString> {
    s.map(cstr)
}

pub(crate) fn opt_ptr(s: &Option<CString>) -> *const c_char {
    s.as_ref().map_or(ptr::null(), |c| c.as_ptr())
}

pub(crate) unsafe fn from_c(p: *const c_char) -> Option<String> {
    if p.is_null() {
        None
    } else {
        Some(CStr::from_ptr(p).to_string_lossy().into_owned())
    }
}

pub(crate) fn from_chars(b: &[c_char]) -> String {
    let bytes: Vec<u8> = b.iter().take_while(|&&c| c != 0).map(|&c| c as u8).collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

pub(crate) fn to_chars(s: &str, out: &mut [c_char]) {
    let mut n = s.len().min(out.len().saturating_sub(1));
    while n > 0 && !s.is_char_boundary(n) {
        n -= 1;
    }
    for (o, &b) in out.iter_mut().zip(s.as_bytes()[..n].iter()) {
        *o = b as c_char;
    }
    for o in out[n..].iter_mut() {
        *o = 0;
    }
}

pub(crate) fn fits(len: usize, w: usize, h: usize, stride: usize) -> bool {
    let top = i32::MAX as usize;
    if w > stride || w > top || h > top || stride > top {
        return false;
    }
    h == 0 || (h - 1).checked_mul(stride).and_then(|n| n.checked_add(w)).is_some_and(|n| len >= n)
}

pub(crate) fn check(r: c_int) -> Result<()> {
    if r < 0 {
        Err(Error::code(r))
    } else {
        Ok(())
    }
}

/// Turns a goome error code into readable text.
pub fn strerror(code: i32) -> String {
    unsafe { from_c(sys::goome_strerror(code)).unwrap_or_default() }
}

/// Reads a hotkey string like `ctrl+shift+k` into `(key, mods)`.
pub fn parse_combo(combo: &str) -> Result<(i32, i32)> {
    let c = cstr(combo);
    let (mut key, mut mods) = (0, 0);
    let r = unsafe { sys::goome_parse_combo(c.as_ptr(), &mut key, &mut mods) };
    if r != OK {
        return Err(Error::new(r, format!("cant read the hotkey {:?}", combo)));
    }
    Ok((key, mods))
}

/// Sends a line to the control fifo of another goome app that uses the same `state_dir`.
pub fn send(state_dir: &str, msg: &str) -> Result<()> {
    let (d, m) = (cstr(state_dir), cstr(msg));
    check(unsafe { sys::goome_send(d.as_ptr(), m.as_ptr()) })
}

/// Milliseconds on goome's clock.
pub fn now_ms() -> i64 {
    unsafe { sys::goome_now_ms() as i64 }
}

/// Writes pixels out as a ppm file. `stride` is in pixels.
pub fn save_ppm(path: &str, px: &[u32], w: usize, h: usize, stride: usize) -> Result<()> {
    if !fits(px.len(), w, h, stride) {
        return Err(Error::new(EARG, "the pixels dont fit that size"));
    }
    let p = cstr(path);
    check(unsafe { sys::goome_save_ppm(p.as_ptr(), px.as_ptr(), w as c_int, h as c_int, stride as c_int) })
}

/// A rectangle in integer coords.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    /// Left edge.
    pub x: i32,
    /// Top edge.
    pub y: i32,
    /// Width.
    pub w: i32,
    /// Height.
    pub h: i32,
}

impl Rect {
    /// Makes a rect from its left, top, width and height.
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
    }

    pub(crate) fn raw(&self) -> sys::goome_rect {
        sys::goome_rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub(crate) fn from_raw(r: &sys::goome_rect) -> Rect {
        Rect {
            x: r.x,
            y: r.y,
            w: r.w,
            h: r.h,
        }
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from(r: (i32, i32, i32, i32)) -> Rect {
        Rect::new(r.0, r.1, r.2, r.3)
    }
}

/// One monitor, as `Goome::monitors` reports it.
#[derive(Clone, Debug, PartialEq)]
pub struct Monitor {
    /// Index to pass as `mon` to `capture`, `begin` and friends.
    pub id: i32,
    /// Name the os gives it.
    pub name: String,
    /// Where it sits on the desktop, logical coords.
    pub area: Rect,
    /// Real px per logical px.
    pub scale: f64,
    /// Width in real px.
    pub pw: i32,
    /// Height in real px.
    pub ph: i32,
    /// 0, 90, 180 or 270, +360 if mirrored. Grabs and canvases already undo it.
    pub rotation: i32,
}

/// One input or window event from `Goome::poll`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    /// Which event it is, one of `DOWN`, `UP`, `MOTION`, `SCROLL`, `KEY`, `HOTKEY`, `CONTROL`, `MONITORS`, `QUIT`, `CLOSE`, `FOCUS`, `DROP`, `TEXT`, `PREEDIT`.
    pub kind: i32,
    /// Pointer x, logical.
    pub x: f64,
    /// Pointer y, logical.
    pub y: f64,
    /// Monitor the event happened on.
    pub mon: i32,
    /// 1 left, 2 middle, 3 right.
    pub button: i32,
    /// Scroll notches, +1 = down.
    pub dy: i32,
    /// Lowercase ascii or one of the `K_*` consts.
    pub key: i32,
    /// `SHIFT`, `CTRL`, `ALT` and `SUPER` ored together.
    pub mods: i32,
    /// The utf-8 the key typed, can be empty.
    pub text: String,
    /// Meaning depends on `kind`: the hotkey id, `FOCUS` is 1 for gained and 0 for lost, `PREEDIT` is the caret byte offset.
    pub id: i32,
    /// The `CONTROL` line, the path for `DROP`, or the text for `TEXT` and `PREEDIT`.
    pub msg: String,
}

impl Event {
    /// Makes an empty event of that kind.
    pub fn new(kind: i32) -> Event {
        Event { kind, ..Event::default() }
    }

    /// A pointer move to x,y.
    pub fn motion(x: f64, y: f64) -> Event {
        Event {
            kind: MOTION,
            x,
            y,
            ..Event::default()
        }
    }

    /// A mouse button press at x,y. Button is 1 left, 2 middle, 3 right.
    pub fn down(x: f64, y: f64, button: i32) -> Event {
        Event {
            kind: DOWN,
            x,
            y,
            button,
            ..Event::default()
        }
    }

    /// A mouse button release at x,y.
    pub fn up(x: f64, y: f64, button: i32) -> Event {
        Event {
            kind: UP,
            x,
            y,
            button,
            ..Event::default()
        }
    }

    /// A scroll at x,y, `dy` in notches, +1 = down.
    pub fn scroll(x: f64, y: f64, dy: i32) -> Event {
        Event {
            kind: SCROLL,
            x,
            y,
            dy,
            ..Event::default()
        }
    }

    /// A key press. `key` is lowercase ascii or a `K_*` const, `mods` is `SHIFT`/`CTRL`/`ALT`/`SUPER` ored.
    pub fn key(key: i32, mods: i32, text: &str) -> Event {
        Event {
            kind: KEY,
            key,
            mods,
            text: text.to_string(),
            ..Event::default()
        }
    }

    /// A finished chunk of typed text, like from an ime or dead key.
    pub fn text(text: &str) -> Event {
        Event {
            kind: TEXT,
            msg: text.to_string(),
            ..Event::default()
        }
    }

    /// Text that is still being composed, with the caret as a byte offset into it.
    pub fn preedit(text: &str, cursor: usize) -> Event {
        Event {
            kind: PREEDIT,
            msg: text.to_string(),
            id: cursor.min(i32::MAX as usize) as i32,
            ..Event::default()
        }
    }

    /// Copies a raw event from the C side.
    pub fn from_raw(e: &sys::goome_event) -> Event {
        Event {
            kind: e.type_,
            x: e.x,
            y: e.y,
            mon: e.mon,
            button: e.button,
            dy: e.dy,
            key: e.key,
            mods: e.mods,
            text: from_chars(&e.text),
            id: e.id,
            msg: from_chars(&e.msg),
        }
    }

    /// Turns it back into a raw C event.
    pub fn to_raw(&self) -> sys::goome_event {
        let mut e: sys::goome_event = unsafe { std::mem::zeroed() };
        e.type_ = self.kind;
        e.x = self.x;
        e.y = self.y;
        e.mon = self.mon;
        e.button = self.button;
        e.dy = self.dy;
        e.key = self.key;
        e.mods = self.mods;
        to_chars(&self.text, &mut e.text);
        e.id = self.id;
        to_chars(&self.msg, &mut e.msg);
        e
    }
}

/// A grabbed screen image.
#[derive(Clone, Debug)]
pub struct Image {
    /// `0xFFRRGGBB`, `w * h` of them with no row padding.
    pub pixels: Vec<u32>,
    /// Width in real px.
    pub w: usize,
    /// Height in real px.
    pub h: usize,
    /// The logical desktop area it covers.
    pub area: Rect,
    /// Monitor it came from.
    pub mon: i32,
    /// Our own overlay is in it, mask it with `Goome::alpha_at`.
    pub has_self: bool,
    /// The mouse got baked in at `cursor`.
    pub has_cursor: bool,
    /// Where the mouse is in the image, logical.
    pub cursor: Rect,
}

impl Image {
    /// The pixel at x,y.
    pub fn pixel(&self, x: usize, y: usize) -> u32 {
        self.pixels[y * self.w + x]
    }

    /// One row of pixels.
    pub fn row(&self, y: usize) -> &[u32] {
        &self.pixels[y * self.w..(y + 1) * self.w]
    }
}

/// A look at the last frame goome sent. Good until the next `begin`.
pub struct FrameView<'a> {
    raw: sys::goome_image,
    _g: PhantomData<&'a Goome>,
}

impl<'a> FrameView<'a> {
    /// Width in real px.
    pub fn w(&self) -> usize {
        self.raw.w as usize
    }

    /// Height in real px.
    pub fn h(&self) -> usize {
        self.raw.h as usize
    }

    /// Row length in pixels, can be more than the width.
    pub fn stride(&self) -> usize {
        self.raw.stride as usize
    }

    /// Premultiplied argb, `stride` pixels per row.
    pub fn pixels(&self) -> &'a [u32] {
        if self.raw.px.is_null() || self.raw.h <= 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.raw.px, self.h() * self.stride()) }
    }

    /// One row of pixels, `w` long.
    pub fn row(&self, y: usize) -> &'a [u32] {
        let s = self.stride();
        &self.pixels()[y * s..y * s + self.w()]
    }

    /// The pixel at x,y.
    pub fn pixel(&self, x: usize, y: usize) -> u32 {
        self.pixels()[y * self.stride() + x]
    }

    /// The raw C image.
    pub fn raw(&self) -> &sys::goome_image {
        &self.raw
    }
}

/// Settings for opening goome. Build it with the chained setters, then call `open`.
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// Window title, `None` = "goome".
    pub title: Option<String>,
    /// `AUTO` or force one. The `GOOME_BACKEND` env var wins.
    pub backend: i32,
    /// Where the portal token and control fifo live, `None` = neither.
    pub state_dir: Option<String>,
    /// Overlay only, never asks for screen access.
    pub no_capture: bool,
    /// For gnome and portal streams, 0 = 5.
    pub capture_fps: i32,
    /// Keeps our overlay out of grabs where the os lets us.
    pub hide_self: bool,
    /// `TEST` backend only, env `GOOME_TEST_FRAMES`.
    pub test_frames: Option<String>,
    /// `TEST` backend only, env `GOOME_TEST_SCRIPT`.
    pub test_script: Option<String>,
    /// `TEST` backend only, env `GOOME_TEST_SHOTS`.
    pub test_shots: Option<String>,
    /// `MODE_OVERLAY`, `MODE_WINDOW` or `MODE_OFFSCREEN`.
    pub mode: i32,
    /// Window or offscreen width, logical. 0 = 960.
    pub width: i32,
    /// Window or offscreen height, logical. 0 = 640.
    pub height: i32,
    /// Smallest window width, window mode only, 0 = none.
    pub min_width: i32,
    /// Smallest window height, window mode only, 0 = none.
    pub min_height: i32,
    /// `WINDOW_*` flags ored together.
    pub window_flags: i32,
    /// Offscreen only, 0 = 1.
    pub scale: f64,
}

impl Options {
    /// Default options.
    pub fn new() -> Options {
        Options::default()
    }

    /// Sets the title.
    pub fn title(mut self, t: &str) -> Options {
        self.title = Some(t.to_string());
        self
    }

    /// Sets the backend.
    pub fn backend(mut self, b: i32) -> Options {
        self.backend = b;
        self
    }

    /// Sets the state dir.
    pub fn state_dir(mut self, d: &str) -> Options {
        self.state_dir = Some(d.to_string());
        self
    }

    /// Sets `no_capture`.
    pub fn no_capture(mut self, on: bool) -> Options {
        self.no_capture = on;
        self
    }

    /// Sets `capture_fps`.
    pub fn capture_fps(mut self, fps: i32) -> Options {
        self.capture_fps = fps;
        self
    }

    /// Sets `hide_self`.
    pub fn hide_self(mut self, on: bool) -> Options {
        self.hide_self = on;
        self
    }

    /// Sets `test_frames`.
    pub fn test_frames(mut self, s: &str) -> Options {
        self.test_frames = Some(s.to_string());
        self
    }

    /// Sets `test_script`.
    pub fn test_script(mut self, s: &str) -> Options {
        self.test_script = Some(s.to_string());
        self
    }

    /// Sets `test_shots`.
    pub fn test_shots(mut self, s: &str) -> Options {
        self.test_shots = Some(s.to_string());
        self
    }

    /// Sets the mode.
    pub fn mode(mut self, m: i32) -> Options {
        self.mode = m;
        self
    }

    /// Sets the window or offscreen size, logical.
    pub fn size(mut self, w: i32, h: i32) -> Options {
        self.width = w;
        self.height = h;
        self
    }

    /// Sets the smallest window size, window mode only.
    pub fn min_size(mut self, w: i32, h: i32) -> Options {
        self.min_width = w;
        self.min_height = h;
        self
    }

    /// Sets the `WINDOW_*` flags.
    pub fn window_flags(mut self, f: i32) -> Options {
        self.window_flags = f;
        self
    }

    /// Sets the scale, offscreen only.
    pub fn scale(mut self, s: f64) -> Options {
        self.scale = s;
        self
    }

    /// Opens goome with these options.
    pub fn open(&self) -> Result<Goome> {
        Goome::open(self)
    }
}

/// Frame pacing for a loop. Call `wait` once per frame.
pub struct Pacer {
    raw: sys::goome_pacer,
    /// Frames per second to hold.
    pub fps: i32,
}

impl Pacer {
    /// Makes a pacer for that many frames per second.
    pub fn new(fps: i32) -> Pacer {
        Pacer {
            raw: sys::goome_pacer::default(),
            fps,
        }
    }

    /// Sleeps until the next frame is due and returns the ms it should wait.
    pub fn wait(&mut self) -> i32 {
        unsafe { sys::goome_pace(&mut self.raw, self.fps) }
    }
}

/// An open goome. Everything runs on one thread, it can't be sent to another.
pub struct Goome {
    g: NonNull<sys::goome>,
    _claim: Claim,
    _not_send: PhantomData<*mut ()>,
}

impl Goome {
    /// Opens goome from `Options`.
    pub fn open(o: &Options) -> Result<Goome> {
        let strs = [&o.title, &o.state_dir, &o.test_frames, &o.test_script, &o.test_shots].map(|s| opt_cstr(s.as_deref()));
        let raw = sys::goome_opts {
            title: opt_ptr(&strs[0]),
            backend: o.backend,
            state_dir: opt_ptr(&strs[1]),
            no_capture: o.no_capture as c_int,
            capture_fps: o.capture_fps,
            hide_self: o.hide_self as c_int,
            test_frames: opt_ptr(&strs[2]),
            test_script: opt_ptr(&strs[3]),
            test_shots: opt_ptr(&strs[4]),
            mode: o.mode,
            width: o.width,
            height: o.height,
            min_width: o.min_width,
            min_height: o.min_height,
            window_flags: o.window_flags,
            scale: o.scale,
        };
        let mut c = claim()?;
        if o.mode != MODE_OFFSCREEN && wants_test(o.backend) {
            c.take_test()?;
        }
        let mut err = 0;
        let g = unsafe { sys::goome_open(&raw, &mut err) };
        Goome::wrap(g, err, c)
    }

    /// Opens a normal resizable window, centered.
    pub fn app(title: &str, w: i32, h: i32) -> Result<Goome> {
        let t = cstr(title);
        let mut c = claim()?;
        if wants_test(AUTO) {
            c.take_test()?;
        }
        let mut err = 0;
        let g = unsafe { sys::goome_open_app(t.as_ptr(), w, h, &mut err) };
        Goome::wrap(g, err, c)
    }

    /// Opens an offscreen surface of that logical size and scale, no window.
    pub fn offscreen(w: i32, h: i32, scale: f64) -> Result<Goome> {
        Options::new().title("goome").mode(MODE_OFFSCREEN).size(w, h).scale(scale).open()
    }

    /// Opens the `TEST` backend and runs the given script.
    pub fn test(script: &str) -> Result<Goome> {
        Options::new().backend(TEST).test_script(script).open()
    }

    fn wrap(g: *mut sys::goome, err: c_int, c: Claim) -> Result<Goome> {
        match NonNull::new(g) {
            Some(g) => Ok(Goome {
                g,
                _claim: c,
                _not_send: PhantomData,
            }),
            None => Err(Error::code(if err != 0 { err } else { ENODISPLAY })),
        }
    }

    /// The raw C handle.
    pub fn raw(&self) -> *mut sys::goome {
        self.g.as_ptr()
    }

    /// Makes a [`Ui`] on top of this.
    pub fn ui(self) -> Result<Ui> {
        Ui::new(self)
    }

    /// Why the last call failed.
    pub fn error(&self) -> String {
        unsafe { from_c(sys::goome_error(self.raw())).unwrap_or_default() }
    }

    fn fail(&self, r: c_int) -> Result<()> {
        if r < 0 {
            let m = self.error();
            Err(Error::new(r, if m.is_empty() { strerror(r) } else { m }))
        } else {
            Ok(())
        }
    }

    /// Name of the backend in use.
    pub fn backend_name(&self) -> String {
        unsafe { from_c(sys::goome_backend_name(self.raw())).unwrap_or_default() }
    }

    /// `CAP_*` bits for what this backend can do.
    pub fn caps(&self) -> i32 {
        unsafe { sys::goome_caps(self.raw()) }
    }

    /// `MODE_OVERLAY`, `MODE_WINDOW` or `MODE_OFFSCREEN`.
    pub fn mode(&self) -> i32 {
        unsafe { sys::goome_mode(self.raw()) }
    }

    /// All monitors.
    pub fn monitors(&self) -> Vec<Monitor> {
        let mut out: Vec<sys::goome_monitor> = vec![unsafe { std::mem::zeroed() }; 16];
        let n = unsafe { sys::goome_monitors(self.raw(), out.as_mut_ptr(), out.len() as c_int) };
        out.truncate(n.max(0) as usize);
        out.iter()
            .map(|m| Monitor {
                id: m.id,
                name: from_chars(&m.name),
                area: Rect::from_raw(&m.area),
                scale: m.scale,
                pw: m.pw,
                ph: m.ph,
                rotation: m.rotation,
            })
            .collect()
    }

    /// Grabs the screen on that monitor. `r` is a real px rect, `None` = the whole thing.
    pub fn capture(&mut self, mon: i32, r: Option<Rect>) -> Result<Image> {
        let rr = r.map(|r| r.raw());
        let mut img: sys::goome_image = unsafe { std::mem::zeroed() };
        let res = unsafe { sys::goome_capture(self.raw(), mon, rr.as_ref().map_or(ptr::null(), |r| r as *const _), &mut img) };
        self.fail(res)?;
        let (w, h, s) = (img.w.max(0) as usize, img.h.max(0) as usize, img.stride.max(0) as usize);
        let mut px = Vec::with_capacity(w * h);
        if !img.px.is_null() {
            let all = unsafe { std::slice::from_raw_parts(img.px, if h == 0 { 0 } else { (h - 1) * s + w }) };
            for y in 0..h {
                px.extend_from_slice(&all[y * s..y * s + w]);
            }
        }
        Ok(Image {
            pixels: px,
            w,
            h,
            area: Rect::from_raw(&img.area),
            mon: img.mon,
            has_self: img.has_self != 0,
            has_cursor: img.has_cursor != 0,
            cursor: Rect::from_raw(&img.cursor),
        })
    }

    /// Alpha of our overlay at x,y (logical), for masking out our own pixels in a grab.
    pub fn alpha_at(&self, x: f64, y: f64) -> i32 {
        unsafe { sys::goome_alpha_at(self.raw(), x, y) }
    }

    /// Starts a frame on that monitor. `None` = nothing to draw on, skip the frame.
    pub fn begin(&mut self, mon: i32) -> Option<Frame<'_>> {
        let c = unsafe { sys::goome_begin(self.raw(), mon) };
        NonNull::new(c).map(|c| Frame::new(self, c))
    }

    /// Waits up to `timeout_ms` for an event.
    pub fn poll(&self, timeout_ms: i32) -> Option<Event> {
        let mut e: sys::goome_event = unsafe { std::mem::zeroed() };
        if unsafe { sys::goome_poll(self.raw(), &mut e, timeout_ms) } > 0 {
            let mut ev = Event::from_raw(&e);
            if ev.kind == TEXT || ev.kind == PREEDIT {
                ev.msg = self.event_text();
            }
            Some(ev)
        } else {
            None
        }
    }

    /// Iterator over events, waits `timeout_ms` for the first one then drains the rest.
    pub fn events(&self, timeout_ms: i32) -> Events<'_> {
        Events { g: self, wait: timeout_ms }
    }

    /// Pointer `(x, y, buttons)`, `None` if the backend cant say.
    pub fn pointer(&self) -> Option<(f64, f64, i32)> {
        let (mut x, mut y, mut b) = (0.0, 0.0, 0);
        if unsafe { sys::goome_pointer(self.raw(), &mut x, &mut y, &mut b) } < 0 {
            None
        } else {
            Some((x, y, b))
        }
    }

    /// Turns keyboard capture on or off.
    pub fn keyboard(&self, on: bool) -> Result<()> {
        self.fail(unsafe { sys::goome_keyboard(self.raw(), on as c_int) })
    }

    /// Overlay: focus + pointer capture until `grab(false)`. Window: drags keep going outside it.
    pub fn grab(&self, on: bool) -> Result<()> {
        self.fail(unsafe { sys::goome_grab(self.raw(), on as c_int) })
    }

    /// If the grab is on.
    pub fn grabbed(&self) -> bool {
        unsafe { sys::goome_grabbed(self.raw()) > 0 }
    }

    /// Registers a global hotkey like `ctrl+shift+k`. It shows up as a `HOTKEY` event with that `id`.
    pub fn hotkey(&self, combo: &str, id: i32) -> Result<()> {
        let c = cstr(combo);
        self.fail(unsafe { sys::goome_hotkey(self.raw(), c.as_ptr(), id) })
    }

    /// Path of the control fifo, `None` if there isnt one.
    pub fn control_path(&self) -> Option<String> {
        unsafe { from_c(sys::goome_control_path(self.raw())) }
    }

    /// Sets the window title.
    pub fn window_title(&self, title: &str) -> Result<()> {
        let t = cstr(title);
        self.fail(unsafe { sys::goome_window_title(self.raw(), t.as_ptr()) })
    }

    /// Sets the logical client size.
    pub fn window_size(&self, w: i32, h: i32) -> Result<()> {
        self.fail(unsafe { sys::goome_window_size(self.raw(), w, h) })
    }

    /// Where the window is. `ENOTSUP` where the os wont say (wayland).
    pub fn window_pos(&self) -> Result<(i32, i32)> {
        let (mut x, mut y) = (0, 0);
        self.fail(unsafe { sys::goome_window_pos(self.raw(), &mut x, &mut y, 0) })?;
        Ok((x, y))
    }

    /// Moves the window. `ENOTSUP` where the os wont allow it (wayland).
    pub fn set_window_pos(&self, x: i32, y: i32) -> Result<()> {
        let (mut x, mut y) = (x, y);
        self.fail(unsafe { sys::goome_window_pos(self.raw(), &mut x, &mut y, 1) })
    }

    /// Sets the window state, one of `STATE_*`.
    pub fn window_state(&self, state: i32) -> Result<()> {
        self.fail(unsafe { sys::goome_window_state(self.raw(), state) })
    }

    /// The current window state, one of `STATE_*`.
    pub fn window_get_state(&self) -> i32 {
        unsafe { sys::goome_window_get_state(self.raw()) }
    }

    /// Starts a native move (0) or resize (`EDGE_*` bits) from the press happening now.
    pub fn window_drag(&self, edges: i32) -> Result<()> {
        self.fail(unsafe { sys::goome_window_drag(self.raw(), edges) })
    }

    /// Shows or hides the window.
    pub fn window_show(&self, on: bool) -> Result<()> {
        self.fail(unsafe { sys::goome_window_show(self.raw(), on as c_int) })
    }

    /// Sets the cursor shape, one of `CURSOR_*`.
    pub fn cursor(&self, shape: i32) -> Result<()> {
        self.fail(unsafe { sys::goome_cursor(self.raw(), shape) })
    }

    /// The last shape asked for, offscreen embedders read this.
    pub fn cursor_shape(&self) -> i32 {
        unsafe { sys::goome_cursor_shape(self.raw()) }
    }

    /// Puts text on the clipboard.
    pub fn clipboard_set(&self, text: &str) -> Result<()> {
        let t = cstr(text);
        self.fail(unsafe { sys::goome_clipboard_set(self.raw(), t.as_ptr()) })
    }

    /// Reads the clipboard text. Can block about 250ms on x11 and wayland.
    pub fn clipboard_get(&self) -> Result<String> {
        let mut cap = 4096usize;
        loop {
            let mut buf = vec![0 as c_char; cap];
            let n = unsafe { sys::goome_clipboard_get(self.raw(), buf.as_mut_ptr(), cap as c_int) };
            self.fail(n)?;
            if (n as usize) < cap {
                return Ok(from_chars(&buf));
            }
            cap = n as usize + 1;
        }
    }

    /// Full path of the last `DROP` event, good until the next poll.
    pub fn drop_path(&self) -> Option<String> {
        unsafe { from_c(sys::goome_drop_path(self.raw())) }
    }

    /// Full text of the last `TEXT` or `PREEDIT` event, good until the next poll.
    pub fn event_text(&self) -> String {
        unsafe { from_c(sys::goome_event_text(self.raw())).unwrap_or_default() }
    }

    /// Byte offset of the caret in the last preedit text.
    pub fn preedit_cursor(&self) -> usize {
        unsafe { sys::goome_preedit_cursor(self.raw()).max(0) as usize }
    }

    /// Turns ime on or off. `caret` (logical) is where the candidate window goes.
    pub fn text_input(&self, on: bool, caret: Option<Rect>) -> Result<()> {
        let r = caret.map(|r| sys::goome_rect {
            x: r.x,
            y: r.y,
            w: r.w,
            h: r.h,
        });
        let p = r.as_ref().map_or(ptr::null(), |r| r as *const sys::goome_rect);
        self.fail(unsafe { sys::goome_text_input(self.raw(), on as c_int, p) })
    }

    /// Queues an event, any mode.
    pub fn push(&self, ev: &Event) -> Result<()> {
        let e = ev.to_raw();
        self.fail(unsafe { sys::goome_push(self.raw(), &e) })
    }

    /// Queues typed text as a `TEXT` event.
    pub fn push_text(&self, text: &str) -> Result<()> {
        for ch in text.chars() {
            let mut b = [0u8; 4];
            let s = ch.encode_utf8(&mut b);
            let key = if ch.is_ascii() { ch.to_ascii_lowercase() as i32 } else { 0 };
            let mods = if ch.is_ascii_uppercase() { SHIFT } else { 0 };
            self.push(&Event::key(key, mods, s))?;
        }
        Ok(())
    }

    /// Resizes an offscreen surface. Offscreen only.
    pub fn resize(&mut self, w: i32, h: i32, scale: f64) -> Result<()> {
        self.fail(unsafe { sys::goome_resize(self.raw(), w, h, scale) })
    }

    /// The last frame goome sent, premultiplied argb. Good until the next `begin`.
    pub fn last_frame(&self) -> Result<FrameView<'_>> {
        let mut img: sys::goome_image = unsafe { std::mem::zeroed() };
        self.fail(unsafe { sys::goome_frame(self.raw(), &mut img) })?;
        Ok(FrameView { raw: img, _g: PhantomData })
    }

    /// If x,y is in the hit rects of the last frame.
    pub fn hit(&self, x: f64, y: f64) -> bool {
        unsafe { sys::goome_hit(self.raw(), x, y) > 0 }
    }

    /// Real px rect that changed since the last call, `None` = nothing. The first one after open or resize is the whole frame.
    pub fn frame_dirty(&self) -> Option<Rect> {
        let mut r = sys::goome_rect::default();
        if unsafe { sys::goome_frame_dirty(self.raw(), &mut r) } > 0 {
            Some(Rect::from_raw(&r))
        } else {
            None
        }
    }

    /// Copies `r` (real px, `None` = whole) of the last frame into `dst` in the given `FMT_*` byte order. `dst` starts at the top left of `r`.
    pub fn frame_copy(&self, dst: &mut [u8], stride_bytes: usize, format: i32, r: Option<Rect>) -> Result<()> {
        let f = self.last_frame()?;
        let r = r.unwrap_or(Rect::new(0, 0, f.raw.w, f.raw.h));
        if r.x < 0 || r.y < 0 || r.w < 0 || r.h < 0 || r.x + r.w > f.raw.w || r.y + r.h > f.raw.h {
            return Err(Error::new(EARG, "that rect isnt inside the frame"));
        }
        let need = if r.h == 0 { 0 } else { (r.h as usize - 1) * stride_bytes + r.w as usize * 4 };
        if stride_bytes < r.w as usize * 4 || dst.len() < need || stride_bytes > i32::MAX as usize {
            return Err(Error::new(EARG, "dst is too small for that rect"));
        }
        let rr = r.raw();
        self.fail(unsafe { sys::goome_frame_copy(self.raw(), dst.as_mut_ptr().cast(), stride_bytes as c_int, format, &rr) })
    }
}

impl Drop for Goome {
    fn drop(&mut self) {
        unsafe { sys::goome_close(self.g.as_ptr()) }
    }
}

/// What `Goome::events` gives back.
pub struct Events<'a> {
    g: &'a Goome,
    wait: i32,
}

impl<'a> Iterator for Events<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        let e = self.g.poll(self.wait);
        self.wait = 0;
        e
    }
}
