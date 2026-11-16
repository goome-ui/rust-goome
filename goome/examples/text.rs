use goome::{App, Font};

const LINES: [&str; 5] = [
    "emoji \u{1f600} \u{1f44d}\u{1f3fd} \u{1f408}\u{200d}\u{2b1b} \u{2764}\u{fe0f}",
    "\u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{5e2}\u{5d5}\u{5dc}\u{5dd} (123)",
    "\u{645}\u{631}\u{62d}\u{628}\u{627} \u{628}\u{627}\u{644}\u{639}\u{627}\u{644}\u{645}",
    "mixed: goome \u{5d6}\u{5d4} \u{5e7}\u{5dc} 42 ok",
    "\u{1f1ef}\u{1f1f5} \u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f467} \u{263a}\u{fe0f}",
];

fn main() {
    let mut app = match App::new("Text", 640, 460) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("cant open a window: {}", e);
            std::process::exit(1);
        }
    };
    let font = Font::system(None).ok();
    if let (Some(f), Some(path)) = (&font, std::env::args().nth(1)) {
        match Font::load(&path) {
            Ok(extra) => f.fallback(&extra),
            Err(e) => eprintln!("{}", e),
        }
    }
    app.ui().set_font(font.as_ref());
    if font.is_none() {
        eprintln!("theres no system font here, using the built in one (no emoji or rtl with it)");
    }
    let (mut dir, mut note) = (0, String::from("type here, the ime works too \u{1f60e}"));
    while let Some(mut ui) = app.frame() {
        ui.heading("emoji and rtl");
        ui.radio("Auto", &mut dir, goome::DIR_AUTO);
        ui.same_line();
        ui.radio("Left to right", &mut dir, goome::DIR_LTR);
        ui.same_line();
        ui.radio("Right to left", &mut dir, goome::DIR_RTL);
        goome::text_dir(dir);
        ui.separator();
        for l in LINES {
            ui.label(l);
        }
        ui.separator();
        ui.input("Note", &mut note);
        ui.text_edit("more", &mut note, 90.0, goome::EDIT_WRAP);
    }
}
