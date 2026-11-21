use goome_sys as sys;

/// Backend: pick one by itself.
pub const AUTO: i32 = sys::GOOME_AUTO;
/// Backend: wayland.
pub const WAYLAND: i32 = sys::GOOME_WAYLAND;
/// Backend: gnome.
pub const GNOME: i32 = sys::GOOME_GNOME;
/// Backend: xdg portal.
pub const PORTAL: i32 = sys::GOOME_PORTAL;
/// Backend: x11.
pub const X11: i32 = sys::GOOME_X11;
/// Backend: windows.
pub const WIN32: i32 = sys::GOOME_WIN32;
/// Backend: macos.
pub const MACOS: i32 = sys::GOOME_MACOS;
/// Backend: scripted test one, no screen.
pub const TEST: i32 = sys::GOOME_TEST;
/// Backend: offscreen, no window.
pub const OFFSCREEN: i32 = sys::GOOME_OFFSCREEN;
/// Mode: transparent overlay over the desktop.
pub const MODE_OVERLAY: i32 = sys::GOOME_MODE_OVERLAY;
/// Mode: a normal window.
pub const MODE_WINDOW: i32 = sys::GOOME_MODE_WINDOW;
/// Mode: offscreen surface you read the frames from.
pub const MODE_OFFSCREEN: i32 = sys::GOOME_MODE_OFFSCREEN;
/// Window flag: can be resized.
pub const WINDOW_RESIZABLE: i32 = sys::GOOME_WINDOW_RESIZABLE;
/// Window flag: no border.
pub const WINDOW_BORDERLESS: i32 = sys::GOOME_WINDOW_BORDERLESS;
/// Window flag: transparent background.
pub const WINDOW_TRANSPARENT: i32 = sys::GOOME_WINDOW_TRANSPARENT;
/// Window flag: stays on top.
pub const WINDOW_ON_TOP: i32 = sys::GOOME_WINDOW_ON_TOP;
/// Window flag: starts hidden.
pub const WINDOW_HIDDEN: i32 = sys::GOOME_WINDOW_HIDDEN;
/// Window flag: opens centered.
pub const WINDOW_CENTER: i32 = sys::GOOME_WINDOW_CENTER;
/// Window flag: no title bar.
pub const WINDOW_NO_TITLEBAR: i32 = sys::GOOME_WINDOW_NO_TITLEBAR;
/// Window flag: no console window (windows).
pub const WINDOW_NO_CONSOLE: i32 = sys::GOOME_WINDOW_NO_CONSOLE;
/// Error code: it worked.
pub const OK: i32 = sys::GOOME_OK;
/// Error code: not supported here.
pub const ENOTSUP: i32 = sys::GOOME_ENOTSUP;
/// Error code: the os said no.
pub const EDENIED: i32 = sys::GOOME_EDENIED;
/// Error code: no display to talk to.
pub const ENODISPLAY: i32 = sys::GOOME_ENODISPLAY;
/// Error code: the thing is gone.
pub const EGONE: i32 = sys::GOOME_EGONE;
/// Error code: try again.
pub const EAGAIN: i32 = sys::GOOME_EAGAIN;
/// Error code: out of memory.
pub const ENOMEM: i32 = sys::GOOME_ENOMEM;
/// Error code: bad argument.
pub const EARG: i32 = sys::GOOME_EARG;
/// Capability: can grab the screen.
pub const CAP_CAPTURE: i32 = sys::GOOME_CAP_CAPTURE;
/// Capability: can read the pointer anywhere.
pub const CAP_GLOBAL_POINTER: i32 = sys::GOOME_CAP_GLOBAL_POINTER;
/// Capability: global hotkeys.
pub const CAP_HOTKEYS: i32 = sys::GOOME_CAP_HOTKEYS;
/// Capability: can hide our overlay from grabs.
pub const CAP_HIDE_SELF: i32 = sys::GOOME_CAP_HIDE_SELF;
/// Capability: can capture the keyboard.
pub const CAP_KEYBOARD: i32 = sys::GOOME_CAP_KEYBOARD;
/// Capability: can control the window.
pub const CAP_WINDOW: i32 = sys::GOOME_CAP_WINDOW;
/// Capability: clipboard works.
pub const CAP_CLIPBOARD: i32 = sys::GOOME_CAP_CLIPBOARD;
/// Capability: can set the cursor shape.
pub const CAP_CURSOR: i32 = sys::GOOME_CAP_CURSOR;
/// Capability: can grab focus and pointer.
pub const CAP_GRAB: i32 = sys::GOOME_CAP_GRAB;
/// `Ui::wants` bit: the ui wants the mouse.
pub const WANT_MOUSE: i32 = sys::GOOME_WANT_MOUSE;
/// `Ui::wants` bit: the ui wants the keyboard.
pub const WANT_KEYBOARD: i32 = sys::GOOME_WANT_KEYBOARD;
/// Event kind: nothing.
pub const NONE: i32 = sys::GOOME_NONE;
/// Event kind: mouse button pressed.
pub const DOWN: i32 = sys::GOOME_DOWN;
/// Event kind: mouse button released.
pub const UP: i32 = sys::GOOME_UP;
/// Event kind: pointer moved.
pub const MOTION: i32 = sys::GOOME_MOTION;
/// Event kind: scrolled.
pub const SCROLL: i32 = sys::GOOME_SCROLL;
/// Event kind: key pressed.
pub const KEY: i32 = sys::GOOME_KEY;
/// Event kind: a global hotkey fired, `id` is the one you registered.
pub const HOTKEY: i32 = sys::GOOME_HOTKEY;
/// Event kind: a line came in on the control fifo, in `msg`.
pub const CONTROL: i32 = sys::GOOME_CONTROL;
/// Event kind: the monitors changed.
pub const MONITORS: i32 = sys::GOOME_MONITORS;
/// Event kind: goome is shutting down.
pub const QUIT: i32 = sys::GOOME_QUIT;
/// Event kind: window close button, alt-f4 or cmd-w. The app decides what to do.
pub const CLOSE: i32 = sys::GOOME_CLOSE;
/// Event kind: focus changed, `id` is 1 gained and 0 lost.
pub const FOCUS: i32 = sys::GOOME_FOCUS;
/// Event kind: a file was dropped on the window, `msg` is the path and x,y is where.
pub const DROP: i32 = sys::GOOME_DROP;
/// Event kind: an ime, dead key or compose finished some text, in `msg`.
pub const TEXT: i32 = sys::GOOME_TEXT;
/// Event kind: the text being composed changed, `msg` is it and `id` the caret byte offset. Empty = composing ended.
pub const PREEDIT: i32 = sys::GOOME_PREEDIT;
/// Cursor: arrow.
pub const CURSOR_ARROW: i32 = sys::GOOME_CURSOR_ARROW;
/// Cursor: text beam.
pub const CURSOR_IBEAM: i32 = sys::GOOME_CURSOR_IBEAM;
/// Cursor: hand.
pub const CURSOR_HAND: i32 = sys::GOOME_CURSOR_HAND;
/// Cursor: move.
pub const CURSOR_MOVE: i32 = sys::GOOME_CURSOR_MOVE;
/// Cursor: resize left and right.
pub const CURSOR_RESIZE_EW: i32 = sys::GOOME_CURSOR_RESIZE_EW;
/// Cursor: resize up and down.
pub const CURSOR_RESIZE_NS: i32 = sys::GOOME_CURSOR_RESIZE_NS;
/// Cursor: resize diagonally, top left to bottom right.
pub const CURSOR_RESIZE_NWSE: i32 = sys::GOOME_CURSOR_RESIZE_NWSE;
/// Cursor: resize diagonally, top right to bottom left.
pub const CURSOR_RESIZE_NESW: i32 = sys::GOOME_CURSOR_RESIZE_NESW;
/// Cursor: busy.
pub const CURSOR_WAIT: i32 = sys::GOOME_CURSOR_WAIT;
/// Cursor: not allowed.
pub const CURSOR_NO: i32 = sys::GOOME_CURSOR_NO;
/// Cursor: hidden.
pub const CURSOR_HIDDEN: i32 = sys::GOOME_CURSOR_HIDDEN;
/// Window edge for `window_drag`: left.
pub const EDGE_LEFT: i32 = sys::GOOME_EDGE_LEFT;
/// Window edge for `window_drag`: right.
pub const EDGE_RIGHT: i32 = sys::GOOME_EDGE_RIGHT;
/// Window edge for `window_drag`: top.
pub const EDGE_TOP: i32 = sys::GOOME_EDGE_TOP;
/// Window edge for `window_drag`: bottom.
pub const EDGE_BOTTOM: i32 = sys::GOOME_EDGE_BOTTOM;
/// Window state: normal.
pub const STATE_NORMAL: i32 = sys::GOOME_STATE_NORMAL;
/// Window state: minimized.
pub const STATE_MINIMIZED: i32 = sys::GOOME_STATE_MINIMIZED;
/// Window state: maximized.
pub const STATE_MAXIMIZED: i32 = sys::GOOME_STATE_MAXIMIZED;
/// Window state: fullscreen.
pub const STATE_FULLSCREEN: i32 = sys::GOOME_STATE_FULLSCREEN;
/// Key: backspace.
pub const K_BACKSPACE: i32 = sys::GOOME_K_BACKSPACE;
/// Key: tab.
pub const K_TAB: i32 = sys::GOOME_K_TAB;
/// Key: enter.
pub const K_ENTER: i32 = sys::GOOME_K_ENTER;
/// Key: escape.
pub const K_ESC: i32 = sys::GOOME_K_ESC;
/// Key: space.
pub const K_SPACE: i32 = sys::GOOME_K_SPACE;
/// Key: delete.
pub const K_DELETE: i32 = sys::GOOME_K_DELETE;
/// Key: F1, F2 to F12 follow it.
pub const K_F1: i32 = sys::GOOME_K_F1;
/// Key: F12.
pub const K_F12: i32 = sys::GOOME_K_F12;
/// Key: left arrow.
pub const K_LEFT: i32 = sys::GOOME_K_LEFT;
/// Key: right arrow.
pub const K_RIGHT: i32 = sys::GOOME_K_RIGHT;
/// Key: up arrow.
pub const K_UP: i32 = sys::GOOME_K_UP;
/// Key: down arrow.
pub const K_DOWN: i32 = sys::GOOME_K_DOWN;
/// Key: home.
pub const K_HOME: i32 = sys::GOOME_K_HOME;
/// Key: end.
pub const K_END: i32 = sys::GOOME_K_END;
/// Key: page up.
pub const K_PGUP: i32 = sys::GOOME_K_PGUP;
/// Key: page down.
pub const K_PGDN: i32 = sys::GOOME_K_PGDN;
/// Key: insert.
pub const K_INSERT: i32 = sys::GOOME_K_INSERT;
/// Modifier: shift.
pub const SHIFT: i32 = sys::GOOME_SHIFT;
/// Modifier: control.
pub const CTRL: i32 = sys::GOOME_CTRL;
/// Modifier: alt.
pub const ALT: i32 = sys::GOOME_ALT;
/// Modifier: super, the windows or cmd key.
pub const SUPER: i32 = sys::GOOME_SUPER;
/// `frame_copy` format: BGRA bytes, premultiplied.
pub const FMT_BGRA_PREMUL: i32 = sys::GOOME_FMT_BGRA_PREMUL;
/// `frame_copy` format: RGBA bytes, premultiplied.
pub const FMT_RGBA_PREMUL: i32 = sys::GOOME_FMT_RGBA_PREMUL;
/// `frame_copy` format: BGRA bytes, not premultiplied.
pub const FMT_BGRA: i32 = sys::GOOME_FMT_BGRA;
/// `frame_copy` format: RGBA bytes, not premultiplied.
pub const FMT_RGBA: i32 = sys::GOOME_FMT_RGBA;
/// Icon: arrow pointing down.
pub const ICON_ARROW_DOWN: i32 = sys::GOOME_ICON_ARROW_DOWN;
/// Icon: arrow pointing right.
pub const ICON_ARROW_RIGHT: i32 = sys::GOOME_ICON_ARROW_RIGHT;
/// Icon: arrow pointing up.
pub const ICON_ARROW_UP: i32 = sys::GOOME_ICON_ARROW_UP;
/// Icon: arrow pointing left.
pub const ICON_ARROW_LEFT: i32 = sys::GOOME_ICON_ARROW_LEFT;
/// Icon: checkmark.
pub const ICON_CHECK: i32 = sys::GOOME_ICON_CHECK;
/// Icon: dot in a radio button.
pub const ICON_RADIO_DOT: i32 = sys::GOOME_ICON_RADIO_DOT;
/// Icon: close button.
pub const ICON_CLOSE: i32 = sys::GOOME_ICON_CLOSE;
/// Icon: minimize button.
pub const ICON_MINIMIZE: i32 = sys::GOOME_ICON_MINIMIZE;
/// Icon: maximize button.
pub const ICON_MAXIMIZE: i32 = sys::GOOME_ICON_MAXIMIZE;
/// Icon: restore button.
pub const ICON_RESTORE: i32 = sys::GOOME_ICON_RESTORE;
/// Icon: resize grip.
pub const ICON_RESIZE: i32 = sys::GOOME_ICON_RESIZE;
/// Icon: sort ascending arrow.
pub const ICON_SORT_UP: i32 = sys::GOOME_ICON_SORT_UP;
/// Icon: sort descending arrow.
pub const ICON_SORT_DOWN: i32 = sys::GOOME_ICON_SORT_DOWN;
/// Icon: plus.
pub const ICON_PLUS: i32 = sys::GOOME_ICON_PLUS;
/// Icon: minus.
pub const ICON_MINUS: i32 = sys::GOOME_ICON_MINUS;
/// Icon: slider knob.
pub const ICON_SLIDER_KNOB: i32 = sys::GOOME_ICON_SLIDER_KNOB;
/// Icon: toggle knob.
pub const ICON_TOGGLE_KNOB: i32 = sys::GOOME_ICON_TOGGLE_KNOB;
/// Icon: collapse arrow.
pub const ICON_COLLAPSE: i32 = sys::GOOME_ICON_COLLAPSE;
/// Icon: expand arrow.
pub const ICON_EXPAND: i32 = sys::GOOME_ICON_EXPAND;
/// Icon: drag handle.
pub const ICON_DRAG_HANDLE: i32 = sys::GOOME_ICON_DRAG_HANDLE;
/// Icon: toast.
pub const ICON_TOAST: i32 = sys::GOOME_ICON_TOAST;
/// Icon: color picker.
pub const ICON_PICKER: i32 = sys::GOOME_ICON_PICKER;
/// How many icons there are.
pub const ICON_COUNT: i32 = sys::GOOME_ICON_COUNT;
/// Window flag: can be dragged.
pub const WIN_MOVE: i32 = sys::GOOME_WIN_MOVE;
/// Window flag: can be resized.
pub const WIN_RESIZE: i32 = sys::GOOME_WIN_RESIZE;
/// Window flag: can be collapsed.
pub const WIN_COLLAPSE: i32 = sys::GOOME_WIN_COLLAPSE;
/// Window flag: no title bar.
pub const WIN_NO_TITLE: i32 = sys::GOOME_WIN_NO_TITLE;
/// Window flag: has a close button.
pub const WIN_CLOSE: i32 = sys::GOOME_WIN_CLOSE;
/// Input flag: shows the mask char instead of the text.
pub const INPUT_PASSWORD: i32 = sys::GOOME_INPUT_PASSWORD;
/// Input flag: can be selected but not edited.
pub const INPUT_READONLY: i32 = sys::GOOME_INPUT_READONLY;
/// Input flag: only takes numbers.
pub const INPUT_NUMBER: i32 = sys::GOOME_INPUT_NUMBER;
/// Input flag: dont select everything when it gets focus.
pub const INPUT_NO_SELECT_ON_FOCUS: i32 = sys::GOOME_INPUT_NO_SELECT_ON_FOCUS;
/// Input flag: returns when enter is pressed.
pub const INPUT_ENTER_RETURNS: i32 = sys::GOOME_INPUT_ENTER_RETURNS;
/// App flag: draw a title bar.
pub const APP_TITLEBAR: i32 = sys::GOOME_APP_TITLEBAR;
/// App flag: has a menu bar.
pub const APP_MENUBAR: i32 = sys::GOOME_APP_MENUBAR;
/// App flag: has a status bar.
pub const APP_STATUSBAR: i32 = sys::GOOME_APP_STATUSBAR;
/// App flag: no padding around the content.
pub const APP_NO_PAD: i32 = sys::GOOME_APP_NO_PAD;
/// Text edit flag: can be selected but not edited.
pub const EDIT_READONLY: i32 = sys::GOOME_EDIT_READONLY;
/// Text edit flag: shows line numbers.
pub const EDIT_LINE_NUMBERS: i32 = sys::GOOME_EDIT_LINE_NUMBERS;
/// Text edit flag: wraps long lines.
pub const EDIT_WRAP: i32 = sys::GOOME_EDIT_WRAP;
/// Text edit flag: mono font.
pub const EDIT_MONO: i32 = sys::GOOME_EDIT_MONO;
/// Table flag: columns can be clicked to sort.
pub const TABLE_SORTABLE: i32 = sys::GOOME_TABLE_SORTABLE;
/// Table flag: columns can be resized.
pub const TABLE_RESIZABLE: i32 = sys::GOOME_TABLE_RESIZABLE;
/// Table flag: alternate row colors.
pub const TABLE_STRIPES: i32 = sys::GOOME_TABLE_STRIPES;
/// Table flag: borders between cells.
pub const TABLE_BORDERS: i32 = sys::GOOME_TABLE_BORDERS;
/// Style slot for `push_color` and `push_num`: window background color.
pub const S_BG: i32 = sys::GOOME_S_BG;
/// Style slot for `push_color` and `push_num`: panel color.
pub const S_PANEL: i32 = sys::GOOME_S_PANEL;
/// Style slot for `push_color` and `push_num`: border color.
pub const S_BORDER: i32 = sys::GOOME_S_BORDER;
/// Style slot for `push_color` and `push_num`: text color.
pub const S_TEXT: i32 = sys::GOOME_S_TEXT;
/// Style slot for `push_color` and `push_num`: dimmed text color.
pub const S_TEXT_DIM: i32 = sys::GOOME_S_TEXT_DIM;
/// Style slot for `push_color` and `push_num`: accent color.
pub const S_ACCENT: i32 = sys::GOOME_S_ACCENT;
/// Style slot for `push_color` and `push_num`: text on the accent.
pub const S_ACCENT_TEXT: i32 = sys::GOOME_S_ACCENT_TEXT;
/// Style slot for `push_color` and `push_num`: hover color.
pub const S_HOVER: i32 = sys::GOOME_S_HOVER;
/// Style slot for `push_color` and `push_num`: pressed color.
pub const S_ACTIVE: i32 = sys::GOOME_S_ACTIVE;
/// Style slot for `push_color` and `push_num`: input background.
pub const S_INPUT_BG: i32 = sys::GOOME_S_INPUT_BG;
/// Style slot for `push_color` and `push_num`: shadow color.
pub const S_SHADOW: i32 = sys::GOOME_S_SHADOW;
/// Style slot for `push_color` and `push_num`: corner radius.
pub const S_RADIUS: i32 = sys::GOOME_S_RADIUS;
/// Style slot for `push_color` and `push_num`: padding.
pub const S_PAD: i32 = sys::GOOME_S_PAD;
/// Style slot for `push_color` and `push_num`: gap between widgets.
pub const S_GAP: i32 = sys::GOOME_S_GAP;
/// Style slot for `push_color` and `push_num`: border width.
pub const S_BORDER_WIDTH: i32 = sys::GOOME_S_BORDER_WIDTH;
/// Style slot for `push_color` and `push_num`: text size.
pub const S_FONT_SIZE: i32 = sys::GOOME_S_FONT_SIZE;
/// Style slot for `push_color` and `push_num`: shadow blur.
pub const S_SHADOW_BLUR: i32 = sys::GOOME_S_SHADOW_BLUR;
/// Style slot for `push_color` and `push_num`: animation time in ms.
pub const S_ANIM_MS: i32 = sys::GOOME_S_ANIM_MS;
/// Style slot for `push_color` and `push_num`: focus ring color.
pub const S_FOCUS: i32 = sys::GOOME_S_FOCUS;
/// Style slot for `push_color` and `push_num`: selection color.
pub const S_SELECT: i32 = sys::GOOME_S_SELECT;
/// Style slot for `push_color` and `push_num`: caret color.
pub const S_CARET: i32 = sys::GOOME_S_CARET;
/// Style slot for `push_color` and `push_num`: link color.
pub const S_LINK: i32 = sys::GOOME_S_LINK;
/// Style slot for `push_color` and `push_num`: badge color.
pub const S_BADGE: i32 = sys::GOOME_S_BADGE;
/// Style slot for `push_color` and `push_num`: toast background.
pub const S_TOAST: i32 = sys::GOOME_S_TOAST;
/// Style slot for `push_color` and `push_num`: tooltip background.
pub const S_TOOLTIP: i32 = sys::GOOME_S_TOOLTIP;
/// Style slot for `push_color` and `push_num`: danger color.
pub const S_DANGER: i32 = sys::GOOME_S_DANGER;
/// Style slot for `push_color` and `push_num`: knob color.
pub const S_KNOB: i32 = sys::GOOME_S_KNOB;
/// Style slot for `push_color` and `push_num`: modal dim color.
pub const S_DIM: i32 = sys::GOOME_S_DIM;
/// Style slot for `push_color` and `push_num`: checkbox size.
pub const S_CHECK_SIZE: i32 = sys::GOOME_S_CHECK_SIZE;
/// Style slot for `push_color` and `push_num`: checkbox corner radius.
pub const S_CHECK_RADIUS: i32 = sys::GOOME_S_CHECK_RADIUS;
/// Style slot for `push_color` and `push_num`: toggle width.
pub const S_TOGGLE_W: i32 = sys::GOOME_S_TOGGLE_W;
/// Style slot for `push_color` and `push_num`: toggle height.
pub const S_TOGGLE_H: i32 = sys::GOOME_S_TOGGLE_H;
/// Style slot for `push_color` and `push_num`: slider height.
pub const S_SLIDER_H: i32 = sys::GOOME_S_SLIDER_H;
/// Style slot for `push_color` and `push_num`: slider track thickness.
pub const S_SLIDER_TRACK: i32 = sys::GOOME_S_SLIDER_TRACK;
/// Style slot for `push_color` and `push_num`: knob size.
pub const S_KNOB_SIZE: i32 = sys::GOOME_S_KNOB_SIZE;
/// Style slot for `push_color` and `push_num`: scrollbar width.
pub const S_SCROLL_W: i32 = sys::GOOME_S_SCROLL_W;
/// Style slot for `push_color` and `push_num`: smallest scrollbar thumb.
pub const S_SCROLL_MIN: i32 = sys::GOOME_S_SCROLL_MIN;
/// Style slot for `push_color` and `push_num`: list item height.
pub const S_ITEM_H: i32 = sys::GOOME_S_ITEM_H;
/// Style slot for `push_color` and `push_num`: menu item height.
pub const S_MENU_ITEM_H: i32 = sys::GOOME_S_MENU_ITEM_H;
/// Style slot for `push_color` and `push_num`: tab height.
pub const S_TAB_H: i32 = sys::GOOME_S_TAB_H;
/// Style slot for `push_color` and `push_num`: window title bar height.
pub const S_WIN_TITLE_H: i32 = sys::GOOME_S_WIN_TITLE_H;
/// Style slot for `push_color` and `push_num`: app title bar height.
pub const S_APP_TITLE_H: i32 = sys::GOOME_S_APP_TITLE_H;
/// Style slot for `push_color` and `push_num`: menu bar height.
pub const S_MENUBAR_H: i32 = sys::GOOME_S_MENUBAR_H;
/// Style slot for `push_color` and `push_num`: status bar height.
pub const S_STATUSBAR_H: i32 = sys::GOOME_S_STATUSBAR_H;
/// Style slot for `push_color` and `push_num`: focus ring width.
pub const S_FOCUS_W: i32 = sys::GOOME_S_FOCUS_W;
/// Style slot for `push_color` and `push_num`: focus ring gap.
pub const S_FOCUS_GAP: i32 = sys::GOOME_S_FOCUS_GAP;
/// Style slot for `push_color` and `push_num`: separator thickness.
pub const S_SEP_W: i32 = sys::GOOME_S_SEP_W;
/// Style slot for `push_color` and `push_num`: resize grip size.
pub const S_GRIP: i32 = sys::GOOME_S_GRIP;
/// Style slot for `push_color` and `push_num`: tree indent.
pub const S_INDENT: i32 = sys::GOOME_S_INDENT;
/// Style slot for `push_color` and `push_num`: plot line width.
pub const S_PLOT_W: i32 = sys::GOOME_S_PLOT_W;
/// Style slot for `push_color` and `push_num`: spinner thickness.
pub const S_SPINNER: i32 = sys::GOOME_S_SPINNER;
/// Style slot for `push_color` and `push_num`: caret width.
pub const S_CARET_W: i32 = sys::GOOME_S_CARET_W;
/// Style slot for `push_color` and `push_num`: smallest window width.
pub const S_WIN_MIN_W: i32 = sys::GOOME_S_WIN_MIN_W;
/// Style slot for `push_color` and `push_num`: smallest window height.
pub const S_WIN_MIN_H: i32 = sys::GOOME_S_WIN_MIN_H;
/// Style slot for `push_color` and `push_num`: popup gap.
pub const S_POP_GAP: i32 = sys::GOOME_S_POP_GAP;
/// Style slot for `push_color` and `push_num`: menu gap.
pub const S_MENU_GAP: i32 = sys::GOOME_S_MENU_GAP;
/// Style slot for `push_color` and `push_num`: tooltip horizontal padding.
pub const S_TIP_PAD_X: i32 = sys::GOOME_S_TIP_PAD_X;
/// Style slot for `push_color` and `push_num`: tooltip vertical padding.
pub const S_TIP_PAD_Y: i32 = sys::GOOME_S_TIP_PAD_Y;
/// Style slot for `push_color` and `push_num`: tooltip x offset.
pub const S_TIP_X: i32 = sys::GOOME_S_TIP_X;
/// Style slot for `push_color` and `push_num`: tooltip y offset.
pub const S_TIP_Y: i32 = sys::GOOME_S_TIP_Y;
/// Style slot for `push_color` and `push_num`: tooltip corner radius.
pub const S_TIP_RADIUS: i32 = sys::GOOME_S_TIP_RADIUS;
/// Style slot for `push_color` and `push_num`: shadow x offset.
pub const S_SHADOW_X: i32 = sys::GOOME_S_SHADOW_X;
/// Style slot for `push_color` and `push_num`: shadow y offset.
pub const S_SHADOW_Y: i32 = sys::GOOME_S_SHADOW_Y;
/// Style slot for `push_color` and `push_num`: text alignment.
pub const S_ALIGN: i32 = sys::GOOME_S_ALIGN;
/// Style slot for `push_color` and `push_num`: heading text size.
pub const S_HEADING_SIZE: i32 = sys::GOOME_S_HEADING_SIZE;
/// Style slot for `push_color` and `push_num`: small text size.
pub const S_SMALL_SIZE: i32 = sys::GOOME_S_SMALL_SIZE;
/// Style slot for `push_color` and `push_num`: easing.
pub const S_EASE: i32 = sys::GOOME_S_EASE;
/// Style slot for `push_color` and `push_num`: hover fade in ms.
pub const S_HOVER_MS: i32 = sys::GOOME_S_HOVER_MS;
/// Style slot for `push_color` and `push_num`: press fade in ms.
pub const S_PRESS_MS: i32 = sys::GOOME_S_PRESS_MS;
/// Style slot for `push_color` and `push_num`: popup open time in ms.
pub const S_POPUP_MS: i32 = sys::GOOME_S_POPUP_MS;
/// Style slot for `push_color` and `push_num`: collapse time in ms.
pub const S_COLLAPSE_MS: i32 = sys::GOOME_S_COLLAPSE_MS;
/// Style slot for `push_color` and `push_num`: toast fade in ms.
pub const S_TOAST_FADE_MS: i32 = sys::GOOME_S_TOAST_FADE_MS;
/// How many style slots there are.
pub const S_COUNT: i32 = sys::GOOME_S_COUNT;
/// Text alignment: whatever fits the widget.
pub const ALIGN_AUTO: i32 = sys::GOOME_ALIGN_AUTO;
/// Text alignment: left.
pub const ALIGN_LEFT: i32 = sys::GOOME_ALIGN_LEFT;
/// Text alignment: center.
pub const ALIGN_CENTER: i32 = sys::GOOME_ALIGN_CENTER;
/// Text alignment: right.
pub const ALIGN_RIGHT: i32 = sys::GOOME_ALIGN_RIGHT;
/// Easing: smooth, the default.
pub const EASE_SMOOTH: i32 = sys::GOOME_EASE_SMOOTH;
/// Easing: linear.
pub const EASE_LINEAR: i32 = sys::GOOME_EASE_LINEAR;
/// Easing: fast start, slow end.
pub const EASE_OUT: i32 = sys::GOOME_EASE_OUT;
/// Easing: slow at both ends.
pub const EASE_IN_OUT: i32 = sys::GOOME_EASE_IN_OUT;
/// Easing: overshoots a bit then settles.
pub const EASE_BACK: i32 = sys::GOOME_EASE_BACK;
/// Easing: your own function from `Config::set_ease`.
pub const EASE_CUSTOM: i32 = sys::GOOME_EASE_CUSTOM;
/// Easing: slow start, fast end.
pub const EASE_IN: i32 = sys::GOOME_EASE_IN;
/// Easing: springy.
pub const EASE_SPRING: i32 = sys::GOOME_EASE_SPRING;
/// Easing: bounces at the end.
pub const EASE_BOUNCE: i32 = sys::GOOME_EASE_BOUNCE;
/// Skin kind: button.
pub const SKIN_BUTTON: i32 = sys::GOOME_SKIN_BUTTON;
/// Skin kind: panel.
pub const SKIN_PANEL: i32 = sys::GOOME_SKIN_PANEL;
/// Skin kind: input.
pub const SKIN_INPUT: i32 = sys::GOOME_SKIN_INPUT;
/// Skin kind: checkbox.
pub const SKIN_CHECKBOX: i32 = sys::GOOME_SKIN_CHECKBOX;
/// Skin kind: radio.
pub const SKIN_RADIO: i32 = sys::GOOME_SKIN_RADIO;
/// Skin kind: toggle.
pub const SKIN_TOGGLE: i32 = sys::GOOME_SKIN_TOGGLE;
/// Skin kind: slider track.
pub const SKIN_SLIDER_TRACK: i32 = sys::GOOME_SKIN_SLIDER_TRACK;
/// Skin kind: slider fill.
pub const SKIN_SLIDER_FILL: i32 = sys::GOOME_SKIN_SLIDER_FILL;
/// Skin kind: scrollbar.
pub const SKIN_SCROLLBAR: i32 = sys::GOOME_SKIN_SCROLLBAR;
/// Skin kind: tab.
pub const SKIN_TAB: i32 = sys::GOOME_SKIN_TAB;
/// Skin kind: titlebar.
pub const SKIN_TITLEBAR: i32 = sys::GOOME_SKIN_TITLEBAR;
/// Skin kind: tooltip.
pub const SKIN_TOOLTIP: i32 = sys::GOOME_SKIN_TOOLTIP;
/// Skin kind: menu.
pub const SKIN_MENU: i32 = sys::GOOME_SKIN_MENU;
/// Skin kind: popup.
pub const SKIN_POPUP: i32 = sys::GOOME_SKIN_POPUP;
/// How many skin kinds there are.
pub const SKIN_COUNT: i32 = sys::GOOME_SKIN_COUNT;
/// `custom` flag: tracks dragging, alone it locks to the x axis.
pub const ITEM_DRAG_X: i32 = sys::GOOME_ITEM_DRAG_X;
/// `custom` flag: tracks dragging, alone it locks to the y axis.
pub const ITEM_DRAG_Y: i32 = sys::GOOME_ITEM_DRAG_Y;
/// `custom` flag: can take keyboard focus.
pub const ITEM_FOCUSABLE: i32 = sys::GOOME_ITEM_FOCUSABLE;
/// `custom` flag: doesnt add its own hit area.
pub const ITEM_NO_HIT: i32 = sys::GOOME_ITEM_NO_HIT;
/// `custom` flag: keeps reporting while held.
pub const ITEM_REPEAT: i32 = sys::GOOME_ITEM_REPEAT;
/// Theme preset: dark.
pub const PRESET_DARK: i32 = sys::GOOME_PRESET_DARK;
/// Theme preset: light.
pub const PRESET_LIGHT: i32 = sys::GOOME_PRESET_LIGHT;
/// Theme preset: midnight.
pub const PRESET_MIDNIGHT: i32 = sys::GOOME_PRESET_MIDNIGHT;
/// Theme preset: paper.
pub const PRESET_PAPER: i32 = sys::GOOME_PRESET_PAPER;
/// Theme preset: mint.
pub const PRESET_MINT: i32 = sys::GOOME_PRESET_MINT;
/// Theme preset: high contrast.
pub const PRESET_HIGH_CONTRAST: i32 = sys::GOOME_PRESET_HIGH_CONTRAST;
/// Theme preset: rose.
pub const PRESET_ROSE: i32 = sys::GOOME_PRESET_ROSE;
/// How many presets there are.
pub const PRESET_COUNT: i32 = sys::GOOME_PRESET_COUNT;
/// Key action: copy.
pub const ACT_COPY: i32 = sys::GOOME_ACT_COPY;
/// Key action: cut.
pub const ACT_CUT: i32 = sys::GOOME_ACT_CUT;
/// Key action: paste.
pub const ACT_PASTE: i32 = sys::GOOME_ACT_PASTE;
/// Key action: undo.
pub const ACT_UNDO: i32 = sys::GOOME_ACT_UNDO;
/// Key action: redo.
pub const ACT_REDO: i32 = sys::GOOME_ACT_REDO;
/// Key action: select all.
pub const ACT_SELECT_ALL: i32 = sys::GOOME_ACT_SELECT_ALL;
/// Key action: jump a word left.
pub const ACT_WORD_LEFT: i32 = sys::GOOME_ACT_WORD_LEFT;
/// Key action: jump a word right.
pub const ACT_WORD_RIGHT: i32 = sys::GOOME_ACT_WORD_RIGHT;
/// Key action: delete the word to the left.
pub const ACT_DEL_WORD_LEFT: i32 = sys::GOOME_ACT_DEL_WORD_LEFT;
/// Key action: delete the word to the right.
pub const ACT_DEL_WORD_RIGHT: i32 = sys::GOOME_ACT_DEL_WORD_RIGHT;
/// Key action: jump to the start.
pub const ACT_DOC_START: i32 = sys::GOOME_ACT_DOC_START;
/// Key action: jump to the end.
pub const ACT_DOC_END: i32 = sys::GOOME_ACT_DOC_END;
/// How many key actions there are.
pub const ACT_COUNT: i32 = sys::GOOME_ACT_COUNT;
/// Cursor spot: over a button.
pub const CUR_BUTTON: i32 = sys::GOOME_CUR_BUTTON;
/// Cursor spot: over an input.
pub const CUR_INPUT: i32 = sys::GOOME_CUR_INPUT;
/// Cursor spot: over a link.
pub const CUR_LINK: i32 = sys::GOOME_CUR_LINK;
/// Cursor spot: over a draggable number.
pub const CUR_DRAG_VALUE: i32 = sys::GOOME_CUR_DRAG_VALUE;
/// Cursor spot: over a side by side split handle.
pub const CUR_SPLIT_X: i32 = sys::GOOME_CUR_SPLIT_X;
/// Cursor spot: over a stacked split handle.
pub const CUR_SPLIT_Y: i32 = sys::GOOME_CUR_SPLIT_Y;
/// Cursor spot: over a window resize grip.
pub const CUR_GRIP: i32 = sys::GOOME_CUR_GRIP;
/// Cursor spot: over a table column edge.
pub const CUR_COLUMN: i32 = sys::GOOME_CUR_COLUMN;
/// Cursor spot: while dragging something.
pub const CUR_DRAG_DROP: i32 = sys::GOOME_CUR_DRAG_DROP;
/// Cursor spot: over a custom item that drags on x.
pub const CUR_CUSTOM_X: i32 = sys::GOOME_CUR_CUSTOM_X;
/// Cursor spot: over a custom item that drags on y.
pub const CUR_CUSTOM_Y: i32 = sys::GOOME_CUR_CUSTOM_Y;
/// Cursor spot: over a custom item that drags both ways.
pub const CUR_CUSTOM_XY: i32 = sys::GOOME_CUR_CUSTOM_XY;
/// How many cursor spots there are.
pub const CUR_COUNT: i32 = sys::GOOME_CUR_COUNT;
/// Accessibility role: none.
pub const ROLE_NONE: i32 = sys::GOOME_ROLE_NONE;
/// Accessibility role: window.
pub const ROLE_WINDOW: i32 = sys::GOOME_ROLE_WINDOW;
/// Accessibility role: button.
pub const ROLE_BUTTON: i32 = sys::GOOME_ROLE_BUTTON;
/// Accessibility role: checkbox.
pub const ROLE_CHECKBOX: i32 = sys::GOOME_ROLE_CHECKBOX;
/// Accessibility role: radio button.
pub const ROLE_RADIO: i32 = sys::GOOME_ROLE_RADIO;
/// Accessibility role: slider.
pub const ROLE_SLIDER: i32 = sys::GOOME_ROLE_SLIDER;
/// Accessibility role: text.
pub const ROLE_TEXT: i32 = sys::GOOME_ROLE_TEXT;
/// Accessibility role: text input.
pub const ROLE_INPUT: i32 = sys::GOOME_ROLE_INPUT;
/// Accessibility role: dropdown.
pub const ROLE_COMBO: i32 = sys::GOOME_ROLE_COMBO;
/// Accessibility role: list.
pub const ROLE_LIST: i32 = sys::GOOME_ROLE_LIST;
/// Accessibility role: list item.
pub const ROLE_ITEM: i32 = sys::GOOME_ROLE_ITEM;
/// Accessibility role: tab.
pub const ROLE_TAB: i32 = sys::GOOME_ROLE_TAB;
/// Accessibility role: menu.
pub const ROLE_MENU: i32 = sys::GOOME_ROLE_MENU;
/// Accessibility role: menu item.
pub const ROLE_MENUITEM: i32 = sys::GOOME_ROLE_MENUITEM;
/// Accessibility role: table.
pub const ROLE_TABLE: i32 = sys::GOOME_ROLE_TABLE;
/// Accessibility role: tree.
pub const ROLE_TREE: i32 = sys::GOOME_ROLE_TREE;
/// Accessibility role: progress bar.
pub const ROLE_PROGRESS: i32 = sys::GOOME_ROLE_PROGRESS;
/// Accessibility role: image.
pub const ROLE_IMAGE: i32 = sys::GOOME_ROLE_IMAGE;
/// Accessibility role: group.
pub const ROLE_GROUP: i32 = sys::GOOME_ROLE_GROUP;
/// Accessibility role: dialog.
pub const ROLE_DIALOG: i32 = sys::GOOME_ROLE_DIALOG;
/// Accessibility state: has keyboard focus.
pub const A11Y_FOCUSED: i32 = sys::GOOME_A11Y_FOCUSED;
/// Accessibility state: checked.
pub const A11Y_CHECKED: i32 = sys::GOOME_A11Y_CHECKED;
/// Accessibility state: disabled.
pub const A11Y_DISABLED: i32 = sys::GOOME_A11Y_DISABLED;
/// Accessibility state: expanded.
pub const A11Y_EXPANDED: i32 = sys::GOOME_A11Y_EXPANDED;
/// Accessibility state: selected.
pub const A11Y_SELECTED: i32 = sys::GOOME_A11Y_SELECTED;
/// Text direction: from the first strong letter of each line.
pub const DIR_AUTO: i32 = sys::GOOME_DIR_AUTO;
/// Text direction: left to right.
pub const DIR_LTR: i32 = sys::GOOME_DIR_LTR;
/// Text direction: right to left.
pub const DIR_RTL: i32 = sys::GOOME_DIR_RTL;
