use goome::{A11yNode, Event, Options, Ui};

#[derive(Default)]
struct State {
    ok: i32,
    dark: bool,
    mode: i32,
    fruit: i32,
    tab: i32,
    open: bool,
    knob: i32,
    vol: f64,
    name: String,
    note: String,
    tree: Vec<A11yNode>,
}

fn frame(ui: &mut Ui, k: &mut State) {
    ui.pump(0);
    let Some(mut f) = ui.begin(0) else { return };
    {
        let mut a = f.app(Some("Demo"), goome::APP_TITLEBAR | goome::APP_STATUSBAR);
        a.label("Hello there");
        if a.button("OK") {
            k.ok += 1;
        }
        a.checkbox("Dark", &mut k.dark);
        a.radio("Big", &mut k.mode, 1);
        a.slider_fmt("Volume", &mut k.vol, 0.0, 10.0, Some("%.1f"));
        a.input_cap("Name", &mut k.name, 64);
        a.combo("Fruit", &mut k.fruit, &["Mango", "Kiwi", "Fig"]);
        a.tabs("tabs", &["One", "Two"], &mut k.tab);
        if let Some(mut t) = a.tree("Folder", &mut k.open) {
            t.label("inside");
        }
        a.progress(0.25, None);
        a.text_edit("Notes", &mut k.note, 60.0, 0);
        a.a11y_role(goome::ROLE_BUTTON, "Knob");
        a.a11y_value("11");
        if a.custom("knob", 40.0, 40.0, 0).clicked {
            k.knob += 1;
        }
        a.statusbar("ready");
    }
    drop(f);
    k.tree = ui.a11y_tree();
}

fn find(k: &State, role: i32, label: &str) -> Option<usize> {
    k.tree.iter().position(|n| n.role == role && n.label == label)
}

fn send(ui: &Ui, k: &State, verb: &str, i: usize, text: Option<&str>) {
    let mut e = Event::new(goome::CONTROL);
    e.msg = format!("a11y {} {}{}", verb, k.tree[i].id, text.map(|t| format!(" {}", t)).unwrap_or_default());
    ui.push(&e).unwrap();
}

#[test]
fn tree_and_actions() {
    let mut ui = Options::new()
        .backend(goome::TEST)
        .mode(goome::MODE_WINDOW)
        .size(420, 700)
        .test_script("end 100000")
        .open()
        .unwrap()
        .ui()
        .unwrap();
    ui.set_font(None);
    let mut k = State {
        vol: 2.0,
        name: "Bo".into(),
        ..State::default()
    };
    frame(&mut ui, &mut k);
    assert!(k.tree.len() > 15);
    let app = find(&k, goome::ROLE_WINDOW, "Demo").unwrap();
    let ok = find(&k, goome::ROLE_BUTTON, "OK").unwrap();
    let dark = find(&k, goome::ROLE_CHECKBOX, "Dark").unwrap();
    let vol = find(&k, goome::ROLE_SLIDER, "Volume").unwrap();
    let name = find(&k, goome::ROLE_INPUT, "Name").unwrap();
    let fruit = find(&k, goome::ROLE_COMBO, "Fruit").unwrap();
    let tree = find(&k, goome::ROLE_TREE, "Folder").unwrap();
    let knob = find(&k, goome::ROLE_BUTTON, "Knob").unwrap();
    let tab2 = find(&k, goome::ROLE_TAB, "Two").unwrap();
    let big = find(&k, goome::ROLE_RADIO, "Big").unwrap();
    let notes = find(&k, goome::ROLE_INPUT, "Notes").unwrap();
    assert_eq!((app, k.tree[app].parent), (0, None));
    assert!(k.tree[ok].parent == Some(app) && k.tree[ok].w > 0.0 && k.tree[ok].h > 0.0);
    assert!(find(&k, goome::ROLE_TEXT, "Hello there").is_some() && find(&k, goome::ROLE_TEXT, "ready").is_some());
    assert!(find(&k, goome::ROLE_BUTTON, "Close").is_some() && find(&k, goome::ROLE_BUTTON, "Minimize").is_some());
    assert!(!k.tree[dark].checked() && !k.tree[big].checked());
    let v = &k.tree[vol];
    assert_eq!((v.num, v.min, v.max, v.value.as_str()), (2.0, 0.0, 10.0, "2.0"));
    assert_eq!(k.tree[name].value, "Bo");
    assert!(k.tree[fruit].value == "Mango" && !k.tree[fruit].expanded());
    assert_eq!(k.tree[k.tree[tab2].parent.unwrap()].role, goome::ROLE_GROUP);
    assert!(!k.tree[tab2].selected() && k.tree[find(&k, goome::ROLE_TAB, "One").unwrap()].selected());
    assert!(!k.tree[tree].expanded() && find(&k, goome::ROLE_TEXT, "inside").is_none());
    assert_eq!(k.tree[knob].value, "11");
    let pr = k.tree.iter().position(|n| n.role == goome::ROLE_PROGRESS).unwrap();
    assert_eq!((k.tree[pr].num, k.tree[pr].value.as_str()), (0.25, "25%"));
    let ids: Vec<u32> = k.tree.iter().map(|n| n.id).collect();
    frame(&mut ui, &mut k);
    assert_eq!(ids, k.tree.iter().map(|n| n.id).collect::<Vec<_>>());
    let mut uniq = ids.clone();
    uniq.sort();
    uniq.dedup();
    assert_eq!(uniq.len(), ids.len());

    send(&ui, &k, "press", ok, None);
    send(&ui, &k, "press", dark, None);
    send(&ui, &k, "press", big, None);
    send(&ui, &k, "set", vol, Some("7.5"));
    send(&ui, &k, "set", name, Some("hello there"));
    send(&ui, &k, "press", knob, None);
    send(&ui, &k, "expand", tree, None);
    send(&ui, &k, "press", tab2, None);
    send(&ui, &k, "set", notes, Some("line one"));
    frame(&mut ui, &mut k);
    assert_eq!((k.ok, k.dark, k.mode, k.vol, k.knob, k.open, k.tab), (1, true, 1, 7.5, 1, true, 1));
    assert_eq!((k.name.as_str(), k.note.as_str()), ("hello there", "line one"));
    frame(&mut ui, &mut k);
    let tree = find(&k, goome::ROLE_TREE, "Folder").unwrap();
    assert!(k.tree[find(&k, goome::ROLE_CHECKBOX, "Dark").unwrap()].checked());
    assert_eq!(k.tree[find(&k, goome::ROLE_SLIDER, "Volume").unwrap()].value, "7.5");
    assert!(k.tree[tree].expanded());
    let inside = find(&k, goome::ROLE_TEXT, "inside").unwrap();
    assert_eq!(k.tree[inside].parent, Some(tree));

    let name = find(&k, goome::ROLE_INPUT, "Name").unwrap();
    send(&ui, &k, "focus", name, None);
    frame(&mut ui, &mut k);
    frame(&mut ui, &mut k);
    let n = &k.tree[find(&k, goome::ROLE_INPUT, "Name").unwrap()];
    assert!(n.focused() && n.caret == 11);
    assert_eq!(k.tree.iter().filter(|n| n.focused()).count(), 1);

    let fruit = find(&k, goome::ROLE_COMBO, "Fruit").unwrap();
    send(&ui, &k, "expand", fruit, None);
    frame(&mut ui, &mut k);
    frame(&mut ui, &mut k);
    let fruit = find(&k, goome::ROLE_COMBO, "Fruit").unwrap();
    let fig = find(&k, goome::ROLE_ITEM, "Fig").unwrap();
    let list = k.tree[fig].parent.unwrap();
    assert!(k.tree[fruit].expanded() && k.tree[list].role == goome::ROLE_LIST && k.tree[list].parent == Some(fruit));
    send(&ui, &k, "press", fig, None);
    frame(&mut ui, &mut k);
    frame(&mut ui, &mut k);
    assert!(k.fruit == 2 && find(&k, goome::ROLE_ITEM, "Fig").is_none());

    ui.a11y(false);
    frame(&mut ui, &mut k);
    assert!(k.tree.is_empty());
}

#[test]
fn offscreen_starts_off() {
    let mut ui = goome::Goome::offscreen(200, 100, 1.0).unwrap().ui().unwrap();
    ui.set_font(None);
    for pass in 0..2 {
        if let Some(mut f) = ui.begin(0) {
            f.button("Hi");
        }
        assert_eq!(ui.a11y_tree().len(), pass);
        ui.a11y(true);
    }
    if let Some(mut f) = ui.begin(0) {
        f.button("Hi");
    }
    let t = ui.a11y_tree();
    assert!(t.len() == 1 && t[0].role == goome::ROLE_BUTTON && t[0].label == "Hi");
}
