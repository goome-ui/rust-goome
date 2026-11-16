#![doc = include_str!("../README.md")]

pub use goome_sys as sys;

mod app;
mod consts;
mod draw;
mod ui;

pub use app::{App, AppFrame};
pub use consts::*;
pub use draw::{builtin_font_count, builtin_measure, set_builtin_font, Bitmap, Canvas, Draw, Font, Frame, Radii, UiCanvas};
#[doc(hidden)]
pub use ui::mirror_layout;
pub use ui::{Config, Item, Scope, Skin, Style, Theme, Ui, UiFrame, WindowState};

use std::ffi::{c_char, c_int, CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub code: i32,
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

pub(crate) fn check(r: c_int) -> Result<()> {
    if r < 0 {
        Err(Error::code(r))
    } else {
        Ok(())
    }
}

pub fn strerror(code: i32) -> String {
    unsafe { from_c(sys::goome_strerror(code)).unwrap_or_default() }
}

pub fn parse_combo(combo: &str) -> Result<(i32, i32)> {
    let c = cstr(combo);
    let (mut key, mut mods) = (0, 0);
    let r = unsafe { sys::goome_parse_combo(c.as_ptr(), &mut key, &mut mods) };
    if r != OK {
        return Err(Error::new(r, format!("cant read the hotkey {:?}", combo)));
    }
    Ok((key, mods))
}

pub fn send(state_dir: &str, msg: &str) -> Result<()> {
    let (d, m) = (cstr(state_dir), cstr(msg));
    check(unsafe { sys::goome_send(d.as_ptr(), m.as_ptr()) })
}

pub fn now_ms() -> i64 {
    unsafe { sys::goome_now_ms() as i64 }
}

pub fn save_ppm(path: &str, px: &[u32], w: usize, h: usize, stride: usize) -> Result<()> {
    if w > stride || (h > 0 && px.len() < (h - 1) * stride + w) {
        return Err(Error::new(EARG, "the pixels dont fit that size"));
    }
    let p = cstr(path);
    check(unsafe { sys::goome_save_ppm(p.as_ptr(), px.as_ptr(), w as c_int, h as c_int, stride as c_int) })
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Monitor {
    pub id: i32,
    pub name: String,
    pub area: Rect,
    pub scale: f64,
    pub pw: i32,
    pub ph: i32,
    pub rotation: i32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
    pub kind: i32,
    pub x: f64,
    pub y: f64,
    pub mon: i32,
    pub button: i32,
    pub dy: i32,
    pub key: i32,
    pub mods: i32,
    pub text: String,
    pub id: i32,
    pub msg: String,
}

impl Event {
    pub fn new(kind: i32) -> Event {
        Event { kind, ..Event::default() }
    }

    pub fn motion(x: f64, y: f64) -> Event {
        Event {
            kind: MOTION,
            x,
            y,
            ..Event::default()
        }
    }

    pub fn down(x: f64, y: f64, button: i32) -> Event {
        Event {
            kind: DOWN,
            x,
            y,
            button,
            ..Event::default()
        }
    }

    pub fn up(x: f64, y: f64, button: i32) -> Event {
        Event {
            kind: UP,
            x,
            y,
            button,
            ..Event::default()
        }
    }

    pub fn scroll(x: f64, y: f64, dy: i32) -> Event {
        Event {
            kind: SCROLL,
            x,
            y,
            dy,
            ..Event::default()
        }
    }

    pub fn key(key: i32, mods: i32, text: &str) -> Event {
        Event {
            kind: KEY,
            key,
            mods,
            text: text.to_string(),
            ..Event::default()
        }
    }

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

#[derive(Clone, Debug)]
pub struct Image {
    pub pixels: Vec<u32>,
    pub w: usize,
    pub h: usize,
    pub area: Rect,
    pub mon: i32,
    pub has_self: bool,
    pub has_cursor: bool,
    pub cursor: Rect,
}

impl Image {
    pub fn pixel(&self, x: usize, y: usize) -> u32 {
        self.pixels[y * self.w + x]
    }

    pub fn row(&self, y: usize) -> &[u32] {
        &self.pixels[y * self.w..(y + 1) * self.w]
    }
}

pub struct FrameView<'a> {
    raw: sys::goome_image,
    _g: PhantomData<&'a Goome>,
}

impl<'a> FrameView<'a> {
    pub fn w(&self) -> usize {
        self.raw.w as usize
    }

    pub fn h(&self) -> usize {
        self.raw.h as usize
    }

    pub fn stride(&self) -> usize {
        self.raw.stride as usize
    }

    pub fn pixels(&self) -> &'a [u32] {
        if self.raw.px.is_null() || self.raw.h <= 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.raw.px, self.h() * self.stride()) }
    }

    pub fn row(&self, y: usize) -> &'a [u32] {
        let s = self.stride();
        &self.pixels()[y * s..y * s + self.w()]
    }

    pub fn pixel(&self, x: usize, y: usize) -> u32 {
        self.pixels()[y * self.stride() + x]
    }

    pub fn raw(&self) -> &sys::goome_image {
        &self.raw
    }
}

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub title: Option<String>,
    pub backend: i32,
    pub state_dir: Option<String>,
    pub no_capture: bool,
    pub capture_fps: i32,
    pub hide_self: bool,
    pub test_frames: Option<String>,
    pub test_script: Option<String>,
    pub test_shots: Option<String>,
    pub mode: i32,
    pub width: i32,
    pub height: i32,
    pub min_width: i32,
    pub min_height: i32,
    pub window_flags: i32,
    pub scale: f64,
}

impl Options {
    pub fn new() -> Options {
        Options::default()
    }

    pub fn title(mut self, t: &str) -> Options {
        self.title = Some(t.to_string());
        self
    }

    pub fn backend(mut self, b: i32) -> Options {
        self.backend = b;
        self
    }

    pub fn state_dir(mut self, d: &str) -> Options {
        self.state_dir = Some(d.to_string());
        self
    }

    pub fn no_capture(mut self, on: bool) -> Options {
        self.no_capture = on;
        self
    }

    pub fn capture_fps(mut self, fps: i32) -> Options {
        self.capture_fps = fps;
        self
    }

    pub fn hide_self(mut self, on: bool) -> Options {
        self.hide_self = on;
        self
    }

    pub fn test_frames(mut self, s: &str) -> Options {
        self.test_frames = Some(s.to_string());
        self
    }

    pub fn test_script(mut self, s: &str) -> Options {
        self.test_script = Some(s.to_string());
        self
    }

    pub fn test_shots(mut self, s: &str) -> Options {
        self.test_shots = Some(s.to_string());
        self
    }

    pub fn mode(mut self, m: i32) -> Options {
        self.mode = m;
        self
    }

    pub fn size(mut self, w: i32, h: i32) -> Options {
        self.width = w;
        self.height = h;
        self
    }

    pub fn min_size(mut self, w: i32, h: i32) -> Options {
        self.min_width = w;
        self.min_height = h;
        self
    }

    pub fn window_flags(mut self, f: i32) -> Options {
        self.window_flags = f;
        self
    }

    pub fn scale(mut self, s: f64) -> Options {
        self.scale = s;
        self
    }

    pub fn open(&self) -> Result<Goome> {
        Goome::open(self)
    }
}

pub struct Pacer {
    raw: sys::goome_pacer,
    pub fps: i32,
}

impl Pacer {
    pub fn new(fps: i32) -> Pacer {
        Pacer {
            raw: sys::goome_pacer::default(),
            fps,
        }
    }

    pub fn wait(&mut self) -> i32 {
        unsafe { sys::goome_pace(&mut self.raw, self.fps) }
    }
}

pub struct Goome {
    g: NonNull<sys::goome>,
    _claim: Claim,
    _not_send: PhantomData<*mut ()>,
}

impl Goome {
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

    pub fn offscreen(w: i32, h: i32, scale: f64) -> Result<Goome> {
        Options::new().title("goome").mode(MODE_OFFSCREEN).size(w, h).scale(scale).open()
    }

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

    pub fn raw(&self) -> *mut sys::goome {
        self.g.as_ptr()
    }

    pub fn ui(self) -> Result<Ui> {
        Ui::new(self)
    }

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

    pub fn backend_name(&self) -> String {
        unsafe { from_c(sys::goome_backend_name(self.raw())).unwrap_or_default() }
    }

    pub fn caps(&self) -> i32 {
        unsafe { sys::goome_caps(self.raw()) }
    }

    pub fn mode(&self) -> i32 {
        unsafe { sys::goome_mode(self.raw()) }
    }

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

    pub fn alpha_at(&self, x: f64, y: f64) -> i32 {
        unsafe { sys::goome_alpha_at(self.raw(), x, y) }
    }

    pub fn begin(&mut self, mon: i32) -> Option<Frame<'_>> {
        let c = unsafe { sys::goome_begin(self.raw(), mon) };
        NonNull::new(c).map(|c| Frame::new(self, c))
    }

    pub fn poll(&self, timeout_ms: i32) -> Option<Event> {
        let mut e: sys::goome_event = unsafe { std::mem::zeroed() };
        if unsafe { sys::goome_poll(self.raw(), &mut e, timeout_ms) } > 0 {
            Some(Event::from_raw(&e))
        } else {
            None
        }
    }

    pub fn events(&self, timeout_ms: i32) -> Events<'_> {
        Events { g: self, wait: timeout_ms }
    }

    pub fn pointer(&self) -> Option<(f64, f64, i32)> {
        let (mut x, mut y, mut b) = (0.0, 0.0, 0);
        if unsafe { sys::goome_pointer(self.raw(), &mut x, &mut y, &mut b) } < 0 {
            None
        } else {
            Some((x, y, b))
        }
    }

    pub fn keyboard(&self, on: bool) -> Result<()> {
        self.fail(unsafe { sys::goome_keyboard(self.raw(), on as c_int) })
    }

    pub fn grab(&self, on: bool) -> Result<()> {
        self.fail(unsafe { sys::goome_grab(self.raw(), on as c_int) })
    }

    pub fn grabbed(&self) -> bool {
        unsafe { sys::goome_grabbed(self.raw()) > 0 }
    }

    pub fn hotkey(&self, combo: &str, id: i32) -> Result<()> {
        let c = cstr(combo);
        self.fail(unsafe { sys::goome_hotkey(self.raw(), c.as_ptr(), id) })
    }

    pub fn control_path(&self) -> Option<String> {
        unsafe { from_c(sys::goome_control_path(self.raw())) }
    }

    pub fn window_title(&self, title: &str) -> Result<()> {
        let t = cstr(title);
        self.fail(unsafe { sys::goome_window_title(self.raw(), t.as_ptr()) })
    }

    pub fn window_size(&self, w: i32, h: i32) -> Result<()> {
        self.fail(unsafe { sys::goome_window_size(self.raw(), w, h) })
    }

    pub fn window_pos(&self) -> Result<(i32, i32)> {
        let (mut x, mut y) = (0, 0);
        self.fail(unsafe { sys::goome_window_pos(self.raw(), &mut x, &mut y, 0) })?;
        Ok((x, y))
    }

    pub fn set_window_pos(&self, x: i32, y: i32) -> Result<()> {
        let (mut x, mut y) = (x, y);
        self.fail(unsafe { sys::goome_window_pos(self.raw(), &mut x, &mut y, 1) })
    }

    pub fn window_state(&self, state: i32) -> Result<()> {
        self.fail(unsafe { sys::goome_window_state(self.raw(), state) })
    }

    pub fn window_get_state(&self) -> i32 {
        unsafe { sys::goome_window_get_state(self.raw()) }
    }

    pub fn window_drag(&self, edges: i32) -> Result<()> {
        self.fail(unsafe { sys::goome_window_drag(self.raw(), edges) })
    }

    pub fn window_show(&self, on: bool) -> Result<()> {
        self.fail(unsafe { sys::goome_window_show(self.raw(), on as c_int) })
    }

    pub fn cursor(&self, shape: i32) -> Result<()> {
        self.fail(unsafe { sys::goome_cursor(self.raw(), shape) })
    }

    pub fn cursor_shape(&self) -> i32 {
        unsafe { sys::goome_cursor_shape(self.raw()) }
    }

    pub fn clipboard_set(&self, text: &str) -> Result<()> {
        let t = cstr(text);
        self.fail(unsafe { sys::goome_clipboard_set(self.raw(), t.as_ptr()) })
    }

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

    pub fn drop_path(&self) -> Option<String> {
        unsafe { from_c(sys::goome_drop_path(self.raw())) }
    }

    pub fn push(&self, ev: &Event) -> Result<()> {
        let e = ev.to_raw();
        self.fail(unsafe { sys::goome_push(self.raw(), &e) })
    }

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

    pub fn resize(&mut self, w: i32, h: i32, scale: f64) -> Result<()> {
        self.fail(unsafe { sys::goome_resize(self.raw(), w, h, scale) })
    }

    pub fn last_frame(&self) -> Result<FrameView<'_>> {
        let mut img: sys::goome_image = unsafe { std::mem::zeroed() };
        self.fail(unsafe { sys::goome_frame(self.raw(), &mut img) })?;
        Ok(FrameView { raw: img, _g: PhantomData })
    }

    pub fn hit(&self, x: f64, y: f64) -> bool {
        unsafe { sys::goome_hit(self.raw(), x, y) > 0 }
    }

    pub fn frame_dirty(&self) -> Option<Rect> {
        let mut r = sys::goome_rect::default();
        if unsafe { sys::goome_frame_dirty(self.raw(), &mut r) } > 0 {
            Some(Rect::from_raw(&r))
        } else {
            None
        }
    }

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
