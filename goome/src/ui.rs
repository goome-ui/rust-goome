use crate::draw::{Bitmap, Font, UiCanvas};
use crate::{cstr, opt_ptr, sys, Error, Event, Goome, Result, ACT_COUNT, CUR_COUNT, EARG, ICON_COUNT, OK, SKIN_COUNT};
use std::any::Any;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CString};
use std::ops::{Deref, DerefMut, Range};
use std::ptr::{self, NonNull};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub bg: u32,
    pub panel: u32,
    pub border: u32,
    pub text: u32,
    pub text_dim: u32,
    pub accent: u32,
    pub accent_text: u32,
    pub hover: u32,
    pub active: u32,
    pub input_bg: u32,
    pub shadow: u32,
    pub radius: f64,
    pub pad: f64,
    pub gap: f64,
    pub border_width: f64,
    pub font_size: f64,
    pub shadow_blur: f64,
    pub anim_ms: f64,
    font: *mut sys::goome_font,
    skin_button: sys::goome_skin,
    skin_panel: sys::goome_skin,
    skin_input: sys::goome_skin,
    icon: [*const sys::goome_bitmap; ICON_COUNT as usize],
    pub icon_tint: u32,
    skin_checkbox: sys::goome_skin,
    skin_radio: sys::goome_skin,
    skin_toggle: sys::goome_skin,
    skin_slider: sys::goome_skin,
    skin_slider_fill: sys::goome_skin,
    skin_scrollbar: sys::goome_skin,
    skin_tab: sys::goome_skin,
    skin_titlebar: sys::goome_skin,
    skin_tooltip: sys::goome_skin,
    skin_menu: sys::goome_skin,
    skin_popup: sys::goome_skin,
    pub focus: u32,
    pub select: u32,
    pub caret: u32,
    pub link: u32,
    pub badge: u32,
    pub toast: u32,
    pub tooltip: u32,
    pub danger: u32,
    pub knob: u32,
    pub dim: u32,
    pub check_size: f64,
    pub check_radius: f64,
    pub toggle_w: f64,
    pub toggle_h: f64,
    pub slider_h: f64,
    pub slider_track: f64,
    pub knob_size: f64,
    pub scroll_w: f64,
    pub scroll_min: f64,
    pub item_h: f64,
    pub menu_item_h: f64,
    pub tab_h: f64,
    pub win_title_h: f64,
    pub app_title_h: f64,
    pub menubar_h: f64,
    pub statusbar_h: f64,
    pub focus_w: f64,
    pub focus_gap: f64,
    pub sep_w: f64,
    pub grip: f64,
    pub indent: f64,
    pub plot_w: f64,
    pub spinner: f64,
    pub caret_w: f64,
    pub win_min_w: f64,
    pub win_min_h: f64,
    pub pop_gap: f64,
    pub menu_gap: f64,
    pub tip_pad_x: f64,
    pub tip_pad_y: f64,
    pub tip_x: f64,
    pub tip_y: f64,
    pub tip_radius: f64,
    pub shadow_x: f64,
    pub shadow_y: f64,
    pub align: f64,
    pub heading_size: f64,
    pub small_size: f64,
    pub ease: f64,
    pub hover_ms: f64,
    pub press_ms: f64,
    pub popup_ms: f64,
    pub collapse_ms: f64,
    pub toast_fade_ms: f64,
    font_heading: *mut sys::goome_font,
    font_mono: *mut sys::goome_font,
    font_small: *mut sys::goome_font,
}

impl Theme {
    pub fn dark() -> Theme {
        let mut t: Theme = unsafe { std::mem::zeroed() };
        unsafe { sys::goome_ui_theme_dark(t.raw()) };
        t
    }

    pub fn light() -> Theme {
        let mut t: Theme = unsafe { std::mem::zeroed() };
        unsafe { sys::goome_ui_theme_light(t.raw()) };
        t
    }

    pub fn make(accent: u32, dark: bool, radius: f64) -> Theme {
        let mut t = Theme::dark();
        t.set_made(accent, dark, radius);
        t
    }

    pub fn preset(preset: i32) -> Result<Theme> {
        let mut t = Theme::dark();
        t.set_preset(preset)?;
        Ok(t)
    }

    pub fn set_dark(&mut self) {
        unsafe { sys::goome_ui_theme_dark(self.raw()) }
    }

    pub fn set_light(&mut self) {
        unsafe { sys::goome_ui_theme_light(self.raw()) }
    }

    pub fn set_made(&mut self, accent: u32, dark: bool, radius: f64) {
        unsafe { sys::goome_ui_theme_make(self.raw(), accent, dark as c_int, radius) }
    }

    pub fn set_preset(&mut self, preset: i32) -> Result<()> {
        let r = unsafe { sys::goome_ui_theme_preset(self.raw(), preset) };
        if r != OK {
            return Err(Error::new(EARG, format!("theres no theme preset {}", preset)));
        }
        Ok(())
    }

    pub fn has_font(&self) -> bool {
        !self.font.is_null()
    }

    pub fn raw(&mut self) -> *mut sys::goome_theme {
        (self as *mut Theme).cast()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub tip_delay_ms: i32,
    pub dbl_click_ms: i32,
    pub dbl_click_px: i32,
    pub drag_px: i32,
    pub repeat_delay_ms: i32,
    pub repeat_ms: i32,
    pub blink_ms: i32,
    pub menu_delay_ms: i32,
    pub toast_ms: i32,
    pub scroll_lines: f64,
    pub raise_on_click: i32,
    pub esc_closes: i32,
    pub tab_nav: i32,
    pub focus_on_mouse: i32,
    pub cursor: [i32; CUR_COUNT as usize],
    pub keys: [[sys::goome_combo; 4]; ACT_COUNT as usize],
    ease: Option<unsafe extern "C" fn(t: f64, user: *mut c_void) -> f64>,
    ease_user: *mut c_void,
    mask: *const c_char,
}

impl Config {
    pub fn reset(&mut self) {
        unsafe { sys::goome_ui_config_default((self as *mut Config).cast()) }
        self.ease_user = ptr::null_mut();
    }

    pub fn set_ease(&mut self, f: Option<unsafe extern "C" fn(f64, *mut c_void) -> f64>) {
        self.ease = f;
        self.ease_user = ptr::null_mut();
    }
}

#[doc(hidden)]
pub fn mirror_layout() -> Vec<(String, usize, usize)> {
    macro_rules! f {
        ($v:ident, $t:ident, $s:literal, $($f:ident)*) => {
            $v.push(($s.to_string(), std::mem::size_of::<$t>(), 0));
            $( $v.push((format!("{}.{}", $s, stringify!($f)), std::mem::offset_of!($t, $f), {
                let z: $t = unsafe { std::mem::zeroed() };
                std::mem::size_of_val(&z.$f)
            })); )*
        };
    }
    let mut v = Vec::new();
    f!(v, Theme, "goome_theme", bg panel border text text_dim accent accent_text hover active input_bg shadow radius pad gap border_width font_size shadow_blur anim_ms font skin_button skin_panel skin_input icon icon_tint skin_checkbox skin_radio skin_toggle skin_slider skin_slider_fill skin_scrollbar skin_tab skin_titlebar skin_tooltip skin_menu skin_popup focus select caret link badge toast tooltip danger knob dim check_size check_radius toggle_w toggle_h slider_h slider_track knob_size scroll_w scroll_min item_h menu_item_h tab_h win_title_h app_title_h menubar_h statusbar_h focus_w focus_gap sep_w grip indent plot_w spinner caret_w win_min_w win_min_h pop_gap menu_gap tip_pad_x tip_pad_y tip_x tip_y tip_radius shadow_x shadow_y align heading_size small_size ease hover_ms press_ms popup_ms collapse_ms toast_fade_ms font_heading font_mono font_small);
    f!(v, Config, "goome_ui_config", tip_delay_ms dbl_click_ms dbl_click_px drag_px repeat_delay_ms repeat_ms blink_ms menu_delay_ms toast_ms scroll_lines raise_on_click esc_closes tab_nav focus_on_mouse cursor keys ease ease_user mask);
    v
}

#[derive(Clone)]
pub struct Skin {
    pub bitmap: Bitmap,
    pub l: f64,
    pub t: f64,
    pub r: f64,
    pub b: f64,
}

impl Skin {
    pub fn new(bitmap: &Bitmap, l: f64, t: f64, r: f64, b: f64) -> Skin {
        Skin {
            bitmap: bitmap.clone(),
            l,
            t,
            r,
            b,
        }
    }

    pub fn even(bitmap: &Bitmap, edge: f64) -> Skin {
        Skin::new(bitmap, edge, edge, edge, edge)
    }

    fn raw(&self) -> sys::goome_skin {
        sys::goome_skin {
            bm: self.bitmap.raw(),
            l: self.l,
            t: self.t,
            r: self.r,
            b: self.b,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowState {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub open: bool,
}

impl WindowState {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> WindowState {
        WindowState { x, y, w, h, open: true }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Item {
    pub active: bool,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub hovered: bool,
    pub pressed: bool,
    pub held: bool,
    pub released: bool,
    pub clicked: bool,
    pub double_clicked: bool,
    pub right_clicked: bool,
    pub focused: bool,
    pub mx: f64,
    pub my: f64,
    pub dx: f64,
    pub dy: f64,
    pub drag_x: f64,
    pub drag_y: f64,
    pub key: i32,
}

fn num_fmt(fmt: Option<&str>) -> Option<CString> {
    let f = fmt?;
    let b = f.as_bytes();
    let (mut i, mut convs) = (0, 0);
    while i < b.len() {
        if b[i] == b'%' {
            i += 1;
            if i < b.len() && b[i] == b'%' {
                i += 1;
                continue;
            }
            while i < b.len() && b"-+ #0".contains(&b[i]) {
                i += 1;
            }
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            if i < b.len() && b[i] == b'.' {
                i += 1;
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
            }
            if i >= b.len() || !b"fFeEgG".contains(&b[i]) {
                return None;
            }
            convs += 1;
        }
        i += 1;
    }
    if convs > 1 {
        return None;
    }
    Some(cstr(f))
}

fn strs(items: &[&str]) -> (Vec<CString>, Vec<*const c_char>) {
    let owned: Vec<CString> = items.iter().map(|s| cstr(s)).collect();
    let ptrs = owned.iter().map(|c| c.as_ptr()).collect();
    (owned, ptrs)
}

fn read_buf(buf: &[u8]) -> String {
    let n = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..n]).into_owned()
}

fn fill_buf(s: &str, cap: usize) -> Vec<u8> {
    let mut n = s.len().min(cap.saturating_sub(1));
    while n > 0 && !s.is_char_boundary(n) {
        n -= 1;
    }
    let mut v = vec![0u8; cap.max(1)];
    v[..n].copy_from_slice(&s.as_bytes()[..n]);
    v
}

type Held = Box<dyn Any>;

pub struct Ui {
    u: NonNull<sys::goome_ui>,
    g: Goome,
    pins: Vec<Option<Held>>,
    spent: Vec<Held>,
    held: HashMap<u32, Held>,
    edits: HashMap<String, (Vec<u8>, u64)>,
    mask: Option<CString>,
    frames: u64,
}

impl Ui {
    pub fn new(g: Goome) -> Result<Ui> {
        let u = unsafe { sys::goome_ui_new(g.raw()) };
        match NonNull::new(u) {
            Some(u) => Ok(Ui {
                u,
                g,
                pins: Vec::new(),
                spent: Vec::new(),
                held: HashMap::new(),
                edits: HashMap::new(),
                mask: None,
                frames: 0,
            }),
            None => Err(Error::new(crate::ENOMEM, "cant make the ui")),
        }
    }

    pub fn raw(&self) -> *mut sys::goome_ui {
        self.u.as_ptr()
    }

    pub fn goome(&self) -> &Goome {
        &self.g
    }

    pub fn goome_mut(&mut self) -> &mut Goome {
        &mut self.g
    }

    fn h(&self) -> *mut sys::goome_ui {
        self.u.as_ptr()
    }

    pub fn feed(&mut self, ev: &Event) -> bool {
        let e = ev.to_raw();
        unsafe { sys::goome_ui_feed(self.h(), &e) > 0 }
    }

    pub fn pump(&mut self, timeout_ms: i32) -> Vec<Event> {
        let mut out = Vec::new();
        let mut wait = timeout_ms;
        while let Some(e) = self.g.poll(wait) {
            wait = 0;
            self.feed(&e);
            out.push(e);
        }
        out
    }

    pub fn begin(&mut self, mon: i32) -> Option<UiFrame<'_>> {
        let c = unsafe { sys::goome_ui_begin(self.h(), mon) };
        let c = NonNull::new(c)?;
        self.frames += 1;
        Some(UiFrame { ui: self, screen: c })
    }

    pub(crate) fn end_frame(&mut self) {
        unsafe { sys::goome_ui_end(self.h()) };
        self.spent.clear();
        let now = self.frames;
        self.edits.retain(|_, e| e.1 + 600 >= now);
    }

    pub fn add_hit(&mut self, x: f64, y: f64, w: f64, h: f64) {
        unsafe { sys::goome_ui_hit(self.h(), x, y, w, h) }
    }

    pub fn animating(&self) -> bool {
        unsafe { sys::goome_ui_animating(self.h()) > 0 }
    }

    pub fn wants(&self) -> i32 {
        unsafe { sys::goome_ui_wants(self.h()) }
    }

    pub fn auto_grab(&mut self, wants: i32) {
        unsafe { sys::goome_ui_auto_grab(self.h(), wants) }
    }

    fn scope(&mut self, shown: bool, end: Option<unsafe extern "C" fn(*mut sys::goome_ui)>) -> Scope<'_> {
        Scope { ui: self, end, shown }
    }

    fn maybe(&mut self, shown: bool, end: unsafe extern "C" fn(*mut sys::goome_ui)) -> Option<Scope<'_>> {
        if shown {
            Some(self.scope(true, Some(end)))
        } else {
            None
        }
    }

    pub fn window(&mut self, title: &str, win: &mut WindowState, flags: i32) -> Scope<'_> {
        let t = cstr(title);
        let mut open = win.open as c_int;
        let r = unsafe { sys::goome_ui_window(self.h(), t.as_ptr(), &mut win.x, &mut win.y, &mut win.w, &mut win.h, flags, &mut open) };
        win.open = open != 0;
        self.scope(r != 0, Some(sys::goome_ui_window_end))
    }

    pub fn row(&mut self, cols: i32) {
        unsafe { sys::goome_ui_row(self.h(), cols) }
    }

    pub fn same_line(&mut self) {
        unsafe { sys::goome_ui_same_line(self.h()) }
    }

    pub fn space(&mut self, px: f64) {
        unsafe { sys::goome_ui_space(self.h(), px) }
    }

    pub fn separator(&mut self) {
        unsafe { sys::goome_ui_separator(self.h()) }
    }

    pub fn width(&mut self, w: f64) {
        unsafe { sys::goome_ui_width(self.h(), w) }
    }

    pub fn label(&mut self, text: &str) {
        let t = cstr(text);
        unsafe { sys::goome_ui_label(self.h(), b"%s\0".as_ptr().cast(), t.as_ptr()) }
    }

    pub fn heading(&mut self, text: &str) {
        let t = cstr(text);
        unsafe { sys::goome_ui_heading(self.h(), t.as_ptr()) }
    }

    pub fn wrapped(&mut self, text: &str) {
        let t = cstr(text);
        unsafe { sys::goome_ui_wrapped(self.h(), t.as_ptr()) }
    }

    pub fn button(&mut self, label: &str) -> bool {
        let t = cstr(label);
        unsafe { sys::goome_ui_button(self.h(), t.as_ptr()) != 0 }
    }

    pub fn image_button(&mut self, id: &str, b: &Bitmap, w: f64, h: f64) -> bool {
        let t = cstr(id);
        self.spent.push(Box::new(b.clone()));
        unsafe { sys::goome_ui_image_button(self.h(), t.as_ptr(), b.raw(), w, h) != 0 }
    }

    pub fn image(&mut self, b: &Bitmap, w: f64, h: f64, radius: f64) {
        self.spent.push(Box::new(b.clone()));
        unsafe { sys::goome_ui_image(self.h(), b.raw(), w, h, radius) }
    }

    fn flag(&mut self, f: unsafe extern "C" fn(*mut sys::goome_ui, *const c_char, *mut c_int) -> c_int, label: &str, v: &mut bool) -> bool {
        let t = cstr(label);
        let mut i = *v as c_int;
        let r = unsafe { f(self.h(), t.as_ptr(), &mut i) };
        *v = i != 0;
        r != 0
    }

    pub fn checkbox(&mut self, label: &str, v: &mut bool) -> bool {
        self.flag(sys::goome_ui_checkbox, label, v)
    }

    pub fn toggle(&mut self, label: &str, v: &mut bool) -> bool {
        self.flag(sys::goome_ui_toggle, label, v)
    }

    pub fn collapsing(&mut self, label: &str, open: &mut bool) -> bool {
        self.flag(sys::goome_ui_collapsing, label, open)
    }

    pub fn radio(&mut self, label: &str, v: &mut i32, value: i32) -> bool {
        let t = cstr(label);
        unsafe { sys::goome_ui_radio(self.h(), t.as_ptr(), v, value) != 0 }
    }

    pub fn slider(&mut self, label: &str, v: &mut f64, min: f64, max: f64) -> bool {
        self.slider_fmt(label, v, min, max, None)
    }

    pub fn slider_fmt(&mut self, label: &str, v: &mut f64, min: f64, max: f64, fmt: Option<&str>) -> bool {
        let t = cstr(label);
        let f = num_fmt(fmt);
        unsafe { sys::goome_ui_slider(self.h(), t.as_ptr(), v, min, max, opt_ptr(&f)) != 0 }
    }

    pub fn slider_int(&mut self, label: &str, v: &mut i32, min: i32, max: i32) -> bool {
        let t = cstr(label);
        unsafe { sys::goome_ui_slider_int(self.h(), t.as_ptr(), v, min, max) != 0 }
    }

    pub fn input(&mut self, label: &str, text: &mut String) -> bool {
        self.input_cap(label, text, 256)
    }

    pub fn input_cap(&mut self, label: &str, text: &mut String, cap: usize) -> bool {
        let t = cstr(label);
        let mut buf = fill_buf(text, cap);
        let r = unsafe { sys::goome_ui_input(self.h(), t.as_ptr(), buf.as_mut_ptr().cast(), buf.len() as c_int) };
        if r != 0 {
            *text = read_buf(&buf);
        }
        r != 0
    }

    pub fn input_ex(&mut self, label: &str, text: &mut String, flags: i32, hint: Option<&str>) -> i32 {
        self.input_ex_cap(label, text, flags, hint, 256)
    }

    pub fn input_ex_cap(&mut self, label: &str, text: &mut String, flags: i32, hint: Option<&str>, cap: usize) -> i32 {
        let t = cstr(label);
        let hn = hint.map(cstr);
        let mut buf = fill_buf(text, cap);
        let r = unsafe { sys::goome_ui_input_ex(self.h(), t.as_ptr(), buf.as_mut_ptr().cast(), buf.len() as c_int, flags, opt_ptr(&hn)) };
        if r != 0 {
            *text = read_buf(&buf);
        }
        r
    }

    pub fn combo(&mut self, label: &str, sel: &mut i32, items: &[&str]) -> bool {
        let t = cstr(label);
        let (_keep, p) = strs(items);
        unsafe { sys::goome_ui_combo(self.h(), t.as_ptr(), sel, p.as_ptr(), p.len() as c_int) != 0 }
    }

    pub fn selectable(&mut self, label: &str, selected: bool) -> bool {
        let t = cstr(label);
        unsafe { sys::goome_ui_selectable(self.h(), t.as_ptr(), selected as c_int) != 0 }
    }

    pub fn tabs(&mut self, id: &str, names: &[&str], sel: &mut i32) -> bool {
        let t = cstr(id);
        let (_keep, p) = strs(names);
        unsafe { sys::goome_ui_tabs(self.h(), t.as_ptr(), p.as_ptr(), p.len() as c_int, sel) != 0 }
    }

    pub fn progress(&mut self, frac: f64, text: Option<&str>) {
        let t = text.map(cstr);
        unsafe { sys::goome_ui_progress(self.h(), frac, opt_ptr(&t)) }
    }

    pub fn color(&mut self, label: &str, argb: &mut u32) -> bool {
        let t = cstr(label);
        unsafe { sys::goome_ui_color(self.h(), t.as_ptr(), argb) != 0 }
    }

    pub fn tooltip(&mut self, text: &str) {
        let t = cstr(text);
        unsafe { sys::goome_ui_tooltip(self.h(), t.as_ptr()) }
    }

    pub fn hovered(&self) -> bool {
        unsafe { sys::goome_ui_hovered(self.h()) != 0 }
    }

    pub fn scroll(&mut self, id: &str, h: f64) -> Scope<'_> {
        let t = cstr(id);
        let r = unsafe { sys::goome_ui_scroll_begin(self.h(), t.as_ptr(), h) };
        self.scope(r != 0, Some(sys::goome_ui_scroll_end))
    }

    pub fn push_id(&mut self, id: &str) {
        let t = cstr(id);
        unsafe { sys::goome_ui_push_id(self.h(), t.as_ptr()) }
    }

    pub fn pop_id(&mut self) {
        unsafe { sys::goome_ui_pop_id(self.h()) }
    }

    pub fn id(&mut self, id: &str) -> Scope<'_> {
        self.push_id(id);
        self.scope(true, Some(sys::goome_ui_pop_id))
    }

    pub fn app(&mut self, title: Option<&str>, flags: i32) -> Scope<'_> {
        let t = title.map(cstr);
        let r = unsafe { sys::goome_ui_app(self.h(), opt_ptr(&t), flags) };
        self.scope(r != 0, Some(sys::goome_ui_app_end))
    }

    pub fn menubar(&mut self) -> Option<Scope<'_>> {
        let r = unsafe { sys::goome_ui_menubar_begin(self.h()) };
        self.maybe(r != 0, sys::goome_ui_menubar_end)
    }

    pub fn menu(&mut self, label: &str) -> Option<Scope<'_>> {
        let t = cstr(label);
        let r = unsafe { sys::goome_ui_menu_begin(self.h(), t.as_ptr()) };
        self.maybe(r != 0, sys::goome_ui_menu_end)
    }

    pub fn menu_item(&mut self, label: &str, shortcut: Option<&str>, checked: Option<&mut bool>) -> bool {
        let t = cstr(label);
        let s = shortcut.map(cstr);
        match checked {
            Some(c) => {
                let mut v = *c as c_int;
                let r = unsafe { sys::goome_ui_menu_item(self.h(), t.as_ptr(), opt_ptr(&s), &mut v) };
                *c = v != 0;
                r != 0
            }
            None => unsafe { sys::goome_ui_menu_item(self.h(), t.as_ptr(), opt_ptr(&s), ptr::null_mut()) != 0 },
        }
    }

    pub fn menu_separator(&mut self) {
        unsafe { sys::goome_ui_menu_separator(self.h()) }
    }

    pub fn context(&mut self, id: &str) -> Option<Scope<'_>> {
        let t = cstr(id);
        let r = unsafe { sys::goome_ui_context_begin(self.h(), t.as_ptr()) };
        self.maybe(r != 0, sys::goome_ui_context_end)
    }

    pub fn popup_open(&mut self, id: &str) {
        let t = cstr(id);
        unsafe { sys::goome_ui_popup_open(self.h(), t.as_ptr()) }
    }

    pub fn popup(&mut self, id: &str) -> Option<Scope<'_>> {
        let t = cstr(id);
        let r = unsafe { sys::goome_ui_popup_begin(self.h(), t.as_ptr()) };
        self.maybe(r != 0, sys::goome_ui_popup_end)
    }

    pub fn popup_close(&mut self) {
        unsafe { sys::goome_ui_popup_close(self.h()) }
    }

    pub fn modal(&mut self, title: &str, open: &mut bool) -> Option<Scope<'_>> {
        let t = cstr(title);
        let mut v = *open as c_int;
        let r = unsafe { sys::goome_ui_modal_begin(self.h(), t.as_ptr(), &mut v) };
        *open = v != 0;
        self.maybe(r != 0, sys::goome_ui_modal_end)
    }

    pub fn toast(&mut self, text: &str, ms: i32) {
        let t = cstr(text);
        unsafe { sys::goome_ui_toast(self.h(), t.as_ptr(), ms) }
    }

    pub fn statusbar(&mut self, text: &str) {
        let t = cstr(text);
        unsafe { sys::goome_ui_statusbar(self.h(), t.as_ptr()) }
    }

    pub fn focused(&self) -> bool {
        unsafe { sys::goome_ui_focused(self.h()) != 0 }
    }

    pub fn set_focus(&mut self) {
        unsafe { sys::goome_ui_set_focus(self.h()) }
    }

    pub fn text_edit(&mut self, id: &str, text: &mut String, h: f64, flags: i32) -> bool {
        let t = cstr(id);
        let now = self.frames;
        let need = (text.len() * 4).max(65536);
        let e = self.edits.entry(id.to_string()).or_insert_with(|| (vec![0u8; need], now));
        if e.0.len() < text.len() + 1 {
            e.0 = vec![0u8; need];
        }
        e.1 = now;
        if read_buf(&e.0) != *text {
            let cap = e.0.len();
            e.0 = fill_buf(text, cap);
        }
        let cap = e.0.len();
        let r = unsafe { sys::goome_ui_text_edit(self.u.as_ptr(), t.as_ptr(), e.0.as_mut_ptr().cast(), cap as c_int, h, flags) };
        if r != 0 {
            *text = read_buf(&e.0);
        }
        r != 0
    }

    pub fn table(&mut self, id: &str, cols: i32, h: f64, flags: i32) -> Option<Scope<'_>> {
        let t = cstr(id);
        let r = unsafe { sys::goome_ui_table_begin(self.h(), t.as_ptr(), cols, h, flags) };
        self.maybe(r != 0, sys::goome_ui_table_end)
    }

    pub fn table_column(&mut self, name: &str, width: f64) {
        let t = cstr(name);
        unsafe { sys::goome_ui_table_column(self.h(), t.as_ptr(), width) }
    }

    pub fn table_sort(&mut self, col: &mut i32, desc: &mut bool) -> bool {
        let mut d = *desc as c_int;
        let r = unsafe { sys::goome_ui_table_sort(self.h(), col, &mut d) };
        *desc = d != 0;
        r != 0
    }

    pub fn table_row(&mut self) {
        unsafe { sys::goome_ui_table_row(self.h()) }
    }

    pub fn table_next(&mut self) {
        unsafe { sys::goome_ui_table_next(self.h()) }
    }

    pub fn tree(&mut self, label: &str, open: &mut bool) -> Option<Scope<'_>> {
        let t = cstr(label);
        let mut v = *open as c_int;
        let r = unsafe { sys::goome_ui_tree(self.h(), t.as_ptr(), &mut v) };
        *open = v != 0;
        self.maybe(r != 0, sys::goome_ui_tree_end)
    }

    pub fn list_clip(&mut self, count: usize, row_h: f64) -> Range<usize> {
        let (mut first, mut last) = (0, 0);
        unsafe { sys::goome_ui_list_clip(self.h(), count as c_int, row_h, &mut first, &mut last) };
        let first = first.max(0) as usize;
        first..(last.max(0) as usize).max(first).min(count)
    }

    pub fn drag_float(&mut self, label: &str, v: &mut f64, speed: f64, min: f64, max: f64) -> bool {
        self.drag_float_fmt(label, v, speed, min, max, None)
    }

    pub fn drag_float_fmt(&mut self, label: &str, v: &mut f64, speed: f64, min: f64, max: f64, fmt: Option<&str>) -> bool {
        let t = cstr(label);
        let f = num_fmt(fmt);
        unsafe { sys::goome_ui_drag_float(self.h(), t.as_ptr(), v, speed, min, max, opt_ptr(&f)) != 0 }
    }

    pub fn drag_int(&mut self, label: &str, v: &mut i32, speed: f64, min: i32, max: i32) -> bool {
        let t = cstr(label);
        unsafe { sys::goome_ui_drag_int(self.h(), t.as_ptr(), v, speed, min, max) != 0 }
    }

    pub fn number(&mut self, label: &str, v: &mut f64, step: f64) -> bool {
        self.number_fmt(label, v, step, None)
    }

    pub fn number_fmt(&mut self, label: &str, v: &mut f64, step: f64, fmt: Option<&str>) -> bool {
        let t = cstr(label);
        let f = num_fmt(fmt);
        unsafe { sys::goome_ui_number(self.h(), t.as_ptr(), v, step, opt_ptr(&f)) != 0 }
    }

    pub fn plot_lines(&mut self, label: &str, v: &[f32], min: f64, max: f64, h: f64) {
        let t = cstr(label);
        unsafe { sys::goome_ui_plot_lines(self.h(), t.as_ptr(), v.as_ptr(), v.len() as c_int, min, max, h) }
    }

    pub fn plot_bars(&mut self, label: &str, v: &[f32], min: f64, max: f64, h: f64) {
        let t = cstr(label);
        unsafe { sys::goome_ui_plot_bars(self.h(), t.as_ptr(), v.as_ptr(), v.len() as c_int, min, max, h) }
    }

    pub fn split(&mut self, id: &str, vertical: bool, frac: &mut f64) -> Scope<'_> {
        let t = cstr(id);
        let r = unsafe { sys::goome_ui_split_begin(self.h(), t.as_ptr(), vertical as c_int, frac) };
        self.scope(r != 0, Some(sys::goome_ui_split_end))
    }

    pub fn split_next(&mut self) {
        unsafe { sys::goome_ui_split_next(self.h()) }
    }

    pub fn drag_source(&mut self, kind: &str, data: &[u8]) -> bool {
        let t = cstr(kind);
        let keep = data.to_vec();
        let r = unsafe { sys::goome_ui_drag_source(self.h(), t.as_ptr(), keep.as_ptr().cast(), keep.len() as c_int) != 0 };
        self.spent.push(Box::new(keep));
        r
    }

    pub fn drop_target(&mut self, kind: &str, cap: usize) -> Option<Vec<u8>> {
        let t = cstr(kind);
        let mut buf = vec![0u8; cap.max(1)];
        let n = unsafe { sys::goome_ui_drop_target(self.h(), t.as_ptr(), buf.as_mut_ptr().cast(), buf.len() as c_int) };
        if n <= 0 {
            return None;
        }
        buf.truncate((n as usize).min(cap));
        Some(buf)
    }

    pub fn spinner(&mut self, size: f64) {
        unsafe { sys::goome_ui_spinner(self.h(), size) }
    }

    pub fn link(&mut self, text: &str) -> bool {
        let t = cstr(text);
        unsafe { sys::goome_ui_link(self.h(), t.as_ptr()) != 0 }
    }

    pub fn badge(&mut self, text: &str, argb: u32) {
        let t = cstr(text);
        unsafe { sys::goome_ui_badge(self.h(), t.as_ptr(), argb) }
    }

    pub fn pos(&mut self, x: f64, y: f64) {
        unsafe { sys::goome_ui_pos(self.h(), x, y) }
    }

    pub fn size(&mut self, w: f64, h: f64) {
        unsafe { sys::goome_ui_size(self.h(), w, h) }
    }

    pub fn push_color(&mut self, what: i32, argb: u32) {
        unsafe { sys::goome_ui_push_color(self.h(), what, argb) };
        self.pins.push(None);
    }

    pub fn push_num(&mut self, what: i32, v: f64) {
        unsafe { sys::goome_ui_push_num(self.h(), what, v) };
        self.pins.push(None);
    }

    pub fn push_radii(&mut self, r: impl Into<crate::Radii>) {
        unsafe { sys::goome_ui_push_radii(self.h(), r.into().raw()) };
        self.pins.push(None);
    }

    pub fn push_font(&mut self, f: Option<&Font>) {
        unsafe { sys::goome_ui_push_font(self.h(), f.map_or(ptr::null_mut(), |f| f.raw())) };
        self.pins.push(f.map(|f| Box::new(f.clone()) as Held));
    }

    pub fn push_skin(&mut self, kind: i32, s: Option<&Skin>) {
        if !(0..SKIN_COUNT).contains(&kind) {
            return;
        }
        match s {
            Some(s) => {
                let raw = Box::new(s.raw());
                unsafe { sys::goome_ui_push_skin(self.h(), kind, &*raw) };
                self.pins.push(Some(Box::new((raw, s.bitmap.clone()))));
            }
            None => {
                unsafe { sys::goome_ui_push_skin(self.h(), kind, ptr::null()) };
                self.pins.push(None);
            }
        }
    }

    pub fn push_theme(&mut self, t: &Theme) {
        let b = Box::new(*t);
        unsafe { sys::goome_ui_push_theme(self.h(), (&*b as *const Theme).cast()) };
        self.pins.push(Some(b));
    }

    pub fn push_icon(&mut self, icon: i32, b: Option<&Bitmap>) {
        if !(0..ICON_COUNT).contains(&icon) {
            return;
        }
        unsafe { sys::goome_ui_push_icon(self.h(), icon, b.map_or(ptr::null(), |b| b.raw())) };
        self.pins.push(b.map(|b| Box::new(b.clone()) as Held));
    }

    pub fn pop(&mut self, n: usize) {
        unsafe { sys::goome_ui_pop(self.h(), n as c_int) };
        let cut = self.pins.len().saturating_sub(n);
        let gone: Vec<_> = self.pins.drain(cut..).flatten().collect();
        self.spent.extend(gone);
    }

    pub fn style(&mut self) -> Style<'_> {
        Style { ui: self, n: 0 }
    }

    pub fn custom(&mut self, id: &str, w: f64, h: f64, flags: i32) -> Item {
        let t = cstr(id);
        let mut it = sys::goome_ui_item::default();
        let r = unsafe { sys::goome_ui_custom(self.h(), t.as_ptr(), w, h, flags, &mut it) };
        Item {
            active: r != 0,
            x: it.x,
            y: it.y,
            w: it.w,
            h: it.h,
            hovered: it.hovered != 0,
            pressed: it.pressed != 0,
            held: it.held != 0,
            released: it.released != 0,
            clicked: it.clicked != 0,
            double_clicked: it.double_clicked != 0,
            right_clicked: it.right_clicked != 0,
            focused: it.focused != 0,
            mx: it.mx,
            my: it.my,
            dx: it.dx,
            dy: it.dy,
            drag_x: it.drag_x,
            drag_y: it.drag_y,
            key: it.key,
        }
    }

    pub fn canvas(&mut self) -> Option<UiCanvas<'_>> {
        NonNull::new(unsafe { sys::goome_ui_canvas(self.h()) }).map(UiCanvas::new)
    }

    pub fn store(&mut self, id: &str, n: usize) -> &mut [f64] {
        if n == 0 {
            return &mut [];
        }
        let t = cstr(id);
        let p = unsafe { sys::goome_ui_store(self.h(), t.as_ptr(), n as c_int) };
        if p.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(p, n) }
    }

    pub fn theme(&mut self) -> &mut Theme {
        unsafe { &mut *(sys::goome_ui_theme(self.h()) as *mut Theme) }
    }

    fn hold(&mut self, slot: u32, v: Option<Held>) {
        if let Some(old) = self.held.remove(&slot) {
            self.spent.push(old);
        }
        if let Some(v) = v {
            self.held.insert(slot, v);
        }
    }

    pub fn set_font(&mut self, f: Option<&Font>) {
        self.theme().font = f.map_or(ptr::null_mut(), |f| f.raw());
        self.hold(0, f.map(|f| Box::new(f.clone()) as Held));
    }

    pub fn set_heading_font(&mut self, f: Option<&Font>) {
        self.theme().font_heading = f.map_or(ptr::null_mut(), |f| f.raw());
        self.hold(1, f.map(|f| Box::new(f.clone()) as Held));
    }

    pub fn set_mono_font(&mut self, f: Option<&Font>) {
        self.theme().font_mono = f.map_or(ptr::null_mut(), |f| f.raw());
        self.hold(2, f.map(|f| Box::new(f.clone()) as Held));
    }

    pub fn set_small_font(&mut self, f: Option<&Font>) {
        self.theme().font_small = f.map_or(ptr::null_mut(), |f| f.raw());
        self.hold(3, f.map(|f| Box::new(f.clone()) as Held));
    }

    pub fn set_icon(&mut self, icon: i32, b: Option<&Bitmap>) {
        if !(0..ICON_COUNT).contains(&icon) {
            return;
        }
        self.theme().icon[icon as usize] = b.map_or(ptr::null(), |b| b.raw());
        self.hold(100 + icon as u32, b.map(|b| Box::new(b.clone()) as Held));
    }

    pub fn set_skin(&mut self, kind: i32, s: Option<&Skin>) {
        let raw = s.map_or(
            sys::goome_skin {
                bm: ptr::null(),
                l: 0.0,
                t: 0.0,
                r: 0.0,
                b: 0.0,
            },
            |s| s.raw(),
        );
        let t = self.theme();
        let slot = match kind {
            crate::SKIN_BUTTON => &mut t.skin_button,
            crate::SKIN_PANEL => &mut t.skin_panel,
            crate::SKIN_INPUT => &mut t.skin_input,
            crate::SKIN_CHECKBOX => &mut t.skin_checkbox,
            crate::SKIN_RADIO => &mut t.skin_radio,
            crate::SKIN_TOGGLE => &mut t.skin_toggle,
            crate::SKIN_SLIDER_TRACK => &mut t.skin_slider,
            crate::SKIN_SLIDER_FILL => &mut t.skin_slider_fill,
            crate::SKIN_SCROLLBAR => &mut t.skin_scrollbar,
            crate::SKIN_TAB => &mut t.skin_tab,
            crate::SKIN_TITLEBAR => &mut t.skin_titlebar,
            crate::SKIN_TOOLTIP => &mut t.skin_tooltip,
            crate::SKIN_MENU => &mut t.skin_menu,
            crate::SKIN_POPUP => &mut t.skin_popup,
            _ => return,
        };
        *slot = raw;
        self.hold(200 + kind as u32, s.map(|s| Box::new(s.bitmap.clone()) as Held));
    }

    pub fn set_theme(&mut self, t: &Theme) {
        let mut copy = *t;
        let cur = *self.theme();
        copy.font = cur.font;
        copy.font_heading = cur.font_heading;
        copy.font_mono = cur.font_mono;
        copy.font_small = cur.font_small;
        copy.icon = cur.icon;
        copy.skin_button = cur.skin_button;
        copy.skin_panel = cur.skin_panel;
        copy.skin_input = cur.skin_input;
        copy.skin_checkbox = cur.skin_checkbox;
        copy.skin_radio = cur.skin_radio;
        copy.skin_toggle = cur.skin_toggle;
        copy.skin_slider = cur.skin_slider;
        copy.skin_slider_fill = cur.skin_slider_fill;
        copy.skin_scrollbar = cur.skin_scrollbar;
        copy.skin_tab = cur.skin_tab;
        copy.skin_titlebar = cur.skin_titlebar;
        copy.skin_tooltip = cur.skin_tooltip;
        copy.skin_menu = cur.skin_menu;
        copy.skin_popup = cur.skin_popup;
        *self.theme() = copy;
    }

    pub fn scale(&mut self, s: f64) {
        unsafe { sys::goome_ui_scale(self.h(), s) }
    }

    pub fn config(&mut self) -> &mut Config {
        unsafe { &mut *(sys::goome_ui_cfg(self.h()) as *mut Config) }
    }

    pub fn set_mask(&mut self, mask: Option<&str>) {
        let m = mask.map(cstr);
        self.config().mask = opt_ptr(&m);
        if let Some(old) = std::mem::replace(&mut self.mask, m) {
            self.spent.push(Box::new(old));
        }
    }

    pub fn bind(&mut self, act: i32, combos: &str) -> Result<()> {
        let t = cstr(combos);
        let r = unsafe { sys::goome_ui_bind(self.h(), act, t.as_ptr()) };
        if r != OK {
            return Err(Error::new(r, format!("cant bind {:?}", combos)));
        }
        Ok(())
    }

    pub fn anim(&mut self, id: &str, target: f64, ms: f64, ease: i32) -> f64 {
        let t = cstr(id);
        unsafe { sys::goome_ui_anim(self.h(), t.as_ptr(), target, ms, ease) }
    }

    pub fn anim_color(&mut self, id: &str, target: u32, ms: f64, ease: i32) -> u32 {
        let t = cstr(id);
        unsafe { sys::goome_ui_anim_color(self.h(), t.as_ptr(), target, ms, ease) }
    }

    pub fn a11y(&mut self, on: bool) {
        unsafe { sys::goome_ui_a11y(self.h(), on as c_int) }
    }

    pub fn a11y_role(&mut self, role: i32, label: &str) {
        let t = cstr(label);
        unsafe { sys::goome_ui_a11y_role(self.h(), role, t.as_ptr()) }
    }

    pub fn a11y_value(&mut self, value: &str) {
        let t = cstr(value);
        unsafe { sys::goome_ui_a11y_value(self.h(), t.as_ptr()) }
    }

    pub fn a11y_tree(&self) -> Vec<A11yNode> {
        let n = unsafe { sys::goome_a11y_tree(self.h(), ptr::null_mut(), 0) };
        if n <= 0 {
            return Vec::new();
        }
        let mut raw: Vec<sys::goome_a11y_node> = vec![unsafe { std::mem::zeroed() }; n as usize];
        let got = unsafe { sys::goome_a11y_tree(self.h(), raw.as_mut_ptr(), n) }.clamp(0, n) as usize;
        raw.truncate(got);
        raw.iter().map(A11yNode::from_raw).collect()
    }
}

pub fn ease(kind: i32, t: f64) -> f64 {
    unsafe { sys::goome_ease(kind, t) }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct A11yNode {
    pub role: i32,
    pub parent: Option<usize>,
    pub states: i32,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub label: String,
    pub value: String,
    pub id: u32,
    pub num: f64,
    pub min: f64,
    pub max: f64,
    pub caret: i32,
}

impl A11yNode {
    pub fn from_raw(n: &sys::goome_a11y_node) -> A11yNode {
        A11yNode {
            role: n.role,
            parent: if n.parent < 0 { None } else { Some(n.parent as usize) },
            states: n.states,
            x: n.x,
            y: n.y,
            w: n.w,
            h: n.h,
            label: crate::from_chars(&n.label),
            value: crate::from_chars(&n.value),
            id: n.id,
            num: n.num,
            min: n.min,
            max: n.max,
            caret: n.caret,
        }
    }

    pub fn focused(&self) -> bool {
        self.states & crate::A11Y_FOCUSED != 0
    }

    pub fn checked(&self) -> bool {
        self.states & crate::A11Y_CHECKED != 0
    }

    pub fn disabled(&self) -> bool {
        self.states & crate::A11Y_DISABLED != 0
    }

    pub fn expanded(&self) -> bool {
        self.states & crate::A11Y_EXPANDED != 0
    }

    pub fn selected(&self) -> bool {
        self.states & crate::A11Y_SELECTED != 0
    }
}

impl Deref for Ui {
    type Target = Goome;

    fn deref(&self) -> &Goome {
        &self.g
    }
}

impl DerefMut for Ui {
    fn deref_mut(&mut self) -> &mut Goome {
        &mut self.g
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        unsafe { sys::goome_ui_free(self.u.as_ptr()) }
    }
}

pub struct UiFrame<'a> {
    ui: &'a mut Ui,
    screen: NonNull<sys::goome_canvas>,
}

impl UiFrame<'_> {
    pub fn screen(&mut self) -> UiCanvas<'_> {
        UiCanvas::new(self.screen)
    }
}

impl Deref for UiFrame<'_> {
    type Target = Ui;

    fn deref(&self) -> &Ui {
        self.ui
    }
}

impl DerefMut for UiFrame<'_> {
    fn deref_mut(&mut self) -> &mut Ui {
        self.ui
    }
}

impl Drop for UiFrame<'_> {
    fn drop(&mut self) {
        self.ui.end_frame();
    }
}

pub struct Scope<'a> {
    ui: &'a mut Ui,
    end: Option<unsafe extern "C" fn(*mut sys::goome_ui)>,
    shown: bool,
}

impl Scope<'_> {
    pub fn shown(&self) -> bool {
        self.shown
    }
}

impl Deref for Scope<'_> {
    type Target = Ui;

    fn deref(&self) -> &Ui {
        self.ui
    }
}

impl DerefMut for Scope<'_> {
    fn deref_mut(&mut self) -> &mut Ui {
        self.ui
    }
}

impl Drop for Scope<'_> {
    fn drop(&mut self) {
        if let Some(end) = self.end.take() {
            unsafe { end(self.ui.u.as_ptr()) }
        }
    }
}

pub struct Style<'a> {
    ui: &'a mut Ui,
    n: usize,
}

impl Style<'_> {
    pub fn color(&mut self, what: i32, argb: u32) -> &mut Self {
        self.ui.push_color(what, argb);
        self.n += 1;
        self
    }

    pub fn num(&mut self, what: i32, v: f64) -> &mut Self {
        self.ui.push_num(what, v);
        self.n += 1;
        self
    }

    pub fn radii(&mut self, r: impl Into<crate::Radii>) -> &mut Self {
        self.ui.push_radii(r);
        self.n += 1;
        self
    }

    pub fn font(&mut self, f: Option<&Font>) -> &mut Self {
        self.ui.push_font(f);
        self.n += 1;
        self
    }

    pub fn skin(&mut self, kind: i32, s: Option<&Skin>) -> &mut Self {
        let before = self.ui.pins.len();
        self.ui.push_skin(kind, s);
        self.n += self.ui.pins.len() - before;
        self
    }

    pub fn theme(&mut self, t: &Theme) -> &mut Self {
        self.ui.push_theme(t);
        self.n += 1;
        self
    }

    pub fn icon(&mut self, icon: i32, b: Option<&Bitmap>) -> &mut Self {
        let before = self.ui.pins.len();
        self.ui.push_icon(icon, b);
        self.n += self.ui.pins.len() - before;
        self
    }
}

impl Deref for Style<'_> {
    type Target = Ui;

    fn deref(&self) -> &Ui {
        self.ui
    }
}

impl DerefMut for Style<'_> {
    fn deref_mut(&mut self) -> &mut Ui {
        self.ui
    }
}

impl Drop for Style<'_> {
    fn drop(&mut self) {
        if self.n > 0 {
            self.ui.pop(self.n);
        }
    }
}
