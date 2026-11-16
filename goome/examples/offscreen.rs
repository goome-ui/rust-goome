use goome::{Event, Goome, WindowState};
use std::io::Write;

fn crc(data: &[u8]) -> u32 {
    let mut c = 0xFFFFFFFFu32;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = if c & 1 == 1 { 0xEDB88320 ^ (c >> 1) } else { c >> 1 };
        }
    }
    !c
}

fn chunk(out: &mut Vec<u8>, kind: &[u8], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let mut body = kind.to_vec();
    body.extend_from_slice(data);
    out.extend_from_slice(&body);
    out.extend_from_slice(&crc(&body).to_be_bytes());
}

fn stored_zlib(raw: &[u8]) -> Vec<u8> {
    let mut z = vec![0x78, 0x01];
    let blocks: Vec<&[u8]> = raw.chunks(65535).collect();
    for (i, b) in blocks.iter().enumerate() {
        z.push((i + 1 == blocks.len()) as u8);
        z.extend_from_slice(&(b.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(b.len() as u16)).to_le_bytes());
        z.extend_from_slice(b);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &c in raw {
        a = (a + c as u32) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&(b << 16 | a).to_be_bytes());
    z
}

fn write_png(path: &str, px: &[u32], w: usize, h: usize, stride: usize) -> std::io::Result<()> {
    let mut rows = Vec::with_capacity(h * (w * 3 + 1));
    for y in 0..h {
        rows.push(0);
        for x in 0..w {
            let s = px[y * stride + x];
            let bg: u32 = if (x / 16 + y / 16) & 1 == 1 { 0xFF3A4A5A } else { 0xFF2E3C4A };
            let a = s >> 24;
            for k in [16, 8, 0] {
                rows.push((((s >> k) & 255) + ((bg >> k) & 255) * (255 - a) / 255) as u8);
            }
        }
    }
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&(w as u32).to_be_bytes());
    ihdr.extend_from_slice(&(h as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &ihdr);
    chunk(&mut out, b"IDAT", &stored_zlib(&rows));
    chunk(&mut out, b"IEND", &[]);
    std::fs::File::create(path)?.write_all(&out)
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "offscreen.png".into());
    let mut ui = Goome::offscreen(360, 330, 2.0).and_then(|g| g.ui()).expect("cant open goome offscreen");
    ui.theme().anim_ms = 0.0;
    let mut win = WindowState::new(20.0, 20.0, 320.0, 0.0);
    let (mut check, mut volume, mut name) = (false, 0.25, String::from("goome"));
    for frame in 0..10 {
        let pushed = match frame {
            2 => vec![Event::down(40.0, 109.0, 1), Event::up(40.0, 109.0, 1)],
            3 => vec![Event::down(60.0, 163.0, 1)],
            4 => vec![Event::motion(250.0, 163.0)],
            5 => vec![Event::up(250.0, 163.0, 1)],
            6 => vec![Event::down(150.0, 223.0, 1), Event::up(150.0, 223.0, 1)],
            8 => vec![Event::motion(80.0, 279.0)],
            _ => vec![],
        };
        for ev in &pushed {
            ui.push(ev).unwrap();
        }
        if frame == 7 {
            ui.push_text(" from an engine").unwrap();
        }
        ui.pump(0);
        let Some(mut f) = ui.begin(0) else { continue };
        let mut w = f.window("offscreen", &mut win, goome::WIN_MOVE);
        if w.shown() {
            w.label("no window, the engine owns the screen");
            w.checkbox("Pushed a click", &mut check);
            w.slider_fmt("Volume", &mut volume, 0.0, 1.0, Some("%.2f"));
            w.input_cap("Name", &mut name, 64);
            w.progress(volume, None);
            w.button("Hover me");
        }
    }
    let cursor = ui.cursor_shape();
    let f = ui.last_frame().unwrap();
    write_png(&path, f.pixels(), f.w(), f.h(), f.stride()).expect("cant write the png");
    println!("check {} volume {:.2} name '{}' cursor {} -> {}", check as i32, volume, name, cursor, path);
}
