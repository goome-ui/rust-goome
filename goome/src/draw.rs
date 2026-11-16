use crate::{check, claim, cstr, from_c, sys, Claim, Error, Goome, Rect, Result, EARG};
use std::ffi::c_int;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};
use std::rc::Rc;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Radii {
    pub tl: f64,
    pub tr: f64,
    pub br: f64,
    pub bl: f64,
}

impl Radii {
    pub fn all(r: f64) -> Radii {
        Radii { tl: r, tr: r, br: r, bl: r }
    }

    pub fn new(tl: f64, tr: f64, br: f64, bl: f64) -> Radii {
        Radii { tl, tr, br, bl }
    }

    pub(crate) fn raw(self) -> sys::goome_radii {
        sys::goome_radii {
            tl: self.tl,
            tr: self.tr,
            br: self.br,
            bl: self.bl,
        }
    }
}

impl From<f64> for Radii {
    fn from(r: f64) -> Radii {
        Radii::all(r)
    }
}

impl From<i32> for Radii {
    fn from(r: i32) -> Radii {
        Radii::all(r as f64)
    }
}

impl From<(f64, f64, f64, f64)> for Radii {
    fn from(r: (f64, f64, f64, f64)) -> Radii {
        Radii::new(r.0, r.1, r.2, r.3)
    }
}

impl From<[f64; 4]> for Radii {
    fn from(r: [f64; 4]) -> Radii {
        Radii::new(r[0], r[1], r[2], r[3])
    }
}

pub(crate) struct BitmapInner {
    pub(crate) raw: sys::goome_bitmap,
    _keep: Option<Vec<u32>>,
    _claim: Claim,
}

impl Drop for BitmapInner {
    fn drop(&mut self) {
        if self.raw.owned != 0 {
            unsafe { sys::goome_bitmap_free(&mut self.raw) }
        }
    }
}

#[derive(Clone)]
pub struct Bitmap(pub(crate) Rc<BitmapInner>);

impl Bitmap {
    fn take(r: c_int, raw: sys::goome_bitmap, c: Claim) -> Result<Bitmap> {
        if r != crate::OK {
            let why = unsafe { from_c(sys::goome_bitmap_error()) }.unwrap_or_default();
            return Err(Error::new(r, if why.is_empty() { crate::strerror(r) } else { why }));
        }
        Ok(Bitmap(Rc::new(BitmapInner { raw, _keep: None, _claim: c })))
    }

    fn empty() -> sys::goome_bitmap {
        sys::goome_bitmap {
            px: ptr::null_mut(),
            w: 0,
            h: 0,
            stride: 0,
            owned: 0,
        }
    }

    pub fn load(path: &str) -> Result<Bitmap> {
        let p = cstr(path);
        let mut raw = Bitmap::empty();
        let c = claim()?;
        let r = unsafe { sys::goome_bitmap_load(p.as_ptr(), &mut raw) };
        Bitmap::take(r, raw, c)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Bitmap> {
        let mut raw = Bitmap::empty();
        let c = claim()?;
        let r = unsafe { sys::goome_bitmap_load_mem(data.as_ptr().cast(), data.len(), &mut raw) };
        Bitmap::take(r, raw, c)
    }

    pub fn from_argb(argb: &[u32], w: usize, h: usize) -> Result<Bitmap> {
        if argb.len() < w * h {
            return Err(Error::new(EARG, "not enough pixels for that size"));
        }
        let mut raw = Bitmap::empty();
        let c = claim()?;
        let r = unsafe { sys::goome_bitmap_from_argb(argb.as_ptr(), w as c_int, h as c_int, &mut raw) };
        Bitmap::take(r, raw, c)
    }

    pub fn from_premul(px: Vec<u32>, w: usize, h: usize, stride: usize) -> Result<Bitmap> {
        if w > stride || (h > 0 && px.len() < (h - 1) * stride + w) {
            return Err(Error::new(EARG, "not enough pixels for that size"));
        }
        let mut px = px;
        let raw = sys::goome_bitmap {
            px: px.as_mut_ptr(),
            w: w as c_int,
            h: h as c_int,
            stride: stride as c_int,
            owned: 0,
        };
        Ok(Bitmap(Rc::new(BitmapInner {
            raw,
            _keep: Some(px),
            _claim: claim()?,
        })))
    }

    pub fn w(&self) -> usize {
        self.0.raw.w as usize
    }

    pub fn h(&self) -> usize {
        self.0.raw.h as usize
    }

    pub fn stride(&self) -> usize {
        self.0.raw.stride as usize
    }

    pub fn pixels(&self) -> &[u32] {
        let r = &self.0.raw;
        if r.px.is_null() || r.h <= 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(r.px, (r.h as usize - 1) * r.stride as usize + r.w as usize) }
    }

    pub fn raw(&self) -> *const sys::goome_bitmap {
        &self.0.raw
    }
}

pub(crate) struct FontInner {
    pub(crate) f: NonNull<sys::goome_font>,
    _keep: Option<Vec<u8>>,
    _claim: Claim,
}

impl Drop for FontInner {
    fn drop(&mut self) {
        unsafe { sys::goome_font_free(self.f.as_ptr()) }
    }
}

#[derive(Clone)]
pub struct Font(pub(crate) Rc<FontInner>);

impl Font {
    fn wrap(f: *mut sys::goome_font, keep: Option<Vec<u8>>, what: &str, c: Claim) -> Result<Font> {
        match NonNull::new(f) {
            Some(f) => Ok(Font(Rc::new(FontInner { f, _keep: keep, _claim: c }))),
            None => Err(Error::new(crate::EARG, format!("cant load the font {}", what))),
        }
    }

    pub fn load(path: &str) -> Result<Font> {
        let p = cstr(path);
        let c = claim()?;
        Font::wrap(unsafe { sys::goome_font_load(p.as_ptr()) }, None, path, c)
    }

    pub fn load_index(path: &str, index: i32) -> Result<Font> {
        let p = cstr(path);
        let c = claim()?;
        Font::wrap(unsafe { sys::goome_font_load_index(p.as_ptr(), index) }, None, path, c)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Font> {
        let c = claim()?;
        let f = unsafe { sys::goome_font_load_mem(data.as_ptr().cast(), data.len(), 0) };
        Font::wrap(f, Some(data), "from memory", c)
    }

    pub fn system(name: Option<&str>) -> Result<Font> {
        let n = name.map(cstr);
        let c = claim()?;
        let f = unsafe { sys::goome_font_system(n.as_ref().map_or(ptr::null(), |c| c.as_ptr())) };
        Font::wrap(f, None, name.unwrap_or("for the ui"), c)
    }

    pub fn measure(&self, size: f64, text: &str) -> (f64, f64) {
        let t = cstr(text);
        let mut h = 0.0;
        let w = unsafe { sys::goome_text_measure(self.raw(), size, t.as_ptr(), &mut h) };
        (w, h)
    }

    pub fn line_height(&self, size: f64) -> f64 {
        unsafe { sys::goome_text_line_height(self.raw(), size) }
    }

    pub fn raw(&self) -> *mut sys::goome_font {
        self.0.f.as_ptr()
    }
}

pub fn builtin_measure(size: f64, text: &str) -> (f64, f64) {
    let _c = claim().expect("goome is in use on another thread");
    let t = cstr(text);
    let mut h = 0.0;
    let w = unsafe { sys::goome_text_measure(ptr::null_mut(), size, t.as_ptr(), &mut h) };
    (w, h)
}

pub fn set_builtin_font(font: i32) {
    let _c = claim().expect("goome is in use on another thread");
    unsafe { sys::goome_set_font(font) }
}

pub fn builtin_font_count() -> i32 {
    unsafe { sys::goome_font_count() }
}

pub trait Draw {
    fn canvas_ptr(&mut self) -> *mut sys::goome_canvas;

    fn info(&mut self) -> &sys::goome_canvas {
        unsafe { &*self.canvas_ptr() }
    }

    fn w(&mut self) -> usize {
        self.info().w as usize
    }

    fn h(&mut self) -> usize {
        self.info().h as usize
    }

    fn scale(&mut self) -> f64 {
        self.info().scale
    }

    fn clear(&mut self) {
        unsafe { sys::goome_clear(self.canvas_ptr()) }
    }

    fn rect(&mut self, x: f64, y: f64, w: f64, h: f64, argb: u32) {
        unsafe { sys::goome_draw_rect(self.canvas_ptr(), x, y, w, h, argb) }
    }

    fn round_rect(&mut self, x: f64, y: f64, w: f64, h: f64, radius: f64, argb: u32) {
        unsafe { sys::goome_draw_round_rect(self.canvas_ptr(), x, y, w, h, radius, argb) }
    }

    fn frame(&mut self, x: f64, y: f64, w: f64, h: f64, width: f64, argb: u32) {
        unsafe { sys::goome_draw_frame(self.canvas_ptr(), x, y, w, h, width, argb) }
    }

    fn line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, width: f64, argb: u32) {
        unsafe { sys::goome_draw_line(self.canvas_ptr(), x0, y0, x1, y1, width, argb) }
    }

    fn triangle(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, argb: u32) {
        unsafe { sys::goome_draw_triangle(self.canvas_ptr(), x0, y0, x1, y1, x2, y2, argb) }
    }

    fn image(&mut self, x: f64, y: f64, w: f64, h: f64, px: &[u32], iw: usize, ih: usize) {
        if px.len() < iw * ih {
            return;
        }
        unsafe { sys::goome_draw_image(self.canvas_ptr(), x, y, w, h, px.as_ptr(), iw as c_int, ih as c_int) }
    }

    fn erase(&mut self, x: f64, y: f64, w: f64, h: f64) {
        unsafe { sys::goome_erase(self.canvas_ptr(), x, y, w, h) }
    }

    fn text(&mut self, x: f64, y: f64, s: &str, argb: u32) -> f64 {
        let t = cstr(s);
        unsafe { sys::goome_draw_text(self.canvas_ptr(), x, y, t.as_ptr(), 0, argb) }
    }

    fn text_big(&mut self, x: f64, y: f64, s: &str, argb: u32) -> f64 {
        let t = cstr(s);
        unsafe { sys::goome_draw_text(self.canvas_ptr(), x, y, t.as_ptr(), 1, argb) }
    }

    fn text_width(&mut self, s: &str, big: bool) -> f64 {
        let t = cstr(s);
        unsafe { sys::goome_text_width(self.canvas_ptr(), t.as_ptr(), big as c_int) }
    }

    fn font_text(&mut self, font: Option<&Font>, size: f64, x: f64, y: f64, s: &str, argb: u32) -> f64 {
        let t = cstr(s);
        let f = font.map_or(ptr::null_mut(), |f| f.raw());
        unsafe { sys::goome_text(self.canvas_ptr(), f, size, x, y, t.as_ptr(), argb) }
    }

    fn blit(&mut self, src: &mut dyn Draw, alpha: f64, dx: f64, dy: f64) {
        let s = src.canvas_ptr();
        unsafe { sys::goome_canvas_blit(self.canvas_ptr(), s, alpha, dx, dy) }
    }

    fn damage(&mut self, r: Rect) {
        let rr = r.raw();
        unsafe { sys::goome_damage(self.canvas_ptr(), &rr) }
    }

    fn clip_push(&mut self, x: f64, y: f64, w: f64, h: f64) {
        unsafe { sys::goome_clip_push(self.canvas_ptr(), x, y, w, h) }
    }

    fn clip_pop(&mut self) {
        unsafe { sys::goome_clip_pop(self.canvas_ptr()) }
    }

    fn rrect(&mut self, x: f64, y: f64, w: f64, h: f64, r: impl Into<Radii>, argb: u32)
    where
        Self: Sized,
    {
        unsafe { sys::goome_draw_rrect(self.canvas_ptr(), x, y, w, h, r.into().raw(), argb) }
    }

    fn rrect_line(&mut self, x: f64, y: f64, w: f64, h: f64, r: impl Into<Radii>, width: f64, argb: u32)
    where
        Self: Sized,
    {
        unsafe { sys::goome_draw_rrect_line(self.canvas_ptr(), x, y, w, h, r.into().raw(), width, argb) }
    }

    #[allow(clippy::too_many_arguments)]
    fn rrect_grad(&mut self, x: f64, y: f64, w: f64, h: f64, r: impl Into<Radii>, from: u32, to: u32, sideways: bool)
    where
        Self: Sized,
    {
        unsafe { sys::goome_draw_rrect_grad(self.canvas_ptr(), x, y, w, h, r.into().raw(), from, to, sideways as c_int) }
    }

    #[allow(clippy::too_many_arguments)]
    fn shadow(&mut self, x: f64, y: f64, w: f64, h: f64, radius: f64, blur: f64, argb: u32) {
        unsafe { sys::goome_draw_shadow(self.canvas_ptr(), x, y, w, h, radius, blur, argb) }
    }

    fn circle(&mut self, cx: f64, cy: f64, r: f64, argb: u32) {
        unsafe { sys::goome_draw_circle(self.canvas_ptr(), cx, cy, r, argb) }
    }

    fn ring(&mut self, cx: f64, cy: f64, r: f64, width: f64, argb: u32) {
        unsafe { sys::goome_draw_ring(self.canvas_ptr(), cx, cy, r, width, argb) }
    }

    #[allow(clippy::too_many_arguments)]
    fn arc(&mut self, cx: f64, cy: f64, r: f64, width: f64, a0: f64, a1: f64, argb: u32) {
        unsafe { sys::goome_draw_arc(self.canvas_ptr(), cx, cy, r, width, a0, a1, argb) }
    }

    fn poly(&mut self, points: &[(f64, f64)], argb: u32) {
        let xy: Vec<f64> = points.iter().flat_map(|p| [p.0, p.1]).collect();
        unsafe { sys::goome_draw_poly(self.canvas_ptr(), xy.as_ptr(), points.len() as c_int, argb) }
    }

    fn poly_flat(&mut self, xy: &[f64], argb: u32) {
        unsafe { sys::goome_draw_poly(self.canvas_ptr(), xy.as_ptr(), (xy.len() / 2) as c_int, argb) }
    }

    #[allow(clippy::too_many_arguments)]
    fn bitmap(&mut self, b: &Bitmap, x: f64, y: f64, w: f64, h: f64, r: impl Into<Radii>, tint: u32)
    where
        Self: Sized,
    {
        unsafe { sys::goome_draw_bitmap(self.canvas_ptr(), b.raw(), x, y, w, h, r.into().raw(), tint) }
    }

    #[allow(clippy::too_many_arguments)]
    fn bitmap_part(&mut self, b: &Bitmap, sx: i32, sy: i32, sw: i32, sh: i32, x: f64, y: f64, w: f64, h: f64, tint: u32) {
        unsafe { sys::goome_draw_bitmap_part(self.canvas_ptr(), b.raw(), sx, sy, sw, sh, x, y, w, h, tint) }
    }

    #[allow(clippy::too_many_arguments)]
    fn bitmap_nine(&mut self, b: &Bitmap, x: f64, y: f64, w: f64, h: f64, l: f64, t: f64, r: f64, bm: f64) {
        unsafe { sys::goome_draw_bitmap_nine(self.canvas_ptr(), b.raw(), x, y, w, h, l, t, r, bm) }
    }
}

fn pixels_of<'a>(c: &sys::goome_canvas) -> &'a [u32] {
    if c.px.is_null() || c.h <= 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(c.px, c.h as usize * c.stride as usize) }
}

fn pixels_of_mut<'a>(c: &sys::goome_canvas) -> &'a mut [u32] {
    if c.px.is_null() || c.h <= 0 {
        return &mut [];
    }
    unsafe { std::slice::from_raw_parts_mut(c.px, c.h as usize * c.stride as usize) }
}

pub struct Canvas {
    raw: Box<sys::goome_canvas>,
    buf: Vec<u32>,
    _claim: Claim,
}

impl Canvas {
    pub fn new(w: usize, h: usize, stride: usize, scale: f64) -> Result<Canvas> {
        if w > i32::MAX as usize / 4 || h > i32::MAX as usize / 4 || stride > i32::MAX as usize / 4 {
            return Err(Error::new(EARG, "thats way too big for a canvas"));
        }
        let c = claim()?;
        let stride = stride.max(w);
        let mut buf = vec![0u32; (h * stride).max(1)];
        let mut raw: Box<sys::goome_canvas> = Box::new(unsafe { std::mem::zeroed() });
        raw.px = buf.as_mut_ptr();
        raw.w = w as c_int;
        raw.h = h as c_int;
        raw.stride = stride as c_int;
        raw.scale = if scale > 0.0 { scale } else { 1.0 };
        Ok(Canvas { raw, buf, _claim: c })
    }

    pub fn with_origin(mut self, ox: f64, oy: f64) -> Canvas {
        self.raw.ox = ox;
        self.raw.oy = oy;
        self
    }

    pub fn pixels(&self) -> &[u32] {
        &self.buf[..self.raw.h as usize * self.raw.stride as usize]
    }

    pub fn pixels_mut(&mut self) -> &mut [u32] {
        let n = self.raw.h as usize * self.raw.stride as usize;
        &mut self.buf[..n]
    }

    pub fn stride(&self) -> usize {
        self.raw.stride as usize
    }

    pub fn raw(&self) -> &sys::goome_canvas {
        &self.raw
    }
}

impl Draw for Canvas {
    fn canvas_ptr(&mut self) -> *mut sys::goome_canvas {
        &mut *self.raw
    }
}

pub struct Frame<'a> {
    g: &'a mut Goome,
    c: NonNull<sys::goome_canvas>,
    hits: Vec<sys::goome_rect>,
    done: bool,
}

impl<'a> Frame<'a> {
    pub(crate) fn new(g: &'a mut Goome, c: NonNull<sys::goome_canvas>) -> Frame<'a> {
        Frame {
            g,
            c,
            hits: Vec::new(),
            done: false,
        }
    }

    pub fn hit(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.hits.push(sys::goome_rect { x, y, w, h });
    }

    pub fn hit_rect(&mut self, r: Rect) {
        self.hits.push(r.raw());
    }

    pub fn pixels(&self) -> &[u32] {
        pixels_of(unsafe { self.c.as_ref() })
    }

    pub fn pixels_mut(&mut self) -> &mut [u32] {
        pixels_of_mut(unsafe { self.c.as_ref() })
    }

    pub fn stride(&self) -> usize {
        unsafe { self.c.as_ref().stride as usize }
    }

    pub fn raw(&self) -> *mut sys::goome_canvas {
        self.c.as_ptr()
    }

    fn send(&mut self) -> c_int {
        self.done = true;
        let p = if self.hits.is_empty() { ptr::null() } else { self.hits.as_ptr() };
        unsafe { sys::goome_end(self.g.raw(), self.c.as_ptr(), p, self.hits.len() as c_int) }
    }

    pub fn end(mut self) -> Result<()> {
        let r = self.send();
        check(r)
    }
}

impl Draw for Frame<'_> {
    fn canvas_ptr(&mut self) -> *mut sys::goome_canvas {
        self.c.as_ptr()
    }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        if !self.done {
            self.send();
        }
    }
}

pub struct UiCanvas<'a> {
    c: NonNull<sys::goome_canvas>,
    _ui: PhantomData<&'a mut ()>,
}

impl<'a> UiCanvas<'a> {
    pub(crate) fn new(c: NonNull<sys::goome_canvas>) -> UiCanvas<'a> {
        UiCanvas { c, _ui: PhantomData }
    }

    pub fn pixels(&self) -> &[u32] {
        pixels_of(unsafe { self.c.as_ref() })
    }

    pub fn stride(&self) -> usize {
        unsafe { self.c.as_ref().stride as usize }
    }

    pub fn raw(&self) -> *mut sys::goome_canvas {
        self.c.as_ptr()
    }
}

impl Draw for UiCanvas<'_> {
    fn canvas_ptr(&mut self) -> *mut sys::goome_canvas {
        self.c.as_ptr()
    }
}
