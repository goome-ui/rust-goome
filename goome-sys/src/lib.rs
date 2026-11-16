#![no_std]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_double, c_int, c_long, c_void};

pub const GOOME_VERSION: &str = "0.5.0";

pub enum goome {}
pub enum goome_font {}
pub enum goome_ui {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_opts {
    pub title: *const c_char,
    pub backend: c_int,
    pub state_dir: *const c_char,
    pub no_capture: c_int,
    pub capture_fps: c_int,
    pub hide_self: c_int,
    pub test_frames: *const c_char,
    pub test_script: *const c_char,
    pub test_shots: *const c_char,
    pub mode: c_int,
    pub width: c_int,
    pub height: c_int,
    pub min_width: c_int,
    pub min_height: c_int,
    pub window_flags: c_int,
    pub scale: c_double,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct goome_rect {
    pub x: c_int,
    pub y: c_int,
    pub w: c_int,
    pub h: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_monitor {
    pub id: c_int,
    pub name: [c_char; 64],
    pub area: goome_rect,
    pub scale: c_double,
    pub pw: c_int,
    pub ph: c_int,
    pub rotation: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_image {
    pub px: *mut u32,
    pub w: c_int,
    pub h: c_int,
    pub stride: c_int,
    pub area: goome_rect,
    pub mon: c_int,
    pub has_self: c_int,
    pub has_cursor: c_int,
    pub cursor: goome_rect,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_canvas {
    pub px: *mut u32,
    pub w: c_int,
    pub h: c_int,
    pub stride: c_int,
    pub ox: c_double,
    pub oy: c_double,
    pub scale: c_double,
    pub mon: c_int,
    pub dx0: c_int,
    pub dy0: c_int,
    pub dx1: c_int,
    pub dy1: c_int,
    pub cx0: c_int,
    pub cy0: c_int,
    pub cx1: c_int,
    pub cy1: c_int,
    pub kx0: c_int,
    pub ky0: c_int,
    pub kx1: c_int,
    pub ky1: c_int,
    pub nclip: c_int,
    pub clips: [[c_int; 4]; 16],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_event {
    pub type_: c_int,
    pub x: c_double,
    pub y: c_double,
    pub mon: c_int,
    pub button: c_int,
    pub dy: c_int,
    pub key: c_int,
    pub mods: c_int,
    pub text: [c_char; 8],
    pub id: c_int,
    pub msg: [c_char; 64],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct goome_pacer {
    pub next: c_long,
    pub fps: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct goome_radii {
    pub tl: c_double,
    pub tr: c_double,
    pub br: c_double,
    pub bl: c_double,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_bitmap {
    pub px: *mut u32,
    pub w: c_int,
    pub h: c_int,
    pub stride: c_int,
    pub owned: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_skin {
    pub bm: *const goome_bitmap,
    pub l: c_double,
    pub t: c_double,
    pub r: c_double,
    pub b: c_double,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_theme {
    pub bg: u32,
    pub panel: u32,
    pub border: u32,
    pub text: u32,
    pub text_dim: u32,
    pub accent: u32,
    pub accent_text: u32,
    pub hover: u32,
    pub active: u32,
    pub input_bg: u32,
    pub shadow: u32,
    pub radius: c_double,
    pub pad: c_double,
    pub gap: c_double,
    pub border_width: c_double,
    pub font_size: c_double,
    pub shadow_blur: c_double,
    pub anim_ms: c_double,
    pub font: *mut goome_font,
    pub skin_button: goome_skin,
    pub skin_panel: goome_skin,
    pub skin_input: goome_skin,
    pub icon: [*const goome_bitmap; GOOME_ICON_COUNT as usize],
    pub icon_tint: u32,
    pub skin_checkbox: goome_skin,
    pub skin_radio: goome_skin,
    pub skin_toggle: goome_skin,
    pub skin_slider: goome_skin,
    pub skin_slider_fill: goome_skin,
    pub skin_scrollbar: goome_skin,
    pub skin_tab: goome_skin,
    pub skin_titlebar: goome_skin,
    pub skin_tooltip: goome_skin,
    pub skin_menu: goome_skin,
    pub skin_popup: goome_skin,
    pub focus: u32,
    pub select: u32,
    pub caret: u32,
    pub link: u32,
    pub badge: u32,
    pub toast: u32,
    pub tooltip: u32,
    pub danger: u32,
    pub knob: u32,
    pub dim: u32,
    pub check_size: c_double,
    pub check_radius: c_double,
    pub toggle_w: c_double,
    pub toggle_h: c_double,
    pub slider_h: c_double,
    pub slider_track: c_double,
    pub knob_size: c_double,
    pub scroll_w: c_double,
    pub scroll_min: c_double,
    pub item_h: c_double,
    pub menu_item_h: c_double,
    pub tab_h: c_double,
    pub win_title_h: c_double,
    pub app_title_h: c_double,
    pub menubar_h: c_double,
    pub statusbar_h: c_double,
    pub focus_w: c_double,
    pub focus_gap: c_double,
    pub sep_w: c_double,
    pub grip: c_double,
    pub indent: c_double,
    pub plot_w: c_double,
    pub spinner: c_double,
    pub caret_w: c_double,
    pub win_min_w: c_double,
    pub win_min_h: c_double,
    pub pop_gap: c_double,
    pub menu_gap: c_double,
    pub tip_pad_x: c_double,
    pub tip_pad_y: c_double,
    pub tip_x: c_double,
    pub tip_y: c_double,
    pub tip_radius: c_double,
    pub shadow_x: c_double,
    pub shadow_y: c_double,
    pub align: c_double,
    pub heading_size: c_double,
    pub small_size: c_double,
    pub ease: c_double,
    pub hover_ms: c_double,
    pub press_ms: c_double,
    pub popup_ms: c_double,
    pub collapse_ms: c_double,
    pub toast_fade_ms: c_double,
    pub font_heading: *mut goome_font,
    pub font_mono: *mut goome_font,
    pub font_small: *mut goome_font,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct goome_ui_item {
    pub x: c_double,
    pub y: c_double,
    pub w: c_double,
    pub h: c_double,
    pub hovered: c_int,
    pub pressed: c_int,
    pub held: c_int,
    pub released: c_int,
    pub clicked: c_int,
    pub double_clicked: c_int,
    pub right_clicked: c_int,
    pub focused: c_int,
    pub mx: c_double,
    pub my: c_double,
    pub dx: c_double,
    pub dy: c_double,
    pub drag_x: c_double,
    pub drag_y: c_double,
    pub key: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct goome_combo {
    pub key: c_int,
    pub mods: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_ui_config {
    pub tip_delay_ms: c_int,
    pub dbl_click_ms: c_int,
    pub dbl_click_px: c_int,
    pub drag_px: c_int,
    pub repeat_delay_ms: c_int,
    pub repeat_ms: c_int,
    pub blink_ms: c_int,
    pub menu_delay_ms: c_int,
    pub toast_ms: c_int,
    pub scroll_lines: c_double,
    pub raise_on_click: c_int,
    pub esc_closes: c_int,
    pub tab_nav: c_int,
    pub focus_on_mouse: c_int,
    pub cursor: [c_int; GOOME_CUR_COUNT as usize],
    pub keys: [[goome_combo; 4]; GOOME_ACT_COUNT as usize],
    pub ease: Option<unsafe extern "C" fn(t: c_double, user: *mut c_void) -> c_double>,
    pub ease_user: *mut c_void,
    pub mask: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct goome_a11y_node {
    pub role: c_int,
    pub parent: c_int,
    pub states: c_int,
    pub x: c_double,
    pub y: c_double,
    pub w: c_double,
    pub h: c_double,
    pub label: [c_char; 96],
    pub value: [c_char; 64],
    pub id: u32,
    pub num: c_double,
    pub min: c_double,
    pub max: c_double,
    pub caret: c_int,
}

pub const GOOME_AUTO: c_int = 0;
pub const GOOME_WAYLAND: c_int = 1;
pub const GOOME_GNOME: c_int = 2;
pub const GOOME_PORTAL: c_int = 3;
pub const GOOME_X11: c_int = 4;
pub const GOOME_WIN32: c_int = 5;
pub const GOOME_MACOS: c_int = 6;
pub const GOOME_TEST: c_int = 7;
pub const GOOME_OFFSCREEN: c_int = 8;
pub const GOOME_MODE_OVERLAY: c_int = 0;
pub const GOOME_MODE_WINDOW: c_int = 1;
pub const GOOME_MODE_OFFSCREEN: c_int = 2;
pub const GOOME_WINDOW_RESIZABLE: c_int = 1;
pub const GOOME_WINDOW_BORDERLESS: c_int = 2;
pub const GOOME_WINDOW_TRANSPARENT: c_int = 4;
pub const GOOME_WINDOW_ON_TOP: c_int = 8;
pub const GOOME_WINDOW_HIDDEN: c_int = 16;
pub const GOOME_WINDOW_CENTER: c_int = 32;
pub const GOOME_OK: c_int = 0;
pub const GOOME_ENOTSUP: c_int = -1;
pub const GOOME_EDENIED: c_int = -2;
pub const GOOME_ENODISPLAY: c_int = -3;
pub const GOOME_EGONE: c_int = -4;
pub const GOOME_EAGAIN: c_int = -5;
pub const GOOME_ENOMEM: c_int = -6;
pub const GOOME_EARG: c_int = -7;
pub const GOOME_CAP_CAPTURE: c_int = 1;
pub const GOOME_CAP_GLOBAL_POINTER: c_int = 2;
pub const GOOME_CAP_HOTKEYS: c_int = 4;
pub const GOOME_CAP_HIDE_SELF: c_int = 8;
pub const GOOME_CAP_KEYBOARD: c_int = 16;
pub const GOOME_CAP_WINDOW: c_int = 32;
pub const GOOME_CAP_CLIPBOARD: c_int = 64;
pub const GOOME_CAP_CURSOR: c_int = 128;
pub const GOOME_CAP_GRAB: c_int = 256;
pub const GOOME_WANT_MOUSE: c_int = 1;
pub const GOOME_WANT_KEYBOARD: c_int = 2;
pub const GOOME_NONE: c_int = 0;
pub const GOOME_DOWN: c_int = 1;
pub const GOOME_UP: c_int = 2;
pub const GOOME_MOTION: c_int = 3;
pub const GOOME_SCROLL: c_int = 4;
pub const GOOME_KEY: c_int = 5;
pub const GOOME_HOTKEY: c_int = 6;
pub const GOOME_CONTROL: c_int = 7;
pub const GOOME_MONITORS: c_int = 8;
pub const GOOME_QUIT: c_int = 9;
pub const GOOME_CLOSE: c_int = 10;
pub const GOOME_FOCUS: c_int = 11;
pub const GOOME_DROP: c_int = 12;
pub const GOOME_TEXT: c_int = 13;
pub const GOOME_PREEDIT: c_int = 14;
pub const GOOME_CURSOR_ARROW: c_int = 0;
pub const GOOME_CURSOR_IBEAM: c_int = 1;
pub const GOOME_CURSOR_HAND: c_int = 2;
pub const GOOME_CURSOR_MOVE: c_int = 3;
pub const GOOME_CURSOR_RESIZE_EW: c_int = 4;
pub const GOOME_CURSOR_RESIZE_NS: c_int = 5;
pub const GOOME_CURSOR_RESIZE_NWSE: c_int = 6;
pub const GOOME_CURSOR_RESIZE_NESW: c_int = 7;
pub const GOOME_CURSOR_WAIT: c_int = 8;
pub const GOOME_CURSOR_NO: c_int = 9;
pub const GOOME_CURSOR_HIDDEN: c_int = 10;
pub const GOOME_EDGE_LEFT: c_int = 1;
pub const GOOME_EDGE_RIGHT: c_int = 2;
pub const GOOME_EDGE_TOP: c_int = 4;
pub const GOOME_EDGE_BOTTOM: c_int = 8;
pub const GOOME_STATE_NORMAL: c_int = 0;
pub const GOOME_STATE_MINIMIZED: c_int = 1;
pub const GOOME_STATE_MAXIMIZED: c_int = 2;
pub const GOOME_STATE_FULLSCREEN: c_int = 3;
pub const GOOME_K_BACKSPACE: c_int = 8;
pub const GOOME_K_TAB: c_int = 9;
pub const GOOME_K_ENTER: c_int = 13;
pub const GOOME_K_ESC: c_int = 27;
pub const GOOME_K_SPACE: c_int = 32;
pub const GOOME_K_DELETE: c_int = 127;
pub const GOOME_K_F1: c_int = 129;
pub const GOOME_K_F12: c_int = 140;
pub const GOOME_K_LEFT: c_int = 144;
pub const GOOME_K_RIGHT: c_int = 145;
pub const GOOME_K_UP: c_int = 146;
pub const GOOME_K_DOWN: c_int = 147;
pub const GOOME_K_HOME: c_int = 148;
pub const GOOME_K_END: c_int = 149;
pub const GOOME_K_PGUP: c_int = 150;
pub const GOOME_K_PGDN: c_int = 151;
pub const GOOME_K_INSERT: c_int = 152;
pub const GOOME_SHIFT: c_int = 256;
pub const GOOME_CTRL: c_int = 512;
pub const GOOME_ALT: c_int = 1024;
pub const GOOME_SUPER: c_int = 2048;
pub const GOOME_FMT_BGRA_PREMUL: c_int = 0;
pub const GOOME_FMT_RGBA_PREMUL: c_int = 1;
pub const GOOME_FMT_BGRA: c_int = 2;
pub const GOOME_FMT_RGBA: c_int = 3;
pub const GOOME_ICON_ARROW_DOWN: c_int = 0;
pub const GOOME_ICON_ARROW_RIGHT: c_int = 1;
pub const GOOME_ICON_ARROW_UP: c_int = 2;
pub const GOOME_ICON_ARROW_LEFT: c_int = 3;
pub const GOOME_ICON_CHECK: c_int = 4;
pub const GOOME_ICON_RADIO_DOT: c_int = 5;
pub const GOOME_ICON_CLOSE: c_int = 6;
pub const GOOME_ICON_MINIMIZE: c_int = 7;
pub const GOOME_ICON_MAXIMIZE: c_int = 8;
pub const GOOME_ICON_RESTORE: c_int = 9;
pub const GOOME_ICON_RESIZE: c_int = 10;
pub const GOOME_ICON_SORT_UP: c_int = 11;
pub const GOOME_ICON_SORT_DOWN: c_int = 12;
pub const GOOME_ICON_PLUS: c_int = 13;
pub const GOOME_ICON_MINUS: c_int = 14;
pub const GOOME_ICON_SLIDER_KNOB: c_int = 15;
pub const GOOME_ICON_TOGGLE_KNOB: c_int = 16;
pub const GOOME_ICON_COLLAPSE: c_int = 17;
pub const GOOME_ICON_EXPAND: c_int = 18;
pub const GOOME_ICON_DRAG_HANDLE: c_int = 19;
pub const GOOME_ICON_TOAST: c_int = 20;
pub const GOOME_ICON_PICKER: c_int = 21;
pub const GOOME_ICON_COUNT: c_int = 22;
pub const GOOME_WIN_MOVE: c_int = 1;
pub const GOOME_WIN_RESIZE: c_int = 2;
pub const GOOME_WIN_COLLAPSE: c_int = 4;
pub const GOOME_WIN_NO_TITLE: c_int = 8;
pub const GOOME_WIN_CLOSE: c_int = 16;
pub const GOOME_INPUT_PASSWORD: c_int = 1;
pub const GOOME_INPUT_READONLY: c_int = 2;
pub const GOOME_INPUT_NUMBER: c_int = 4;
pub const GOOME_INPUT_NO_SELECT_ON_FOCUS: c_int = 8;
pub const GOOME_INPUT_ENTER_RETURNS: c_int = 16;
pub const GOOME_APP_TITLEBAR: c_int = 1;
pub const GOOME_APP_MENUBAR: c_int = 2;
pub const GOOME_APP_STATUSBAR: c_int = 4;
pub const GOOME_APP_NO_PAD: c_int = 8;
pub const GOOME_EDIT_READONLY: c_int = 1;
pub const GOOME_EDIT_LINE_NUMBERS: c_int = 2;
pub const GOOME_EDIT_WRAP: c_int = 4;
pub const GOOME_EDIT_MONO: c_int = 8;
pub const GOOME_TABLE_SORTABLE: c_int = 1;
pub const GOOME_TABLE_RESIZABLE: c_int = 2;
pub const GOOME_TABLE_STRIPES: c_int = 4;
pub const GOOME_TABLE_BORDERS: c_int = 8;
pub const GOOME_S_BG: c_int = 0;
pub const GOOME_S_PANEL: c_int = 1;
pub const GOOME_S_BORDER: c_int = 2;
pub const GOOME_S_TEXT: c_int = 3;
pub const GOOME_S_TEXT_DIM: c_int = 4;
pub const GOOME_S_ACCENT: c_int = 5;
pub const GOOME_S_ACCENT_TEXT: c_int = 6;
pub const GOOME_S_HOVER: c_int = 7;
pub const GOOME_S_ACTIVE: c_int = 8;
pub const GOOME_S_INPUT_BG: c_int = 9;
pub const GOOME_S_SHADOW: c_int = 10;
pub const GOOME_S_RADIUS: c_int = 11;
pub const GOOME_S_PAD: c_int = 12;
pub const GOOME_S_GAP: c_int = 13;
pub const GOOME_S_BORDER_WIDTH: c_int = 14;
pub const GOOME_S_FONT_SIZE: c_int = 15;
pub const GOOME_S_SHADOW_BLUR: c_int = 16;
pub const GOOME_S_ANIM_MS: c_int = 17;
pub const GOOME_S_FOCUS: c_int = 18;
pub const GOOME_S_SELECT: c_int = 19;
pub const GOOME_S_CARET: c_int = 20;
pub const GOOME_S_LINK: c_int = 21;
pub const GOOME_S_BADGE: c_int = 22;
pub const GOOME_S_TOAST: c_int = 23;
pub const GOOME_S_TOOLTIP: c_int = 24;
pub const GOOME_S_DANGER: c_int = 25;
pub const GOOME_S_KNOB: c_int = 26;
pub const GOOME_S_DIM: c_int = 27;
pub const GOOME_S_CHECK_SIZE: c_int = 28;
pub const GOOME_S_CHECK_RADIUS: c_int = 29;
pub const GOOME_S_TOGGLE_W: c_int = 30;
pub const GOOME_S_TOGGLE_H: c_int = 31;
pub const GOOME_S_SLIDER_H: c_int = 32;
pub const GOOME_S_SLIDER_TRACK: c_int = 33;
pub const GOOME_S_KNOB_SIZE: c_int = 34;
pub const GOOME_S_SCROLL_W: c_int = 35;
pub const GOOME_S_SCROLL_MIN: c_int = 36;
pub const GOOME_S_ITEM_H: c_int = 37;
pub const GOOME_S_MENU_ITEM_H: c_int = 38;
pub const GOOME_S_TAB_H: c_int = 39;
pub const GOOME_S_WIN_TITLE_H: c_int = 40;
pub const GOOME_S_APP_TITLE_H: c_int = 41;
pub const GOOME_S_MENUBAR_H: c_int = 42;
pub const GOOME_S_STATUSBAR_H: c_int = 43;
pub const GOOME_S_FOCUS_W: c_int = 44;
pub const GOOME_S_FOCUS_GAP: c_int = 45;
pub const GOOME_S_SEP_W: c_int = 46;
pub const GOOME_S_GRIP: c_int = 47;
pub const GOOME_S_INDENT: c_int = 48;
pub const GOOME_S_PLOT_W: c_int = 49;
pub const GOOME_S_SPINNER: c_int = 50;
pub const GOOME_S_CARET_W: c_int = 51;
pub const GOOME_S_WIN_MIN_W: c_int = 52;
pub const GOOME_S_WIN_MIN_H: c_int = 53;
pub const GOOME_S_POP_GAP: c_int = 54;
pub const GOOME_S_MENU_GAP: c_int = 55;
pub const GOOME_S_TIP_PAD_X: c_int = 56;
pub const GOOME_S_TIP_PAD_Y: c_int = 57;
pub const GOOME_S_TIP_X: c_int = 58;
pub const GOOME_S_TIP_Y: c_int = 59;
pub const GOOME_S_TIP_RADIUS: c_int = 60;
pub const GOOME_S_SHADOW_X: c_int = 61;
pub const GOOME_S_SHADOW_Y: c_int = 62;
pub const GOOME_S_ALIGN: c_int = 63;
pub const GOOME_S_HEADING_SIZE: c_int = 64;
pub const GOOME_S_SMALL_SIZE: c_int = 65;
pub const GOOME_S_EASE: c_int = 66;
pub const GOOME_S_HOVER_MS: c_int = 67;
pub const GOOME_S_PRESS_MS: c_int = 68;
pub const GOOME_S_POPUP_MS: c_int = 69;
pub const GOOME_S_COLLAPSE_MS: c_int = 70;
pub const GOOME_S_TOAST_FADE_MS: c_int = 71;
pub const GOOME_S_COUNT: c_int = 72;
pub const GOOME_ALIGN_AUTO: c_int = 0;
pub const GOOME_ALIGN_LEFT: c_int = 1;
pub const GOOME_ALIGN_CENTER: c_int = 2;
pub const GOOME_ALIGN_RIGHT: c_int = 3;
pub const GOOME_EASE_SMOOTH: c_int = 0;
pub const GOOME_EASE_LINEAR: c_int = 1;
pub const GOOME_EASE_OUT: c_int = 2;
pub const GOOME_EASE_IN_OUT: c_int = 3;
pub const GOOME_EASE_BACK: c_int = 4;
pub const GOOME_EASE_CUSTOM: c_int = 5;
pub const GOOME_EASE_IN: c_int = 6;
pub const GOOME_EASE_SPRING: c_int = 7;
pub const GOOME_EASE_BOUNCE: c_int = 8;
pub const GOOME_SKIN_BUTTON: c_int = 0;
pub const GOOME_SKIN_PANEL: c_int = 1;
pub const GOOME_SKIN_INPUT: c_int = 2;
pub const GOOME_SKIN_CHECKBOX: c_int = 3;
pub const GOOME_SKIN_RADIO: c_int = 4;
pub const GOOME_SKIN_TOGGLE: c_int = 5;
pub const GOOME_SKIN_SLIDER_TRACK: c_int = 6;
pub const GOOME_SKIN_SLIDER_FILL: c_int = 7;
pub const GOOME_SKIN_SCROLLBAR: c_int = 8;
pub const GOOME_SKIN_TAB: c_int = 9;
pub const GOOME_SKIN_TITLEBAR: c_int = 10;
pub const GOOME_SKIN_TOOLTIP: c_int = 11;
pub const GOOME_SKIN_MENU: c_int = 12;
pub const GOOME_SKIN_POPUP: c_int = 13;
pub const GOOME_SKIN_COUNT: c_int = 14;
pub const GOOME_ITEM_DRAG_X: c_int = 1;
pub const GOOME_ITEM_DRAG_Y: c_int = 2;
pub const GOOME_ITEM_FOCUSABLE: c_int = 4;
pub const GOOME_ITEM_NO_HIT: c_int = 8;
pub const GOOME_ITEM_REPEAT: c_int = 16;
pub const GOOME_PRESET_DARK: c_int = 0;
pub const GOOME_PRESET_LIGHT: c_int = 1;
pub const GOOME_PRESET_MIDNIGHT: c_int = 2;
pub const GOOME_PRESET_PAPER: c_int = 3;
pub const GOOME_PRESET_MINT: c_int = 4;
pub const GOOME_PRESET_HIGH_CONTRAST: c_int = 5;
pub const GOOME_PRESET_ROSE: c_int = 6;
pub const GOOME_PRESET_COUNT: c_int = 7;
pub const GOOME_ACT_COPY: c_int = 0;
pub const GOOME_ACT_CUT: c_int = 1;
pub const GOOME_ACT_PASTE: c_int = 2;
pub const GOOME_ACT_UNDO: c_int = 3;
pub const GOOME_ACT_REDO: c_int = 4;
pub const GOOME_ACT_SELECT_ALL: c_int = 5;
pub const GOOME_ACT_WORD_LEFT: c_int = 6;
pub const GOOME_ACT_WORD_RIGHT: c_int = 7;
pub const GOOME_ACT_DEL_WORD_LEFT: c_int = 8;
pub const GOOME_ACT_DEL_WORD_RIGHT: c_int = 9;
pub const GOOME_ACT_DOC_START: c_int = 10;
pub const GOOME_ACT_DOC_END: c_int = 11;
pub const GOOME_ACT_COUNT: c_int = 12;
pub const GOOME_CUR_BUTTON: c_int = 0;
pub const GOOME_CUR_INPUT: c_int = 1;
pub const GOOME_CUR_LINK: c_int = 2;
pub const GOOME_CUR_DRAG_VALUE: c_int = 3;
pub const GOOME_CUR_SPLIT_X: c_int = 4;
pub const GOOME_CUR_SPLIT_Y: c_int = 5;
pub const GOOME_CUR_GRIP: c_int = 6;
pub const GOOME_CUR_COLUMN: c_int = 7;
pub const GOOME_CUR_DRAG_DROP: c_int = 8;
pub const GOOME_CUR_CUSTOM_X: c_int = 9;
pub const GOOME_CUR_CUSTOM_Y: c_int = 10;
pub const GOOME_CUR_CUSTOM_XY: c_int = 11;
pub const GOOME_CUR_COUNT: c_int = 12;
pub const GOOME_ROLE_NONE: c_int = 0;
pub const GOOME_ROLE_WINDOW: c_int = 1;
pub const GOOME_ROLE_BUTTON: c_int = 2;
pub const GOOME_ROLE_CHECKBOX: c_int = 3;
pub const GOOME_ROLE_RADIO: c_int = 4;
pub const GOOME_ROLE_SLIDER: c_int = 5;
pub const GOOME_ROLE_TEXT: c_int = 6;
pub const GOOME_ROLE_INPUT: c_int = 7;
pub const GOOME_ROLE_COMBO: c_int = 8;
pub const GOOME_ROLE_LIST: c_int = 9;
pub const GOOME_ROLE_ITEM: c_int = 10;
pub const GOOME_ROLE_TAB: c_int = 11;
pub const GOOME_ROLE_MENU: c_int = 12;
pub const GOOME_ROLE_MENUITEM: c_int = 13;
pub const GOOME_ROLE_TABLE: c_int = 14;
pub const GOOME_ROLE_TREE: c_int = 15;
pub const GOOME_ROLE_PROGRESS: c_int = 16;
pub const GOOME_ROLE_IMAGE: c_int = 17;
pub const GOOME_ROLE_GROUP: c_int = 18;
pub const GOOME_ROLE_DIALOG: c_int = 19;
pub const GOOME_A11Y_FOCUSED: c_int = 1;
pub const GOOME_A11Y_CHECKED: c_int = 2;
pub const GOOME_A11Y_DISABLED: c_int = 4;
pub const GOOME_A11Y_EXPANDED: c_int = 8;
pub const GOOME_A11Y_SELECTED: c_int = 16;
pub const GOOME_DIR_AUTO: c_int = 0;
pub const GOOME_DIR_LTR: c_int = 1;
pub const GOOME_DIR_RTL: c_int = 2;

extern "C" {
    pub fn goome_open(o: *const goome_opts, err: *mut c_int) -> *mut goome;
    pub fn goome_open_app(title: *const c_char, w: c_int, h: c_int, err: *mut c_int) -> *mut goome;
    pub fn goome_close(g: *mut goome);
    pub fn goome_error(g: *mut goome) -> *const c_char;
    pub fn goome_strerror(err: c_int) -> *const c_char;
    pub fn goome_backend_name(g: *mut goome) -> *const c_char;
    pub fn goome_caps(g: *mut goome) -> c_int;
    pub fn goome_monitors(g: *mut goome, out: *mut goome_monitor, max: c_int) -> c_int;
    pub fn goome_capture(g: *mut goome, mon: c_int, r: *const goome_rect, img: *mut goome_image) -> c_int;
    pub fn goome_alpha_at(g: *mut goome, x: c_double, y: c_double) -> c_int;
    pub fn goome_begin(g: *mut goome, mon: c_int) -> *mut goome_canvas;
    pub fn goome_end(g: *mut goome, c: *mut goome_canvas, hits: *const goome_rect, nhits: c_int) -> c_int;
    pub fn goome_damage(c: *mut goome_canvas, r: *const goome_rect);
    pub fn goome_poll(g: *mut goome, ev: *mut goome_event, timeout_ms: c_int) -> c_int;
    pub fn goome_pointer(g: *mut goome, x: *mut c_double, y: *mut c_double, buttons: *mut c_int) -> c_int;
    pub fn goome_keyboard(g: *mut goome, on: c_int) -> c_int;
    pub fn goome_grab(g: *mut goome, on: c_int) -> c_int;
    pub fn goome_grabbed(g: *mut goome) -> c_int;
    pub fn goome_hotkey(g: *mut goome, combo: *const c_char, id: c_int) -> c_int;
    pub fn goome_parse_combo(combo: *const c_char, key: *mut c_int, mods: *mut c_int) -> c_int;
    pub fn goome_control_path(g: *mut goome) -> *const c_char;
    pub fn goome_send(state_dir: *const c_char, msg: *const c_char) -> c_int;
    pub fn goome_pace(p: *mut goome_pacer, fps: c_int) -> c_int;
    pub fn goome_now_ms() -> c_long;
    pub fn goome_mode(g: *mut goome) -> c_int;
    pub fn goome_window_title(g: *mut goome, title: *const c_char) -> c_int;
    pub fn goome_window_size(g: *mut goome, w: c_int, h: c_int) -> c_int;
    pub fn goome_window_pos(g: *mut goome, x: *mut c_int, y: *mut c_int, set: c_int) -> c_int;
    pub fn goome_window_state(g: *mut goome, state: c_int) -> c_int;
    pub fn goome_window_get_state(g: *mut goome) -> c_int;
    pub fn goome_window_drag(g: *mut goome, edges: c_int) -> c_int;
    pub fn goome_window_show(g: *mut goome, on: c_int) -> c_int;
    pub fn goome_cursor(g: *mut goome, shape: c_int) -> c_int;
    pub fn goome_cursor_shape(g: *mut goome) -> c_int;
    pub fn goome_clipboard_set(g: *mut goome, utf8: *const c_char) -> c_int;
    pub fn goome_clipboard_get(g: *mut goome, buf: *mut c_char, cap: c_int) -> c_int;
    pub fn goome_drop_path(g: *mut goome) -> *const c_char;
    pub fn goome_event_text(g: *mut goome) -> *const c_char;
    pub fn goome_preedit_cursor(g: *mut goome) -> c_int;
    pub fn goome_text_input(g: *mut goome, on: c_int, caret: *const goome_rect) -> c_int;
    pub fn goome_push(g: *mut goome, ev: *const goome_event) -> c_int;
    pub fn goome_resize(g: *mut goome, w: c_int, h: c_int, scale: c_double) -> c_int;
    pub fn goome_frame(g: *mut goome, out: *mut goome_image) -> c_int;
    pub fn goome_hit(g: *mut goome, x: c_double, y: c_double) -> c_int;
    pub fn goome_frame_dirty(g: *mut goome, r: *mut goome_rect) -> c_int;
    pub fn goome_frame_copy(g: *mut goome, dst: *mut c_void, dst_stride_bytes: c_int, format: c_int, r: *const goome_rect) -> c_int;
    pub fn goome_save_ppm(path: *const c_char, px: *const u32, w: c_int, h: c_int, stride: c_int) -> c_int;
    pub fn goome_clear(c: *mut goome_canvas);
    pub fn goome_draw_rect(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double, argb: u32);
    pub fn goome_draw_round_rect(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double, argb: u32);
    pub fn goome_draw_frame(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double, width: c_double, argb: u32);
    pub fn goome_draw_line(c: *mut goome_canvas, x0: c_double, y0: c_double, x1: c_double, y1: c_double, width: c_double, argb: u32);
    pub fn goome_draw_triangle(c: *mut goome_canvas, x0: c_double, y0: c_double, x1: c_double, y1: c_double, x2: c_double, y2: c_double, argb: u32);
    pub fn goome_draw_image(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double, img: *const u32, iw: c_int, ih: c_int);
    pub fn goome_erase(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double);
    pub fn goome_draw_text(c: *mut goome_canvas, x: c_double, y: c_double, s: *const c_char, big: c_int, argb: u32) -> c_double;
    pub fn goome_text_width(c: *const goome_canvas, s: *const c_char, big: c_int) -> c_double;
    pub fn goome_set_font(font: c_int);
    pub fn goome_font_count() -> c_int;
    pub fn goome_canvas_blit(dst: *mut goome_canvas, src: *const goome_canvas, alpha: c_double, dx: c_double, dy: c_double);
    pub fn goome_radius(r: c_double) -> goome_radii;
    pub fn goome_clip_push(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double);
    pub fn goome_clip_pop(c: *mut goome_canvas);
    pub fn goome_draw_rrect(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double, r: goome_radii, argb: u32);
    pub fn goome_draw_rrect_line(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double, r: goome_radii, width: c_double, argb: u32);
    pub fn goome_draw_rrect_grad(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double, r: goome_radii, from: u32, to: u32, sideways: c_int);
    pub fn goome_draw_shadow(c: *mut goome_canvas, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double, blur: c_double, argb: u32);
    pub fn goome_draw_circle(c: *mut goome_canvas, cx: c_double, cy: c_double, r: c_double, argb: u32);
    pub fn goome_draw_ring(c: *mut goome_canvas, cx: c_double, cy: c_double, r: c_double, width: c_double, argb: u32);
    pub fn goome_draw_arc(c: *mut goome_canvas, cx: c_double, cy: c_double, r: c_double, width: c_double, a0: c_double, a1: c_double, argb: u32);
    pub fn goome_draw_poly(c: *mut goome_canvas, xy: *const c_double, n: c_int, argb: u32);
    pub fn goome_bitmap_load(path: *const c_char, out: *mut goome_bitmap) -> c_int;
    pub fn goome_bitmap_load_mem(data: *const c_void, n: usize, out: *mut goome_bitmap) -> c_int;
    pub fn goome_bitmap_error() -> *const c_char;
    pub fn goome_bitmap_from_argb(argb: *const u32, w: c_int, h: c_int, out: *mut goome_bitmap) -> c_int;
    pub fn goome_bitmap_free(b: *mut goome_bitmap);
    pub fn goome_draw_bitmap(c: *mut goome_canvas, b: *const goome_bitmap, x: c_double, y: c_double, w: c_double, h: c_double, r: goome_radii, tint: u32);
    pub fn goome_draw_bitmap_part(
        c: *mut goome_canvas,
        b: *const goome_bitmap,
        sx: c_int,
        sy: c_int,
        sw: c_int,
        sh: c_int,
        x: c_double,
        y: c_double,
        w: c_double,
        h: c_double,
        tint: u32,
    );
    pub fn goome_draw_bitmap_nine(
        c: *mut goome_canvas,
        b: *const goome_bitmap,
        x: c_double,
        y: c_double,
        w: c_double,
        h: c_double,
        l: c_double,
        t: c_double,
        r: c_double,
        bm: c_double,
    );
    pub fn goome_font_load(path: *const c_char) -> *mut goome_font;
    pub fn goome_font_load_index(path: *const c_char, index: c_int) -> *mut goome_font;
    pub fn goome_font_load_mem(data: *const c_void, n: usize, copy: c_int) -> *mut goome_font;
    pub fn goome_font_system(name: *const c_char) -> *mut goome_font;
    pub fn goome_font_free(f: *mut goome_font);
    pub fn goome_text(c: *mut goome_canvas, f: *mut goome_font, size: c_double, x: c_double, y: c_double, utf8: *const c_char, argb: u32) -> c_double;
    pub fn goome_text_measure(f: *mut goome_font, size: c_double, utf8: *const c_char, height: *mut c_double) -> c_double;
    pub fn goome_text_line_height(f: *mut goome_font, size: c_double) -> c_double;
    pub fn goome_font_fallback(f: *mut goome_font, fallback: *mut goome_font);
    pub fn goome_text_dir(dir: c_int);
    pub fn goome_ui_new(g: *mut goome) -> *mut goome_ui;
    pub fn goome_ui_free(u: *mut goome_ui);
    pub fn goome_ui_theme(u: *mut goome_ui) -> *mut goome_theme;
    pub fn goome_ui_theme_dark(t: *mut goome_theme);
    pub fn goome_ui_theme_light(t: *mut goome_theme);
    pub fn goome_ui_feed(u: *mut goome_ui, ev: *const goome_event) -> c_int;
    pub fn goome_ui_begin(u: *mut goome_ui, mon: c_int) -> *mut goome_canvas;
    pub fn goome_ui_end(u: *mut goome_ui);
    pub fn goome_ui_hit(u: *mut goome_ui, x: c_double, y: c_double, w: c_double, h: c_double);
    pub fn goome_ui_animating(u: *mut goome_ui) -> c_int;
    pub fn goome_ui_wants(u: *mut goome_ui) -> c_int;
    pub fn goome_ui_auto_grab(u: *mut goome_ui, wants: c_int);
    pub fn goome_ui_window(
        u: *mut goome_ui,
        title: *const c_char,
        x: *mut c_double,
        y: *mut c_double,
        w: *mut c_double,
        h: *mut c_double,
        flags: c_int,
        open: *mut c_int,
    ) -> c_int;
    pub fn goome_ui_window_end(u: *mut goome_ui);
    pub fn goome_ui_row(u: *mut goome_ui, cols: c_int);
    pub fn goome_ui_same_line(u: *mut goome_ui);
    pub fn goome_ui_space(u: *mut goome_ui, px: c_double);
    pub fn goome_ui_separator(u: *mut goome_ui);
    pub fn goome_ui_width(u: *mut goome_ui, w: c_double);
    pub fn goome_ui_label(u: *mut goome_ui, fmt: *const c_char, ...);
    pub fn goome_ui_heading(u: *mut goome_ui, text: *const c_char);
    pub fn goome_ui_wrapped(u: *mut goome_ui, text: *const c_char);
    pub fn goome_ui_button(u: *mut goome_ui, label: *const c_char) -> c_int;
    pub fn goome_ui_image_button(u: *mut goome_ui, id: *const c_char, b: *const goome_bitmap, w: c_double, h: c_double) -> c_int;
    pub fn goome_ui_image(u: *mut goome_ui, b: *const goome_bitmap, w: c_double, h: c_double, radius: c_double);
    pub fn goome_ui_checkbox(u: *mut goome_ui, label: *const c_char, v: *mut c_int) -> c_int;
    pub fn goome_ui_toggle(u: *mut goome_ui, label: *const c_char, v: *mut c_int) -> c_int;
    pub fn goome_ui_radio(u: *mut goome_ui, label: *const c_char, v: *mut c_int, value: c_int) -> c_int;
    pub fn goome_ui_slider(u: *mut goome_ui, label: *const c_char, v: *mut c_double, min: c_double, max: c_double, fmt: *const c_char) -> c_int;
    pub fn goome_ui_slider_int(u: *mut goome_ui, label: *const c_char, v: *mut c_int, min: c_int, max: c_int) -> c_int;
    pub fn goome_ui_input(u: *mut goome_ui, label: *const c_char, buf: *mut c_char, cap: c_int) -> c_int;
    pub fn goome_ui_input_ex(u: *mut goome_ui, label: *const c_char, buf: *mut c_char, cap: c_int, flags: c_int, hint: *const c_char) -> c_int;
    pub fn goome_ui_combo(u: *mut goome_ui, label: *const c_char, sel: *mut c_int, items: *const *const c_char, n: c_int) -> c_int;
    pub fn goome_ui_selectable(u: *mut goome_ui, label: *const c_char, selected: c_int) -> c_int;
    pub fn goome_ui_tabs(u: *mut goome_ui, id: *const c_char, names: *const *const c_char, n: c_int, sel: *mut c_int) -> c_int;
    pub fn goome_ui_collapsing(u: *mut goome_ui, label: *const c_char, open: *mut c_int) -> c_int;
    pub fn goome_ui_progress(u: *mut goome_ui, frac: c_double, text: *const c_char);
    pub fn goome_ui_color(u: *mut goome_ui, label: *const c_char, argb: *mut u32) -> c_int;
    pub fn goome_ui_tooltip(u: *mut goome_ui, text: *const c_char);
    pub fn goome_ui_hovered(u: *mut goome_ui) -> c_int;
    pub fn goome_ui_scroll_begin(u: *mut goome_ui, id: *const c_char, h: c_double) -> c_int;
    pub fn goome_ui_scroll_end(u: *mut goome_ui);
    pub fn goome_ui_push_id(u: *mut goome_ui, id: *const c_char);
    pub fn goome_ui_pop_id(u: *mut goome_ui);
    pub fn goome_ui_app(u: *mut goome_ui, title: *const c_char, flags: c_int) -> c_int;
    pub fn goome_ui_app_end(u: *mut goome_ui);
    pub fn goome_ui_menubar_begin(u: *mut goome_ui) -> c_int;
    pub fn goome_ui_menubar_end(u: *mut goome_ui);
    pub fn goome_ui_menu_begin(u: *mut goome_ui, label: *const c_char) -> c_int;
    pub fn goome_ui_menu_end(u: *mut goome_ui);
    pub fn goome_ui_menu_item(u: *mut goome_ui, label: *const c_char, shortcut: *const c_char, checked: *mut c_int) -> c_int;
    pub fn goome_ui_menu_separator(u: *mut goome_ui);
    pub fn goome_ui_context_begin(u: *mut goome_ui, id: *const c_char) -> c_int;
    pub fn goome_ui_context_end(u: *mut goome_ui);
    pub fn goome_ui_popup_open(u: *mut goome_ui, id: *const c_char);
    pub fn goome_ui_popup_begin(u: *mut goome_ui, id: *const c_char) -> c_int;
    pub fn goome_ui_popup_end(u: *mut goome_ui);
    pub fn goome_ui_popup_close(u: *mut goome_ui);
    pub fn goome_ui_modal_begin(u: *mut goome_ui, title: *const c_char, open: *mut c_int) -> c_int;
    pub fn goome_ui_modal_end(u: *mut goome_ui);
    pub fn goome_ui_toast(u: *mut goome_ui, text: *const c_char, ms: c_int);
    pub fn goome_ui_statusbar(u: *mut goome_ui, text: *const c_char);
    pub fn goome_ui_focused(u: *mut goome_ui) -> c_int;
    pub fn goome_ui_set_focus(u: *mut goome_ui);
    pub fn goome_ui_text_edit(u: *mut goome_ui, id: *const c_char, buf: *mut c_char, cap: c_int, h: c_double, flags: c_int) -> c_int;
    pub fn goome_ui_table_begin(u: *mut goome_ui, id: *const c_char, cols: c_int, h: c_double, flags: c_int) -> c_int;
    pub fn goome_ui_table_column(u: *mut goome_ui, name: *const c_char, width: c_double);
    pub fn goome_ui_table_sort(u: *mut goome_ui, col: *mut c_int, desc: *mut c_int) -> c_int;
    pub fn goome_ui_table_row(u: *mut goome_ui);
    pub fn goome_ui_table_next(u: *mut goome_ui);
    pub fn goome_ui_table_end(u: *mut goome_ui);
    pub fn goome_ui_tree(u: *mut goome_ui, label: *const c_char, open: *mut c_int) -> c_int;
    pub fn goome_ui_tree_end(u: *mut goome_ui);
    pub fn goome_ui_list_clip(u: *mut goome_ui, count: c_int, row_h: c_double, first: *mut c_int, last: *mut c_int) -> c_int;
    pub fn goome_ui_drag_float(
        u: *mut goome_ui,
        label: *const c_char,
        v: *mut c_double,
        speed: c_double,
        min: c_double,
        max: c_double,
        fmt: *const c_char,
    ) -> c_int;
    pub fn goome_ui_drag_int(u: *mut goome_ui, label: *const c_char, v: *mut c_int, speed: c_double, min: c_int, max: c_int) -> c_int;
    pub fn goome_ui_number(u: *mut goome_ui, label: *const c_char, v: *mut c_double, step: c_double, fmt: *const c_char) -> c_int;
    pub fn goome_ui_plot_lines(u: *mut goome_ui, label: *const c_char, v: *const f32, n: c_int, min: c_double, max: c_double, h: c_double);
    pub fn goome_ui_plot_bars(u: *mut goome_ui, label: *const c_char, v: *const f32, n: c_int, min: c_double, max: c_double, h: c_double);
    pub fn goome_ui_split_begin(u: *mut goome_ui, id: *const c_char, vertical: c_int, frac: *mut c_double) -> c_int;
    pub fn goome_ui_split_next(u: *mut goome_ui);
    pub fn goome_ui_split_end(u: *mut goome_ui);
    pub fn goome_ui_drag_source(u: *mut goome_ui, type_: *const c_char, data: *const c_void, len: c_int) -> c_int;
    pub fn goome_ui_drop_target(u: *mut goome_ui, type_: *const c_char, buf: *mut c_void, cap: c_int) -> c_int;
    pub fn goome_ui_spinner(u: *mut goome_ui, size: c_double);
    pub fn goome_ui_link(u: *mut goome_ui, text: *const c_char) -> c_int;
    pub fn goome_ui_badge(u: *mut goome_ui, text: *const c_char, argb: u32);
    pub fn goome_ui_pos(u: *mut goome_ui, x: c_double, y: c_double);
    pub fn goome_ui_size(u: *mut goome_ui, w: c_double, h: c_double);
    pub fn goome_ui_push_color(u: *mut goome_ui, what: c_int, argb: u32);
    pub fn goome_ui_push_num(u: *mut goome_ui, what: c_int, v: c_double);
    pub fn goome_ui_push_radii(u: *mut goome_ui, r: goome_radii);
    pub fn goome_ui_push_font(u: *mut goome_ui, f: *mut goome_font);
    pub fn goome_ui_push_skin(u: *mut goome_ui, kind: c_int, s: *const goome_skin);
    pub fn goome_ui_push_theme(u: *mut goome_ui, t: *const goome_theme);
    pub fn goome_ui_push_icon(u: *mut goome_ui, icon: c_int, b: *const goome_bitmap);
    pub fn goome_ui_pop(u: *mut goome_ui, n: c_int);
    pub fn goome_ui_custom(u: *mut goome_ui, id: *const c_char, w: c_double, h: c_double, flags: c_int, it: *mut goome_ui_item) -> c_int;
    pub fn goome_ui_canvas(u: *mut goome_ui) -> *mut goome_canvas;
    pub fn goome_ui_store(u: *mut goome_ui, id: *const c_char, n: c_int) -> *mut c_double;
    pub fn goome_ui_theme_make(t: *mut goome_theme, accent: u32, dark: c_int, radius: c_double);
    pub fn goome_ui_theme_preset(t: *mut goome_theme, preset: c_int) -> c_int;
    pub fn goome_ui_scale(u: *mut goome_ui, s: c_double);
    pub fn goome_ui_cfg(u: *mut goome_ui) -> *mut goome_ui_config;
    pub fn goome_ui_config_default(c: *mut goome_ui_config);
    pub fn goome_ui_bind(u: *mut goome_ui, act: c_int, combos: *const c_char) -> c_int;
    pub fn goome_ease(ease: c_int, t: c_double) -> c_double;
    pub fn goome_ui_anim(u: *mut goome_ui, id: *const c_char, target: c_double, ms: c_double, ease: c_int) -> c_double;
    pub fn goome_ui_anim_color(u: *mut goome_ui, id: *const c_char, target: u32, ms: c_double, ease: c_int) -> u32;
    pub fn goome_ui_a11y(u: *mut goome_ui, on: c_int);
    pub fn goome_ui_a11y_role(u: *mut goome_ui, role: c_int, label: *const c_char);
    pub fn goome_ui_a11y_value(u: *mut goome_ui, value: *const c_char);
    pub fn goome_a11y_tree(u: *mut goome_ui, out: *mut goome_a11y_node, max: c_int) -> c_int;
}
