use goome::{Canvas, Draw, Font};

fn font(name: &str) -> Font {
    Font::from_bytes(std::fs::read(format!("{}/tests/data/{}", env!("CARGO_MANIFEST_DIR"), name)).unwrap()).unwrap()
}

fn bytes(name: &str) -> Vec<u8> {
    std::fs::read(format!("{}/tests/data/{}", env!("CARGO_MANIFEST_DIR"), name)).unwrap()
}

fn hash(c: &Canvas) -> u32 {
    let mut px = vec![0u32; 400 * 120];
    let n = c.pixels().len().min(px.len());
    px[..n].copy_from_slice(&c.pixels()[..n]);
    goome_ref::fnv(&px, 400, 120, 400)
}

fn txt(f: &Font, size: f64, s: &str, argb: u32, dir: i32, w: usize, h: usize) -> u32 {
    let mut c = Canvas::new(w, h, 400, 1.0).unwrap();
    goome::text_dir(dir);
    c.font_text(Some(f), size, 4.0, 4.0, s, argb);
    goome::text_dir(goome::DIR_AUTO);
    hash(&c)
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

const HEB: &str = "ab \u{5d0}\u{5d1}\u{5d2} (12) c";
const ARAB: &str = "\u{628}\u{644}\u{627}\u{645} \u{628}\u{628}\u{628} \u{644}\u{645} \u{628}\u{64e}\u{628}";

#[test]
fn emoji_and_rtl_match_c() {
    let want = [0x3bab3875, 0x85fe883a, 0xd3add387, 0xb79518dd, 0x49a72d7d, 0x469719e1];
    assert_eq!(
        goome_ref::emoji(&bytes("colr.ttf"), &bytes("cbdt.ttf"), &bytes("sbix.ttf"), &bytes("dejavu-sub.ttf")),
        want
    );
    let (f, cb, sb, d) = (font("colr.ttf"), font("cbdt.ttf"), font("sbix.ttf"), font("dejavu-sub.ttf"));
    assert!(near(f.measure(30.0, "\u{1f600}\u{1f3fb}").0, 24.0));
    assert!(near(f.measure(30.0, "\u{1f600}\u{200d}\u{1f600}").0, 24.0));
    assert!(near(f.measure(30.0, "\u{2764}").0, 18.0) && near(f.measure(30.0, "\u{2764}\u{fe0f}").0, 24.0));
    let mut got = [0u32; 6];
    got[0] = txt(
        &f,
        30.0,
        "ab \u{1f600} \u{1f600}\u{1f3fb} \u{1f600}\u{200d}\u{1f600} \u{2764} \u{2764}\u{fe0f}",
        0xFF2040A0,
        0,
        400,
        60,
    );
    let mut c = Canvas::new(400, 120, 400, 1.0).unwrap();
    c.font_text(Some(&cb), 16.0, 4.0, 4.0, "\u{1f431}", 0xFFFFFFFF);
    c.font_text(Some(&cb), 64.0, 30.0, 4.0, "\u{1f431}", 0xFFFFFFFF);
    c.font_text(Some(&sb), 40.0, 120.0, 4.0, "\u{1f431}\u{1f431}", 0x80FFFFFF);
    got[1] = hash(&c);
    d.fallback(&f);
    d.fallback(&d);
    d.fallback(&f);
    assert_eq!(d.fallbacks().len(), 1);
    assert!(near(d.measure(20.0, "goome! \u{1f600}").0, d.measure(20.0, "goome! ").0 + 16.0));
    got[2] = txt(&d, 20.0, "goome! \u{1f600} Hi", 0xFFFFFFFF, 0, 400, 40);
    got[3] = txt(&f, 24.0, HEB, 0xFFFFFFFF, 0, 400, 40);
    assert_eq!(got[3], txt(&f, 24.0, HEB, 0xFFFFFFFF, goome::DIR_LTR, 400, 40));
    got[4] = txt(&f, 24.0, HEB, 0xFFFFFFFF, goome::DIR_RTL, 400, 40);
    assert!(near(
        f.measure(20.0, "\u{628}\u{644}\u{627}\u{645} \u{628}\u{628}\u{628} \u{644}\u{645}").0,
        93.2
    ));
    got[5] = txt(&f, 30.0, ARAB, 0xFFFFFFFF, 0, 400, 50);
    assert_eq!(got, want);
}

#[test]
fn fallback_outlives_its_handle() {
    let d = font("dejavu-sub.ttf");
    let before = d.measure(20.0, "goome! \u{1f600}").0;
    d.fallback(&font("colr.ttf"));
    assert!(near(d.measure(20.0, "goome! \u{1f600}").0, d.measure(20.0, "goome! ").0 + 16.0));
    assert!(before != d.measure(20.0, "goome! \u{1f600}").0);
}
