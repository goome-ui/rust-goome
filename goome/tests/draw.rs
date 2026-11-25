use goome::{Bitmap, Canvas, Draw, Radii};

const SHAPES: [[u32; 14]; 3] = [
    [
        0xa7b3b4c5, 0x0fb29a82, 0x1a98cf16, 0x1e7b729f, 0x21fd3b05, 0x241ce7fd, 0x6910b0d5, 0x7180b28e, 0x6f33ac30, 0xb48adfa3, 0x796b9105, 0x2d93dc92,
        0x1f908fad, 0x23a6cf0c,
    ],
    [
        0x73429a18, 0xd9bd97ab, 0xd031f56c, 0xd2d9fa9a, 0xd639a1a7, 0xa77c3e9e, 0x7bd3fead, 0xac76561e, 0x5bd0648d, 0xd0ac70e1, 0x0bb5dda0, 0xe7c304c7,
        0xccef8860, 0xefc1f212,
    ],
    [
        0x9175788f, 0x6e99ea87, 0x935cc8e1, 0x0b73f68d, 0x23891be5, 0xa28476c1, 0xea64f655, 0x87dc9430, 0x909d11b5, 0x0550bb7a, 0x6455ee86, 0xd8c08e2b,
        0xfd0dd1fd, 0x28c27977,
    ],
];

const STAR: [(f64, f64); 5] = [(80.0, 5.0), (98.0, 85.0), (40.0, 35.0), (120.0, 35.0), (62.0, 85.0)];

fn fnv(px: &[u32], w: usize, h: usize, stride: usize) -> u32 {
    let mut v: u32 = 2166136261;
    for y in 0..h {
        for &p in &px[y * stride..y * stride + w] {
            for k in 0..4 {
                v = (v ^ ((p >> (k * 8)) & 255)).wrapping_mul(16777619);
            }
        }
    }
    v
}

fn test_bitmap() -> Bitmap {
    let mut bp = vec![0u32; 27 * 16];
    for y in 0..16u32 {
        for x in 0..24u32 {
            let a = x * 255 / 23;
            let v = if (x / 4 + y / 4) & 1 == 1 { 0xFFFFFF } else { 0x2050C0 };
            bp[(y * 27 + x) as usize] = a << 24 | ((v >> 16 & 255) * a / 255) << 16 | ((v >> 8 & 255) * a / 255) << 8 | (v & 255) * a / 255;
        }
    }
    Bitmap::from_premul(bp, 24, 16, 27).unwrap()
}

fn shape(c: &mut Canvas, k: usize, b: &Bitmap) {
    match k {
        0 => c.rrect(10.3, 8.6, 120.0, 70.2, (4.0, 20.0, 0.0, 33.0), 0xE02060C0),
        1 => c.rrect_line(10.3, 8.6, 120.0, 70.2, 18.0, 2.5, 0xFF20A060),
        2 => c.rrect_grad(10.0, 10.0, 130.0, 70.0, 12.0, 0xFF102040, 0x8080C0FF, false),
        3 => c.rrect_grad(10.0, 10.0, 130.0, 70.0, 35.0, 0xFFFF4080, 0x004080FF, true),
        4 => c.shadow(30.0, 20.0, 90.0, 50.0, 10.0, 16.0, 0x80000000),
        5 => {
            c.circle(50.5, 50.2, 30.7, 0xFFFFC020);
            c.circle(120.0, 30.0, 1.3, 0xFF000000);
        }
        6 => c.ring(80.0, 50.0, 30.0, 5.0, 0xC0208080),
        7 => {
            c.arc(50.0, 50.0, 30.0, 6.0, 0.3, 4.5, 0xFF3060E0);
            c.arc(120.0, 50.0, 20.0, 30.0, 2.0, 2.5, 0xFF905020);
        }
        8 => c.poly(&STAR, 0xFF2040A0),
        9 => c.bitmap(b, 10.0, 10.0, 100.5, 75.3, 20.0, 0xFFFFFFFF),
        10 => c.bitmap(b, 10.0, 10.0, 24.0, 16.0, 0.0, 0x80FF8080),
        11 => c.bitmap(b, 10.0, 10.0, 9.0, 5.0, 2.0, 0xFFFFFFFF),
        12 => c.bitmap_part(b, 4, 2, 8, 8, 100.0, 10.0, 40.0, 40.0, 0xFFFFFFFF),
        _ => {
            c.clip_push(20.0, 15.0, 80.0, 50.0);
            c.rrect(0.0, 0.0, 160.0, 100.0, 30.0, 0xFF4080FF);
            c.clip_push(50.0, 0.0, 100.0, 40.0);
            c.circle(70.0, 40.0, 30.0, 0xFFFF4040);
            c.text_big(30.0, 30.0, "clip", 0xFF000000);
            c.shadow(40.0, 20.0, 60.0, 30.0, 8.0, 10.0, 0xFF000000);
            c.bitmap(b, 40.0, 10.0, 48.0, 32.0, Radii::all(6.0), 0xFFFFFFFF);
            c.clip_pop();
            c.line(0.0, 0.0, 160.0, 100.0, 3.0, 0xFF00A000);
            c.poly_flat(&[80.0, 5.0, 98.0, 85.0, 40.0, 35.0, 120.0, 35.0, 62.0, 85.0], 0x80000000);
            c.clip_pop();
            c.frame(20.0, 15.0, 80.0, 50.0, 1.0, 0x80000000);
        }
    }
}

#[test]
fn basic_matches_c() {
    let mut c = Canvas::new(160, 100, 200, 1.5).unwrap();
    c.round_rect(5.0, 5.0, 60.0, 40.0, 8.0, 0xC0203040);
    c.line(0.0, 0.0, 100.0, 60.0, 3.0, 0xFFFF0000);
    c.triangle(10.0, 50.0, 60.0, 50.0, 35.0, 10.0, 0x8000FF00);
    c.text_big(8.0, 8.0, "goome!", 0xFFFFFFFF);
    c.frame(70.0, 10.0, 30.0, 30.0, 2.0, 0xFF00FFFF);
    assert!((0..100).all(|y| c.pixels()[y * 200 + 170] == 0));
    let r = *c.raw();
    assert!(r.dx1 > r.dx0 && r.dy1 > r.dy0);
    let h = fnv(c.pixels(), 200, 100, 200);
    assert_eq!(h, 0xb93b60cf);
    assert_eq!(h, goome_ref::basic());
    assert_eq!(h, goome_ref::fnv(c.pixels(), 200, 100, 200));
    c.clear();
    assert!(c.pixels().iter().all(|&p| p == 0));
    assert!(c.text_width("abc", false) > 0.0);
}

#[test]
fn shapes_match_c() {
    let b = test_bitmap();
    for (s, sc) in [1.0, 1.25, 2.0].into_iter().enumerate() {
        for k in 0..14 {
            let (w, h) = ((160.0 * sc) as usize, (100.0 * sc) as usize);
            let mut c = Canvas::new(w, h, w + 3, sc).unwrap();
            shape(&mut c, k, &b);
            let px = c.pixels();
            let got = fnv(px, w + 3, h, w + 3);
            assert_eq!(got, SHAPES[s][k], "scale {} shape {}", sc, k);
            assert_eq!(got, goome_ref::shape(s, k), "scale {} shape {} vs C", sc, k);
            assert!((0..h).all(|y| px[y * (w + 3) + w..(y + 1) * (w + 3)].iter().all(|&p| p == 0)));
            for &p in px {
                let a = p >> 24;
                assert!((p >> 16 & 255) <= a && (p >> 8 & 255) <= a && (p & 255) <= a, "not premultiplied");
            }
        }
    }
}

#[test]
fn clip_keeps_drawing_inside() {
    let b = test_bitmap();
    let mut c = Canvas::new(160, 100, 163, 1.0).unwrap();
    c.clip_push(20.0, 15.0, 80.0, 50.0);
    shape(&mut c, 8, &b);
    shape(&mut c, 4, &b);
    shape(&mut c, 9, &b);
    c.text_big(0.0, 20.0, "outside", 0xFFFFFFFF);
    let r = *c.raw();
    assert!(r.dx0 >= 20 && r.dy0 >= 15 && r.dx1 <= 100 && r.dy1 <= 65 && r.dx1 > r.dx0);
    for y in 0..100 {
        for x in 0..160 {
            if !(20..100).contains(&x) || !(15..65).contains(&y) {
                assert_eq!(c.pixels()[y * 163 + x], 0, "drew outside the clip at {},{}", x, y);
            }
        }
    }
    c.clip_pop();
    assert_eq!(c.raw().nclip, 0);
}

#[test]
fn bitmaps_load_and_fail() {
    let b = Bitmap::from_argb(&[0xFFFF0000, 0x80FFFFFF, 0x00123456, 0x40804020], 2, 2).unwrap();
    assert_eq!(b.pixels(), &[0xFFFF0000, 0x80808080, 0, 0x40201008]);
    assert!(Bitmap::from_bytes(b"not an image").is_err());
    let e = Bitmap::load("theres-no-file-here.png").err().unwrap();
    assert!(!e.message.is_empty());
    assert!(Bitmap::from_argb(&[0; 3], 2, 2).is_err());
    assert!(Bitmap::from_argb(&[0; 3], 9, usize::MAX / 9 + 1).is_err());
    assert!(Bitmap::from_premul(vec![0; 1], 1001, 2, usize::MAX - 999).is_err());
    let copy = b.clone();
    drop(b);
    assert_eq!(copy.w(), 2);
    let mut c = Canvas::new(20, 20, 20, 1.0).unwrap();
    c.image(0.0, 0.0, 20.0, 20.0, &[0xFF00FF00; 16], 4, 4);
    assert_eq!(c.pixels()[0], 0xFF00FF00);
    let mut d = Canvas::new(40, 40, 40, 1.0).unwrap();
    d.blit(&mut c, 1.0, 10.0, 10.0);
    assert_eq!(d.pixels()[10 * 40 + 10], 0xFF00FF00);
}

#[test]
fn builtin_text_and_fonts() {
    assert!(goome::builtin_font_count() >= 1);
    let (w, h) = goome::builtin_measure(14.0, "hello");
    assert!(w > 0.0 && h > 0.0);
    assert!(goome::Font::load("nope.ttf").is_err());
    assert!(goome::Font::from_bytes(vec![1, 2, 3]).is_err());
    let mut c = Canvas::new(120, 40, 120, 1.0).unwrap();
    assert!(c.font_text(None, 16.0, 2.0, 2.0, "goome", 0xFFFFFFFF) > 0.0);
    assert!(c.pixels().iter().any(|&p| p != 0));
}
