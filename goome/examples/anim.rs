use goome::{App, Draw};

const EASES: [(&str, i32); 7] = [
    ("Out", goome::EASE_OUT),
    ("In", goome::EASE_IN),
    ("In out", goome::EASE_IN_OUT),
    ("Smooth", goome::EASE_SMOOTH),
    ("Linear", goome::EASE_LINEAR),
    ("Spring", goome::EASE_SPRING),
    ("Bounce", goome::EASE_BOUNCE),
];

fn main() {
    let mut app = match App::new("Animations", 560, 520) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("cant open a window: {}", e);
            std::process::exit(1);
        }
    };
    let names: Vec<&str> = EASES.iter().map(|e| e.0).collect();
    let (mut open, mut pick, mut ms) = (false, 5, 450.0);
    while let Some(mut ui) = app.frame() {
        ui.heading("goome animations");
        ui.label("click the box or the button, the ui keeps redrawing until its done moving");
        if ui.button(if open { "Send it back" } else { "Send it over" }) {
            open = !open;
        }
        ui.combo("Ease", &mut pick, &names);
        ui.slider_fmt("Time", &mut ms, 50.0, 2000.0, Some("%.0f ms"));
        let ease = EASES[pick as usize].1;
        let x = ui.anim("slide", if open { 1.0 } else { 0.0 }, ms, ease);
        let size = ui.anim("size", if open { 70.0 } else { 40.0 }, ms, ease);
        let col = ui.anim_color("tint", if open { 0xFF6FDC5F } else { 0xFF4FB3F7 }, ms, goome::EASE_LINEAR);
        let it = ui.custom("stage", 500.0, 140.0, 0);
        if it.clicked {
            open = !open;
        }
        if let Some(mut c) = ui.canvas() {
            c.rrect(it.x, it.y, it.w, it.h, 12.0, 0xFF262C36);
            let travel = it.w - 40.0 - size;
            c.rrect(it.x + 20.0 + x * travel, it.y + (it.h - size) / 2.0, size, size, size / 2.0, col);
        }
        ui.label(&format!("value {:.3}", x));
        let mut curve = [0f32; 40];
        for (i, v) in curve.iter_mut().enumerate() {
            *v = goome::ease(ease, i as f64 / 39.0) as f32;
        }
        ui.plot_lines("curve", &curve, -0.2, 1.3, 60.0);
    }
}
