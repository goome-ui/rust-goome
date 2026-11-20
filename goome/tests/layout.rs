use goome::sys;
use std::collections::HashMap;

fn sys_layout() -> Vec<(String, usize, usize)> {
    macro_rules! s {
        ($v:ident, $t:ident, $($f:ident)*) => {
            $v.push((stringify!($t).to_string(), std::mem::size_of::<sys::$t>(), 0));
            $( $v.push((format!("{}.{}", stringify!($t), stringify!($f).trim_end_matches('_')), std::mem::offset_of!(sys::$t, $f), {
                let z: sys::$t = unsafe { std::mem::zeroed() };
                std::mem::size_of_val(&z.$f)
            })); )*
        };
    }
    let mut v = Vec::new();
    s!(v, goome_opts, title backend state_dir no_capture capture_fps hide_self test_frames test_script test_shots mode width height min_width min_height window_flags scale);
    s!(v, goome_rect, x y w h);
    s!(v, goome_monitor, id name area scale pw ph rotation);
    s!(v, goome_image, px w h stride area mon has_self has_cursor cursor);
    s!(v, goome_canvas, px w h stride ox oy scale mon dx0 dy0 dx1 dy1 cx0 cy0 cx1 cy1 kx0 ky0 kx1 ky1 nclip clips);
    s!(v, goome_event, type_ x y mon button dy key mods text id msg);
    s!(v, goome_pacer, next fps);
    s!(v, goome_radii, tl tr br bl);
    s!(v, goome_bitmap, px w h stride owned);
    s!(v, goome_theme, bg panel border text text_dim accent accent_text hover active input_bg shadow radius pad gap border_width font_size shadow_blur anim_ms font skin_button skin_panel skin_input icon icon_tint skin_checkbox skin_radio skin_toggle skin_slider skin_slider_fill skin_scrollbar skin_tab skin_titlebar skin_tooltip skin_menu skin_popup focus select caret link badge toast tooltip danger knob dim check_size check_radius toggle_w toggle_h slider_h slider_track knob_size scroll_w scroll_min item_h menu_item_h tab_h win_title_h app_title_h menubar_h statusbar_h focus_w focus_gap sep_w grip indent plot_w spinner caret_w win_min_w win_min_h pop_gap menu_gap tip_pad_x tip_pad_y tip_x tip_y tip_radius shadow_x shadow_y align heading_size small_size ease hover_ms press_ms popup_ms collapse_ms toast_fade_ms font_heading font_mono font_small);
    s!(v, goome_skin, bm l t r b);
    s!(v, goome_ui_item, x y w h hovered pressed held released clicked double_clicked right_clicked focused mx my dx dy drag_x drag_y key);
    s!(v, goome_combo, key mods);
    s!(v, goome_ui_config, tip_delay_ms dbl_click_ms dbl_click_px drag_px repeat_delay_ms repeat_ms blink_ms menu_delay_ms toast_ms scroll_lines raise_on_click esc_closes tab_nav focus_on_mouse cursor keys ease ease_user mask);
    s!(v, goome_a11y_node, role parent states x y w h label value id num min max caret);
    v
}

fn c_layout() -> HashMap<String, (usize, usize)> {
    goome_ref::layout()
        .lines()
        .map(|l| {
            let p: Vec<&str> = l.split_whitespace().collect();
            (p[0].to_string(), (p[1].parse().unwrap(), p[2].parse().unwrap()))
        })
        .collect()
}

fn same(c: &HashMap<String, (usize, usize)>, mine: &[(String, usize, usize)]) {
    for (name, off, size) in mine {
        let got = c.get(name).unwrap_or_else(|| panic!("C doesnt know {}", name));
        if name.contains('.') {
            assert_eq!((*off, *size), *got, "{}", name);
        } else {
            assert_eq!(*off, got.0, "sizeof {}", name);
        }
    }
}

#[test]
fn structs_match_c() {
    let c = c_layout();
    let mine = sys_layout();
    assert_eq!(mine.len(), c.len(), "every struct field in layout.c should be checked");
    same(&c, &mine);
}

#[test]
fn mirrors_match_c() {
    let c = c_layout();
    let mine = goome::mirror_layout();
    assert!(mine.len() > 100);
    same(&c, &mine);
    assert_eq!(std::mem::size_of::<goome::Rect>(), std::mem::size_of::<sys::goome_rect>());
    assert_eq!(std::mem::size_of::<goome::Radii>(), std::mem::size_of::<sys::goome_radii>());
}

fn header() -> String {
    let h = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../goome-sys/native/goome.h")).unwrap();
    h.split("#if defined(GOOME_IMPLEMENTATION) && !defined(GOOME_IMPLEMENTED)")
        .next()
        .unwrap()
        .to_string()
}

fn mine_consts() -> Vec<(&'static str, i32)> {
    vec![
        ("GOOME_AUTO", goome::AUTO),
        ("GOOME_WAYLAND", goome::WAYLAND),
        ("GOOME_GNOME", goome::GNOME),
        ("GOOME_PORTAL", goome::PORTAL),
        ("GOOME_X11", goome::X11),
        ("GOOME_WIN32", goome::WIN32),
        ("GOOME_MACOS", goome::MACOS),
        ("GOOME_TEST", goome::TEST),
        ("GOOME_OFFSCREEN", goome::OFFSCREEN),
        ("GOOME_MODE_OVERLAY", goome::MODE_OVERLAY),
        ("GOOME_MODE_WINDOW", goome::MODE_WINDOW),
        ("GOOME_MODE_OFFSCREEN", goome::MODE_OFFSCREEN),
        ("GOOME_WINDOW_RESIZABLE", goome::WINDOW_RESIZABLE),
        ("GOOME_WINDOW_BORDERLESS", goome::WINDOW_BORDERLESS),
        ("GOOME_WINDOW_TRANSPARENT", goome::WINDOW_TRANSPARENT),
        ("GOOME_WINDOW_ON_TOP", goome::WINDOW_ON_TOP),
        ("GOOME_WINDOW_HIDDEN", goome::WINDOW_HIDDEN),
        ("GOOME_WINDOW_CENTER", goome::WINDOW_CENTER),
        ("GOOME_WINDOW_NO_TITLEBAR", goome::WINDOW_NO_TITLEBAR),
        ("GOOME_WINDOW_NO_CONSOLE", goome::WINDOW_NO_CONSOLE),
        ("GOOME_OK", goome::OK),
        ("GOOME_ENOTSUP", goome::ENOTSUP),
        ("GOOME_EDENIED", goome::EDENIED),
        ("GOOME_ENODISPLAY", goome::ENODISPLAY),
        ("GOOME_EGONE", goome::EGONE),
        ("GOOME_EAGAIN", goome::EAGAIN),
        ("GOOME_ENOMEM", goome::ENOMEM),
        ("GOOME_EARG", goome::EARG),
        ("GOOME_CAP_CAPTURE", goome::CAP_CAPTURE),
        ("GOOME_CAP_GLOBAL_POINTER", goome::CAP_GLOBAL_POINTER),
        ("GOOME_CAP_HOTKEYS", goome::CAP_HOTKEYS),
        ("GOOME_CAP_HIDE_SELF", goome::CAP_HIDE_SELF),
        ("GOOME_CAP_KEYBOARD", goome::CAP_KEYBOARD),
        ("GOOME_CAP_WINDOW", goome::CAP_WINDOW),
        ("GOOME_CAP_CLIPBOARD", goome::CAP_CLIPBOARD),
        ("GOOME_CAP_CURSOR", goome::CAP_CURSOR),
        ("GOOME_CAP_GRAB", goome::CAP_GRAB),
        ("GOOME_WANT_MOUSE", goome::WANT_MOUSE),
        ("GOOME_WANT_KEYBOARD", goome::WANT_KEYBOARD),
        ("GOOME_NONE", goome::NONE),
        ("GOOME_DOWN", goome::DOWN),
        ("GOOME_UP", goome::UP),
        ("GOOME_MOTION", goome::MOTION),
        ("GOOME_SCROLL", goome::SCROLL),
        ("GOOME_KEY", goome::KEY),
        ("GOOME_HOTKEY", goome::HOTKEY),
        ("GOOME_CONTROL", goome::CONTROL),
        ("GOOME_MONITORS", goome::MONITORS),
        ("GOOME_QUIT", goome::QUIT),
        ("GOOME_CLOSE", goome::CLOSE),
        ("GOOME_FOCUS", goome::FOCUS),
        ("GOOME_DROP", goome::DROP),
        ("GOOME_TEXT", goome::TEXT),
        ("GOOME_PREEDIT", goome::PREEDIT),
        ("GOOME_CURSOR_ARROW", goome::CURSOR_ARROW),
        ("GOOME_CURSOR_IBEAM", goome::CURSOR_IBEAM),
        ("GOOME_CURSOR_HAND", goome::CURSOR_HAND),
        ("GOOME_CURSOR_MOVE", goome::CURSOR_MOVE),
        ("GOOME_CURSOR_RESIZE_EW", goome::CURSOR_RESIZE_EW),
        ("GOOME_CURSOR_RESIZE_NS", goome::CURSOR_RESIZE_NS),
        ("GOOME_CURSOR_RESIZE_NWSE", goome::CURSOR_RESIZE_NWSE),
        ("GOOME_CURSOR_RESIZE_NESW", goome::CURSOR_RESIZE_NESW),
        ("GOOME_CURSOR_WAIT", goome::CURSOR_WAIT),
        ("GOOME_CURSOR_NO", goome::CURSOR_NO),
        ("GOOME_CURSOR_HIDDEN", goome::CURSOR_HIDDEN),
        ("GOOME_EDGE_LEFT", goome::EDGE_LEFT),
        ("GOOME_EDGE_RIGHT", goome::EDGE_RIGHT),
        ("GOOME_EDGE_TOP", goome::EDGE_TOP),
        ("GOOME_EDGE_BOTTOM", goome::EDGE_BOTTOM),
        ("GOOME_STATE_NORMAL", goome::STATE_NORMAL),
        ("GOOME_STATE_MINIMIZED", goome::STATE_MINIMIZED),
        ("GOOME_STATE_MAXIMIZED", goome::STATE_MAXIMIZED),
        ("GOOME_STATE_FULLSCREEN", goome::STATE_FULLSCREEN),
        ("GOOME_K_BACKSPACE", goome::K_BACKSPACE),
        ("GOOME_K_TAB", goome::K_TAB),
        ("GOOME_K_ENTER", goome::K_ENTER),
        ("GOOME_K_ESC", goome::K_ESC),
        ("GOOME_K_SPACE", goome::K_SPACE),
        ("GOOME_K_DELETE", goome::K_DELETE),
        ("GOOME_K_F1", goome::K_F1),
        ("GOOME_K_F12", goome::K_F12),
        ("GOOME_K_LEFT", goome::K_LEFT),
        ("GOOME_K_RIGHT", goome::K_RIGHT),
        ("GOOME_K_UP", goome::K_UP),
        ("GOOME_K_DOWN", goome::K_DOWN),
        ("GOOME_K_HOME", goome::K_HOME),
        ("GOOME_K_END", goome::K_END),
        ("GOOME_K_PGUP", goome::K_PGUP),
        ("GOOME_K_PGDN", goome::K_PGDN),
        ("GOOME_K_INSERT", goome::K_INSERT),
        ("GOOME_SHIFT", goome::SHIFT),
        ("GOOME_CTRL", goome::CTRL),
        ("GOOME_ALT", goome::ALT),
        ("GOOME_SUPER", goome::SUPER),
        ("GOOME_FMT_BGRA_PREMUL", goome::FMT_BGRA_PREMUL),
        ("GOOME_FMT_RGBA_PREMUL", goome::FMT_RGBA_PREMUL),
        ("GOOME_FMT_BGRA", goome::FMT_BGRA),
        ("GOOME_FMT_RGBA", goome::FMT_RGBA),
        ("GOOME_ICON_ARROW_DOWN", goome::ICON_ARROW_DOWN),
        ("GOOME_ICON_ARROW_RIGHT", goome::ICON_ARROW_RIGHT),
        ("GOOME_ICON_ARROW_UP", goome::ICON_ARROW_UP),
        ("GOOME_ICON_ARROW_LEFT", goome::ICON_ARROW_LEFT),
        ("GOOME_ICON_CHECK", goome::ICON_CHECK),
        ("GOOME_ICON_RADIO_DOT", goome::ICON_RADIO_DOT),
        ("GOOME_ICON_CLOSE", goome::ICON_CLOSE),
        ("GOOME_ICON_MINIMIZE", goome::ICON_MINIMIZE),
        ("GOOME_ICON_MAXIMIZE", goome::ICON_MAXIMIZE),
        ("GOOME_ICON_RESTORE", goome::ICON_RESTORE),
        ("GOOME_ICON_RESIZE", goome::ICON_RESIZE),
        ("GOOME_ICON_SORT_UP", goome::ICON_SORT_UP),
        ("GOOME_ICON_SORT_DOWN", goome::ICON_SORT_DOWN),
        ("GOOME_ICON_PLUS", goome::ICON_PLUS),
        ("GOOME_ICON_MINUS", goome::ICON_MINUS),
        ("GOOME_ICON_SLIDER_KNOB", goome::ICON_SLIDER_KNOB),
        ("GOOME_ICON_TOGGLE_KNOB", goome::ICON_TOGGLE_KNOB),
        ("GOOME_ICON_COLLAPSE", goome::ICON_COLLAPSE),
        ("GOOME_ICON_EXPAND", goome::ICON_EXPAND),
        ("GOOME_ICON_DRAG_HANDLE", goome::ICON_DRAG_HANDLE),
        ("GOOME_ICON_TOAST", goome::ICON_TOAST),
        ("GOOME_ICON_PICKER", goome::ICON_PICKER),
        ("GOOME_ICON_COUNT", goome::ICON_COUNT),
        ("GOOME_WIN_MOVE", goome::WIN_MOVE),
        ("GOOME_WIN_RESIZE", goome::WIN_RESIZE),
        ("GOOME_WIN_COLLAPSE", goome::WIN_COLLAPSE),
        ("GOOME_WIN_NO_TITLE", goome::WIN_NO_TITLE),
        ("GOOME_WIN_CLOSE", goome::WIN_CLOSE),
        ("GOOME_INPUT_PASSWORD", goome::INPUT_PASSWORD),
        ("GOOME_INPUT_READONLY", goome::INPUT_READONLY),
        ("GOOME_INPUT_NUMBER", goome::INPUT_NUMBER),
        ("GOOME_INPUT_NO_SELECT_ON_FOCUS", goome::INPUT_NO_SELECT_ON_FOCUS),
        ("GOOME_INPUT_ENTER_RETURNS", goome::INPUT_ENTER_RETURNS),
        ("GOOME_APP_TITLEBAR", goome::APP_TITLEBAR),
        ("GOOME_APP_MENUBAR", goome::APP_MENUBAR),
        ("GOOME_APP_STATUSBAR", goome::APP_STATUSBAR),
        ("GOOME_APP_NO_PAD", goome::APP_NO_PAD),
        ("GOOME_EDIT_READONLY", goome::EDIT_READONLY),
        ("GOOME_EDIT_LINE_NUMBERS", goome::EDIT_LINE_NUMBERS),
        ("GOOME_EDIT_WRAP", goome::EDIT_WRAP),
        ("GOOME_EDIT_MONO", goome::EDIT_MONO),
        ("GOOME_TABLE_SORTABLE", goome::TABLE_SORTABLE),
        ("GOOME_TABLE_RESIZABLE", goome::TABLE_RESIZABLE),
        ("GOOME_TABLE_STRIPES", goome::TABLE_STRIPES),
        ("GOOME_TABLE_BORDERS", goome::TABLE_BORDERS),
        ("GOOME_S_BG", goome::S_BG),
        ("GOOME_S_PANEL", goome::S_PANEL),
        ("GOOME_S_BORDER", goome::S_BORDER),
        ("GOOME_S_TEXT", goome::S_TEXT),
        ("GOOME_S_TEXT_DIM", goome::S_TEXT_DIM),
        ("GOOME_S_ACCENT", goome::S_ACCENT),
        ("GOOME_S_ACCENT_TEXT", goome::S_ACCENT_TEXT),
        ("GOOME_S_HOVER", goome::S_HOVER),
        ("GOOME_S_ACTIVE", goome::S_ACTIVE),
        ("GOOME_S_INPUT_BG", goome::S_INPUT_BG),
        ("GOOME_S_SHADOW", goome::S_SHADOW),
        ("GOOME_S_RADIUS", goome::S_RADIUS),
        ("GOOME_S_PAD", goome::S_PAD),
        ("GOOME_S_GAP", goome::S_GAP),
        ("GOOME_S_BORDER_WIDTH", goome::S_BORDER_WIDTH),
        ("GOOME_S_FONT_SIZE", goome::S_FONT_SIZE),
        ("GOOME_S_SHADOW_BLUR", goome::S_SHADOW_BLUR),
        ("GOOME_S_ANIM_MS", goome::S_ANIM_MS),
        ("GOOME_S_FOCUS", goome::S_FOCUS),
        ("GOOME_S_SELECT", goome::S_SELECT),
        ("GOOME_S_CARET", goome::S_CARET),
        ("GOOME_S_LINK", goome::S_LINK),
        ("GOOME_S_BADGE", goome::S_BADGE),
        ("GOOME_S_TOAST", goome::S_TOAST),
        ("GOOME_S_TOOLTIP", goome::S_TOOLTIP),
        ("GOOME_S_DANGER", goome::S_DANGER),
        ("GOOME_S_KNOB", goome::S_KNOB),
        ("GOOME_S_DIM", goome::S_DIM),
        ("GOOME_S_CHECK_SIZE", goome::S_CHECK_SIZE),
        ("GOOME_S_CHECK_RADIUS", goome::S_CHECK_RADIUS),
        ("GOOME_S_TOGGLE_W", goome::S_TOGGLE_W),
        ("GOOME_S_TOGGLE_H", goome::S_TOGGLE_H),
        ("GOOME_S_SLIDER_H", goome::S_SLIDER_H),
        ("GOOME_S_SLIDER_TRACK", goome::S_SLIDER_TRACK),
        ("GOOME_S_KNOB_SIZE", goome::S_KNOB_SIZE),
        ("GOOME_S_SCROLL_W", goome::S_SCROLL_W),
        ("GOOME_S_SCROLL_MIN", goome::S_SCROLL_MIN),
        ("GOOME_S_ITEM_H", goome::S_ITEM_H),
        ("GOOME_S_MENU_ITEM_H", goome::S_MENU_ITEM_H),
        ("GOOME_S_TAB_H", goome::S_TAB_H),
        ("GOOME_S_WIN_TITLE_H", goome::S_WIN_TITLE_H),
        ("GOOME_S_APP_TITLE_H", goome::S_APP_TITLE_H),
        ("GOOME_S_MENUBAR_H", goome::S_MENUBAR_H),
        ("GOOME_S_STATUSBAR_H", goome::S_STATUSBAR_H),
        ("GOOME_S_FOCUS_W", goome::S_FOCUS_W),
        ("GOOME_S_FOCUS_GAP", goome::S_FOCUS_GAP),
        ("GOOME_S_SEP_W", goome::S_SEP_W),
        ("GOOME_S_GRIP", goome::S_GRIP),
        ("GOOME_S_INDENT", goome::S_INDENT),
        ("GOOME_S_PLOT_W", goome::S_PLOT_W),
        ("GOOME_S_SPINNER", goome::S_SPINNER),
        ("GOOME_S_CARET_W", goome::S_CARET_W),
        ("GOOME_S_WIN_MIN_W", goome::S_WIN_MIN_W),
        ("GOOME_S_WIN_MIN_H", goome::S_WIN_MIN_H),
        ("GOOME_S_POP_GAP", goome::S_POP_GAP),
        ("GOOME_S_MENU_GAP", goome::S_MENU_GAP),
        ("GOOME_S_TIP_PAD_X", goome::S_TIP_PAD_X),
        ("GOOME_S_TIP_PAD_Y", goome::S_TIP_PAD_Y),
        ("GOOME_S_TIP_X", goome::S_TIP_X),
        ("GOOME_S_TIP_Y", goome::S_TIP_Y),
        ("GOOME_S_TIP_RADIUS", goome::S_TIP_RADIUS),
        ("GOOME_S_SHADOW_X", goome::S_SHADOW_X),
        ("GOOME_S_SHADOW_Y", goome::S_SHADOW_Y),
        ("GOOME_S_ALIGN", goome::S_ALIGN),
        ("GOOME_S_HEADING_SIZE", goome::S_HEADING_SIZE),
        ("GOOME_S_SMALL_SIZE", goome::S_SMALL_SIZE),
        ("GOOME_S_EASE", goome::S_EASE),
        ("GOOME_S_HOVER_MS", goome::S_HOVER_MS),
        ("GOOME_S_PRESS_MS", goome::S_PRESS_MS),
        ("GOOME_S_POPUP_MS", goome::S_POPUP_MS),
        ("GOOME_S_COLLAPSE_MS", goome::S_COLLAPSE_MS),
        ("GOOME_S_TOAST_FADE_MS", goome::S_TOAST_FADE_MS),
        ("GOOME_S_COUNT", goome::S_COUNT),
        ("GOOME_ALIGN_AUTO", goome::ALIGN_AUTO),
        ("GOOME_ALIGN_LEFT", goome::ALIGN_LEFT),
        ("GOOME_ALIGN_CENTER", goome::ALIGN_CENTER),
        ("GOOME_ALIGN_RIGHT", goome::ALIGN_RIGHT),
        ("GOOME_EASE_SMOOTH", goome::EASE_SMOOTH),
        ("GOOME_EASE_LINEAR", goome::EASE_LINEAR),
        ("GOOME_EASE_OUT", goome::EASE_OUT),
        ("GOOME_EASE_IN_OUT", goome::EASE_IN_OUT),
        ("GOOME_EASE_BACK", goome::EASE_BACK),
        ("GOOME_EASE_CUSTOM", goome::EASE_CUSTOM),
        ("GOOME_EASE_IN", goome::EASE_IN),
        ("GOOME_EASE_SPRING", goome::EASE_SPRING),
        ("GOOME_EASE_BOUNCE", goome::EASE_BOUNCE),
        ("GOOME_SKIN_BUTTON", goome::SKIN_BUTTON),
        ("GOOME_SKIN_PANEL", goome::SKIN_PANEL),
        ("GOOME_SKIN_INPUT", goome::SKIN_INPUT),
        ("GOOME_SKIN_CHECKBOX", goome::SKIN_CHECKBOX),
        ("GOOME_SKIN_RADIO", goome::SKIN_RADIO),
        ("GOOME_SKIN_TOGGLE", goome::SKIN_TOGGLE),
        ("GOOME_SKIN_SLIDER_TRACK", goome::SKIN_SLIDER_TRACK),
        ("GOOME_SKIN_SLIDER_FILL", goome::SKIN_SLIDER_FILL),
        ("GOOME_SKIN_SCROLLBAR", goome::SKIN_SCROLLBAR),
        ("GOOME_SKIN_TAB", goome::SKIN_TAB),
        ("GOOME_SKIN_TITLEBAR", goome::SKIN_TITLEBAR),
        ("GOOME_SKIN_TOOLTIP", goome::SKIN_TOOLTIP),
        ("GOOME_SKIN_MENU", goome::SKIN_MENU),
        ("GOOME_SKIN_POPUP", goome::SKIN_POPUP),
        ("GOOME_SKIN_COUNT", goome::SKIN_COUNT),
        ("GOOME_ITEM_DRAG_X", goome::ITEM_DRAG_X),
        ("GOOME_ITEM_DRAG_Y", goome::ITEM_DRAG_Y),
        ("GOOME_ITEM_FOCUSABLE", goome::ITEM_FOCUSABLE),
        ("GOOME_ITEM_NO_HIT", goome::ITEM_NO_HIT),
        ("GOOME_ITEM_REPEAT", goome::ITEM_REPEAT),
        ("GOOME_PRESET_DARK", goome::PRESET_DARK),
        ("GOOME_PRESET_LIGHT", goome::PRESET_LIGHT),
        ("GOOME_PRESET_MIDNIGHT", goome::PRESET_MIDNIGHT),
        ("GOOME_PRESET_PAPER", goome::PRESET_PAPER),
        ("GOOME_PRESET_MINT", goome::PRESET_MINT),
        ("GOOME_PRESET_HIGH_CONTRAST", goome::PRESET_HIGH_CONTRAST),
        ("GOOME_PRESET_ROSE", goome::PRESET_ROSE),
        ("GOOME_PRESET_COUNT", goome::PRESET_COUNT),
        ("GOOME_ACT_COPY", goome::ACT_COPY),
        ("GOOME_ACT_CUT", goome::ACT_CUT),
        ("GOOME_ACT_PASTE", goome::ACT_PASTE),
        ("GOOME_ACT_UNDO", goome::ACT_UNDO),
        ("GOOME_ACT_REDO", goome::ACT_REDO),
        ("GOOME_ACT_SELECT_ALL", goome::ACT_SELECT_ALL),
        ("GOOME_ACT_WORD_LEFT", goome::ACT_WORD_LEFT),
        ("GOOME_ACT_WORD_RIGHT", goome::ACT_WORD_RIGHT),
        ("GOOME_ACT_DEL_WORD_LEFT", goome::ACT_DEL_WORD_LEFT),
        ("GOOME_ACT_DEL_WORD_RIGHT", goome::ACT_DEL_WORD_RIGHT),
        ("GOOME_ACT_DOC_START", goome::ACT_DOC_START),
        ("GOOME_ACT_DOC_END", goome::ACT_DOC_END),
        ("GOOME_ACT_COUNT", goome::ACT_COUNT),
        ("GOOME_CUR_BUTTON", goome::CUR_BUTTON),
        ("GOOME_CUR_INPUT", goome::CUR_INPUT),
        ("GOOME_CUR_LINK", goome::CUR_LINK),
        ("GOOME_CUR_DRAG_VALUE", goome::CUR_DRAG_VALUE),
        ("GOOME_CUR_SPLIT_X", goome::CUR_SPLIT_X),
        ("GOOME_CUR_SPLIT_Y", goome::CUR_SPLIT_Y),
        ("GOOME_CUR_GRIP", goome::CUR_GRIP),
        ("GOOME_CUR_COLUMN", goome::CUR_COLUMN),
        ("GOOME_CUR_DRAG_DROP", goome::CUR_DRAG_DROP),
        ("GOOME_CUR_CUSTOM_X", goome::CUR_CUSTOM_X),
        ("GOOME_CUR_CUSTOM_Y", goome::CUR_CUSTOM_Y),
        ("GOOME_CUR_CUSTOM_XY", goome::CUR_CUSTOM_XY),
        ("GOOME_CUR_COUNT", goome::CUR_COUNT),
        ("GOOME_ROLE_NONE", goome::ROLE_NONE),
        ("GOOME_ROLE_WINDOW", goome::ROLE_WINDOW),
        ("GOOME_ROLE_BUTTON", goome::ROLE_BUTTON),
        ("GOOME_ROLE_CHECKBOX", goome::ROLE_CHECKBOX),
        ("GOOME_ROLE_RADIO", goome::ROLE_RADIO),
        ("GOOME_ROLE_SLIDER", goome::ROLE_SLIDER),
        ("GOOME_ROLE_TEXT", goome::ROLE_TEXT),
        ("GOOME_ROLE_INPUT", goome::ROLE_INPUT),
        ("GOOME_ROLE_COMBO", goome::ROLE_COMBO),
        ("GOOME_ROLE_LIST", goome::ROLE_LIST),
        ("GOOME_ROLE_ITEM", goome::ROLE_ITEM),
        ("GOOME_ROLE_TAB", goome::ROLE_TAB),
        ("GOOME_ROLE_MENU", goome::ROLE_MENU),
        ("GOOME_ROLE_MENUITEM", goome::ROLE_MENUITEM),
        ("GOOME_ROLE_TABLE", goome::ROLE_TABLE),
        ("GOOME_ROLE_TREE", goome::ROLE_TREE),
        ("GOOME_ROLE_PROGRESS", goome::ROLE_PROGRESS),
        ("GOOME_ROLE_IMAGE", goome::ROLE_IMAGE),
        ("GOOME_ROLE_GROUP", goome::ROLE_GROUP),
        ("GOOME_ROLE_DIALOG", goome::ROLE_DIALOG),
        ("GOOME_A11Y_FOCUSED", goome::A11Y_FOCUSED),
        ("GOOME_A11Y_CHECKED", goome::A11Y_CHECKED),
        ("GOOME_A11Y_DISABLED", goome::A11Y_DISABLED),
        ("GOOME_A11Y_EXPANDED", goome::A11Y_EXPANDED),
        ("GOOME_A11Y_SELECTED", goome::A11Y_SELECTED),
        ("GOOME_DIR_AUTO", goome::DIR_AUTO),
        ("GOOME_DIR_LTR", goome::DIR_LTR),
        ("GOOME_DIR_RTL", goome::DIR_RTL),
    ]
}

#[test]
fn constants_match_c() {
    let c = goome_ref::consts();
    let mine: HashMap<&str, i32> = mine_consts().into_iter().collect();
    assert!(c.len() > 250);
    for (name, v) in &c {
        assert_eq!(mine.get(name.as_str()), Some(v), "{}", name);
    }
    let h = header();
    let mut count = 0;
    let mut rest = h.as_str();
    while let Some(i) = rest.find("enum {") {
        let body = &rest[i + 6..];
        let end = body.find('}').unwrap();
        for part in body[..end].lines().map(|l| l.split("//").next().unwrap()).flat_map(|l| l.split(',')) {
            let name = part.split('=').next().unwrap().trim();
            if name.starts_with("GOOME_") {
                assert!(mine.contains_key(name), "{} isnt mirrored", name);
                count += 1;
            }
        }
        rest = &body[end..];
    }
    assert_eq!(count, c.len());
}

#[test]
fn every_function_is_declared() {
    let h = header();
    let sys_src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../goome-sys/src/lib.rs")).unwrap();
    let mut missing = Vec::new();
    let mut seen = 0;
    let b = h.as_bytes();
    let mut i = 0;
    while let Some(j) = h[i..].find("goome_") {
        let s = i + j;
        let mut e = s;
        while e < b.len() && (b[e].is_ascii_alphanumeric() || b[e] == b'_') {
            e += 1;
        }
        let name = &h[s..e];
        let after = h[e..].trim_start();
        let before_ok = s == 0 || !(b[s - 1].is_ascii_alphanumeric() || b[s - 1] == b'_');
        if before_ok && after.starts_with('(') && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            seen += 1;
            if !sys_src.contains(&format!("pub fn {}(", name)) {
                missing.push(name.to_string());
            }
        }
        i = e;
    }
    assert!(seen > 150);
    assert_eq!(missing, Vec::<String>::new());
}

#[test]
fn version_matches_header() {
    assert!(header().contains(&format!("#define GOOME_VERSION \"{}\"", goome::VERSION)));
}
