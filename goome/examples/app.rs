use goome::App;

struct Note {
    title: String,
    body: String,
    pinned: bool,
}

fn main() {
    let mut app = match App::new("Notes", 980, 640) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("cant open a window: {}", e);
            std::process::exit(1);
        }
    };
    app.flags = goome::APP_MENUBAR | goome::APP_STATUSBAR | goome::APP_NO_PAD;
    let mut notes = vec![
        Note {
            title: "Groceries".into(),
            body: "milk\neggs\ncoffee\nrice\ncat food".into(),
            pinned: true,
        },
        Note {
            title: "Release checklist".into(),
            body: "bump the version\nrun the tests\nwrite the changelog\ntag it".into(),
            pinned: false,
        },
        Note {
            title: "Books".into(),
            body: "finish the one on the nightstand first".into(),
            pinned: false,
        },
    ];
    let (mut sel, mut side, mut search) = (0usize, 0.27, String::new());
    let (mut sidebar, mut wrap, mut light, mut about, mut confirm) = (true, true, false, false, false);
    let mut quit = false;
    while let Some(mut ui) = app.frame() {
        for ev in ui.events().to_vec() {
            if ev.kind == goome::KEY && ev.mods & goome::CTRL != 0 {
                match char::from_u32(ev.key as u32) {
                    Some('n') => {
                        notes.push(Note {
                            title: "Untitled".into(),
                            body: String::new(),
                            pinned: false,
                        });
                        sel = notes.len() - 1;
                    }
                    Some('b') => sidebar = !sidebar,
                    Some('q') => quit = true,
                    _ => {}
                }
            }
        }
        if let Some(mut bar) = ui.menubar() {
            if let Some(mut m) = bar.menu("File") {
                if m.menu_item("New note", Some("Ctrl+N"), None) {
                    notes.push(Note {
                        title: "Untitled".into(),
                        body: String::new(),
                        pinned: false,
                    });
                    sel = notes.len() - 1;
                }
                m.menu_separator();
                if m.menu_item("Quit", Some("Ctrl+Q"), None) {
                    quit = true;
                }
            }
            if let Some(mut m) = bar.menu("Edit") {
                if m.menu_item("Copy note", None, None) && sel < notes.len() {
                    let body = notes[sel].body.clone();
                    let _ = m.clipboard_set(&body);
                    m.toast("Copied", 1500);
                }
                if m.menu_item("Delete note", Some("Del"), None) && !notes.is_empty() {
                    confirm = true;
                }
            }
            if let Some(mut m) = bar.menu("View") {
                m.menu_item("Sidebar", Some("Ctrl+B"), Some(&mut sidebar));
                m.menu_item("Wrap text", None, Some(&mut wrap));
                if m.menu_item("Light theme", None, Some(&mut light)) {
                    if light {
                        m.theme().set_light();
                    } else {
                        m.theme().set_dark();
                    }
                }
            }
            if let Some(mut m) = bar.menu("Help") {
                if m.menu_item("About Notes", None, None) {
                    about = true;
                }
            }
        }
        if sidebar {
            let mut panes = ui.split("main", false, &mut side);
            panes.input_ex("##search", &mut search, 0, Some("Search notes"));
            let needle = search.to_lowercase();
            {
                let mut list = panes.scroll("list", 0.0);
                for (i, n) in notes.iter().enumerate() {
                    if !needle.is_empty() && !n.title.to_lowercase().contains(&needle) && !n.body.to_lowercase().contains(&needle) {
                        continue;
                    }
                    let mut id = list.id(&i.to_string());
                    let name = if n.pinned { format!("* {}", n.title) } else { n.title.clone() };
                    if id.selectable(&name, i == sel) {
                        sel = i;
                    }
                }
            }
            if panes.button("New note") {
                notes.push(Note {
                    title: "Untitled".into(),
                    body: String::new(),
                    pinned: false,
                });
                sel = notes.len() - 1;
            }
            panes.split_next();
            editor(&mut panes, &mut notes, sel, wrap);
        } else {
            editor(&mut ui, &mut notes, sel, wrap);
        }
        if let Some(mut m) = ui.modal("Delete note?", &mut confirm) {
            m.label("this cant be undone");
            m.row(2);
            if m.button("Delete") && sel < notes.len() {
                notes.remove(sel);
                sel = sel.saturating_sub(1);
                m.popup_close();
            }
            if m.button("Keep it") {
                m.popup_close();
            }
        }
        if let Some(mut m) = ui.modal("About Notes", &mut about) {
            let line = format!("goome {} on {}", goome::VERSION, m.backend_name());
            m.label(&line);
            m.wrapped("a tiny notes app made with goome and Rust.");
        }
        let words: usize = notes.iter().map(|n| n.body.split_whitespace().count()).sum();
        ui.statusbar(&format!("{} notes, {} words", notes.len(), words));
        if quit {
            ui.quit();
        }
    }
    println!("{} frames", app.frames);
}

fn editor(ui: &mut goome::Ui, notes: &mut [Note], sel: usize, wrap: bool) {
    let Some(n) = notes.get_mut(sel) else {
        ui.label("no notes yet, make one with Ctrl+N");
        return;
    };
    ui.input("Title", &mut n.title);
    ui.same_line();
    ui.toggle("Pinned", &mut n.pinned);
    let flags = goome::EDIT_LINE_NUMBERS | if wrap { goome::EDIT_WRAP } else { 0 };
    ui.text_edit("body", &mut n.body, 0.0, flags);
}
