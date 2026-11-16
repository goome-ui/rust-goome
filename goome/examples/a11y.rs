use goome::Goome;

const ROLES: [&str; 20] = [
    "none", "window", "button", "checkbox", "radio", "slider", "text", "input", "combo", "list", "item", "tab", "menu", "menuitem", "table", "tree",
    "progress", "image", "group", "dialog",
];

fn main() {
    let mut ui = Goome::offscreen(420, 360, 1.0).and_then(|g| g.ui()).expect("cant open goome offscreen");
    ui.a11y(true);
    let (mut dark, mut vol, mut name, mut fruit) = (true, 4.0, String::from("sev"), 1);
    for _ in 0..2 {
        let Some(mut f) = ui.begin(0) else { continue };
        let mut a = f.app(Some("Settings"), goome::APP_TITLEBAR);
        a.label("this is what a screen reader gets");
        a.checkbox("Dark mode", &mut dark);
        a.slider_fmt("Volume", &mut vol, 0.0, 10.0, Some("%.0f"));
        a.input("Name", &mut name);
        a.combo("Fruit", &mut fruit, &["Mango", "Kiwi", "Fig"]);
        a.a11y_role(goome::ROLE_BUTTON, "Big red button");
        a.a11y_value("dont press it");
        a.custom("red", 80.0, 40.0, 0);
        a.button("Save");
    }
    let tree = ui.a11y_tree();
    for n in &tree {
        let mut depth = 0;
        let mut p = n.parent;
        while let Some(i) = p {
            depth += 1;
            p = tree[i].parent;
        }
        let role = ROLES.get(n.role as usize).copied().unwrap_or("?");
        let mut line = format!("{}{} \"{}\"", "  ".repeat(depth), role, n.label);
        if !n.value.is_empty() {
            line += &format!(" = {}", n.value);
        }
        if n.checked() {
            line += " (checked)";
        }
        println!("{} at {:.0},{:.0} {:.0}x{:.0}", line, n.x, n.y, n.w, n.h);
    }
}
