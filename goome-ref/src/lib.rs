use std::ffi::{c_char, c_int, CStr};

extern crate goome_sys;

extern "C" {
    fn ref_layout(out: *mut c_char, cap: c_int) -> c_int;
    fn ref_const_count() -> c_int;
    fn ref_const_name(i: c_int) -> *const c_char;
    fn ref_const_value(i: c_int) -> c_int;
    fn ref_fnv(px: *const u32, w: c_int, h: c_int, stride: c_int) -> u32;
    fn ref_basic() -> u32;
    fn ref_shape(s: c_int, k: c_int) -> u32;
    fn ref_frame(out: *mut c_char, cap: c_int) -> c_int;
    fn ref_style() -> u32;
}

fn text(f: unsafe extern "C" fn(*mut c_char, c_int) -> c_int) -> String {
    let mut buf = vec![0 as c_char; 1 << 16];
    let n = unsafe { f(buf.as_mut_ptr(), buf.len() as c_int) };
    assert!(n >= 0);
    unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() }
}

pub fn layout() -> String {
    text(ref_layout)
}

pub fn consts() -> Vec<(String, i32)> {
    (0..unsafe { ref_const_count() })
        .map(|i| unsafe { (CStr::from_ptr(ref_const_name(i)).to_string_lossy().into_owned(), ref_const_value(i)) })
        .collect()
}

pub fn fnv(px: &[u32], w: usize, h: usize, stride: usize) -> u32 {
    assert!(h == 0 || px.len() >= (h - 1) * stride + w);
    unsafe { ref_fnv(px.as_ptr(), w as c_int, h as c_int, stride as c_int) }
}

pub fn basic() -> u32 {
    unsafe { ref_basic() }
}

pub fn shape(scale: usize, k: usize) -> u32 {
    unsafe { ref_shape(scale as c_int, k as c_int) }
}

pub fn frame() -> String {
    text(ref_frame)
}

pub fn style() -> u32 {
    unsafe { ref_style() }
}

extern "C" {
    fn ref_anim(out: *mut u32) -> c_int;
    fn ref_emoji(
        colr: *const u8,
        ncolr: usize,
        cbdt: *const u8,
        ncbdt: usize,
        sbix: *const u8,
        nsbix: usize,
        sub: *const u8,
        nsub: usize,
        out: *mut u32,
    ) -> c_int;
    fn ref_ime(out: *mut u32, input: *mut c_char, cap: c_int) -> c_int;
}

pub fn anim() -> ([u32; 4], bool) {
    let mut h = [0u32; 4];
    let done = unsafe { ref_anim(h.as_mut_ptr()) };
    assert!(done >= 0);
    (h, done == 1)
}

pub fn emoji(colr: &[u8], cbdt: &[u8], sbix: &[u8], sub: &[u8]) -> [u32; 6] {
    let mut h = [0u32; 6];
    let ok = unsafe {
        ref_emoji(
            colr.as_ptr(),
            colr.len(),
            cbdt.as_ptr(),
            cbdt.len(),
            sbix.as_ptr(),
            sbix.len(),
            sub.as_ptr(),
            sub.len(),
            h.as_mut_ptr(),
        )
    };
    assert_eq!(ok, 1);
    h
}

pub fn ime() -> ([u32; 5], String) {
    let mut h = [0u32; 5];
    let mut buf = vec![0 as c_char; 64];
    assert_eq!(unsafe { ref_ime(h.as_mut_ptr(), buf.as_mut_ptr(), buf.len() as c_int) }, 0);
    (h, unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() })
}
