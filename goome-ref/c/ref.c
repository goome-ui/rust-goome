#include <stdio.h>
#include <stddef.h>
#include <string.h>

#include "goome.h"

static char *rf_out;
static int rf_cap, rf_len;

static void rf_put(const char *name, int a, int b)
{
    if (rf_len < rf_cap) rf_len += snprintf(rf_out + rf_len, (size_t)(rf_cap - rf_len), "%s %d %d\n", name, a, b);
}

#define S(t) rf_put(#t, (int)sizeof(t), 0)
#define F(t, f) rf_put(#t "." #f, (int)offsetof(t, f), (int)sizeof(((t *)0)->f))

int ref_layout(char *out, int cap)
{
    rf_out = out, rf_cap = cap, rf_len = 0;
    out[0] = 0;
    S(goome_opts);
    F(goome_opts, title); F(goome_opts, backend); F(goome_opts, state_dir); F(goome_opts, no_capture);
    F(goome_opts, capture_fps); F(goome_opts, hide_self); F(goome_opts, test_frames); F(goome_opts, test_script);
    F(goome_opts, test_shots); F(goome_opts, mode); F(goome_opts, width); F(goome_opts, height);
    F(goome_opts, min_width); F(goome_opts, min_height); F(goome_opts, window_flags); F(goome_opts, scale);
    S(goome_rect);
    F(goome_rect, x); F(goome_rect, y); F(goome_rect, w); F(goome_rect, h);
    S(goome_monitor);
    F(goome_monitor, id); F(goome_monitor, name); F(goome_monitor, area); F(goome_monitor, scale); F(goome_monitor, pw);
    F(goome_monitor, ph); F(goome_monitor, rotation);
    S(goome_image);
    F(goome_image, px); F(goome_image, w); F(goome_image, h); F(goome_image, stride); F(goome_image, area);
    F(goome_image, mon); F(goome_image, has_self); F(goome_image, has_cursor); F(goome_image, cursor);
    S(goome_canvas);
    F(goome_canvas, px); F(goome_canvas, w); F(goome_canvas, h); F(goome_canvas, stride); F(goome_canvas, ox);
    F(goome_canvas, oy); F(goome_canvas, scale); F(goome_canvas, mon); F(goome_canvas, dx0); F(goome_canvas, dy0);
    F(goome_canvas, dx1); F(goome_canvas, dy1); F(goome_canvas, cx0); F(goome_canvas, cy0); F(goome_canvas, cx1);
    F(goome_canvas, cy1); F(goome_canvas, kx0); F(goome_canvas, ky0); F(goome_canvas, kx1); F(goome_canvas, ky1);
    F(goome_canvas, nclip); F(goome_canvas, clips);
    S(goome_event);
    F(goome_event, type); F(goome_event, x); F(goome_event, y); F(goome_event, mon); F(goome_event, button);
    F(goome_event, dy); F(goome_event, key); F(goome_event, mods); F(goome_event, text); F(goome_event, id);
    F(goome_event, msg);
    S(goome_pacer);
    F(goome_pacer, next); F(goome_pacer, fps);
    S(goome_radii);
    F(goome_radii, tl); F(goome_radii, tr); F(goome_radii, br); F(goome_radii, bl);
    S(goome_bitmap);
    F(goome_bitmap, px); F(goome_bitmap, w); F(goome_bitmap, h); F(goome_bitmap, stride); F(goome_bitmap, owned);
    S(goome_theme);
    F(goome_theme, bg); F(goome_theme, panel); F(goome_theme, border); F(goome_theme, text); F(goome_theme, text_dim);
    F(goome_theme, accent); F(goome_theme, accent_text); F(goome_theme, hover); F(goome_theme, active);
    F(goome_theme, input_bg); F(goome_theme, shadow); F(goome_theme, radius); F(goome_theme, pad); F(goome_theme, gap);
    F(goome_theme, border_width); F(goome_theme, font_size); F(goome_theme, shadow_blur); F(goome_theme, anim_ms);
    F(goome_theme, font); F(goome_theme, skin_button); F(goome_theme, skin_panel); F(goome_theme, skin_input);
    F(goome_theme, icon); F(goome_theme, icon_tint); F(goome_theme, skin_checkbox); F(goome_theme, skin_radio);
    F(goome_theme, skin_toggle); F(goome_theme, skin_slider); F(goome_theme, skin_slider_fill); F(goome_theme, skin_scrollbar);
    F(goome_theme, skin_tab); F(goome_theme, skin_titlebar); F(goome_theme, skin_tooltip); F(goome_theme, skin_menu);
    F(goome_theme, skin_popup);
    F(goome_theme, focus); F(goome_theme, select); F(goome_theme, caret); F(goome_theme, link); F(goome_theme, badge);
    F(goome_theme, toast); F(goome_theme, tooltip); F(goome_theme, danger); F(goome_theme, knob); F(goome_theme, dim);
    F(goome_theme, check_size); F(goome_theme, check_radius); F(goome_theme, toggle_w); F(goome_theme, toggle_h); F(goome_theme, slider_h);
    F(goome_theme, slider_track); F(goome_theme, knob_size); F(goome_theme, scroll_w); F(goome_theme, scroll_min); F(goome_theme, item_h);
    F(goome_theme, menu_item_h); F(goome_theme, tab_h); F(goome_theme, win_title_h); F(goome_theme, app_title_h); F(goome_theme, menubar_h);
    F(goome_theme, statusbar_h); F(goome_theme, focus_w); F(goome_theme, focus_gap); F(goome_theme, sep_w); F(goome_theme, grip);
    F(goome_theme, indent); F(goome_theme, plot_w); F(goome_theme, spinner); F(goome_theme, caret_w); F(goome_theme, win_min_w);
    F(goome_theme, win_min_h); F(goome_theme, pop_gap); F(goome_theme, menu_gap); F(goome_theme, tip_pad_x); F(goome_theme, tip_pad_y);
    F(goome_theme, tip_x); F(goome_theme, tip_y); F(goome_theme, tip_radius); F(goome_theme, shadow_x); F(goome_theme, shadow_y);
    F(goome_theme, align); F(goome_theme, heading_size); F(goome_theme, small_size); F(goome_theme, ease); F(goome_theme, hover_ms);
    F(goome_theme, press_ms); F(goome_theme, popup_ms); F(goome_theme, collapse_ms); F(goome_theme, toast_fade_ms); F(goome_theme, font_heading);
    F(goome_theme, font_mono); F(goome_theme, font_small);
    S(goome_skin);
    F(goome_skin, bm); F(goome_skin, l); F(goome_skin, t); F(goome_skin, r); F(goome_skin, b);
    S(goome_ui_item);
    F(goome_ui_item, x); F(goome_ui_item, y); F(goome_ui_item, w); F(goome_ui_item, h); F(goome_ui_item, hovered);
    F(goome_ui_item, pressed); F(goome_ui_item, held); F(goome_ui_item, released); F(goome_ui_item, clicked);
    F(goome_ui_item, double_clicked); F(goome_ui_item, right_clicked); F(goome_ui_item, focused); F(goome_ui_item, mx);
    F(goome_ui_item, my); F(goome_ui_item, dx); F(goome_ui_item, dy); F(goome_ui_item, drag_x); F(goome_ui_item, drag_y);
    F(goome_ui_item, key);
    S(goome_combo);
    F(goome_combo, key); F(goome_combo, mods);
    S(goome_ui_config);
    F(goome_ui_config, tip_delay_ms); F(goome_ui_config, dbl_click_ms); F(goome_ui_config, dbl_click_px); F(goome_ui_config, drag_px); F(goome_ui_config, repeat_delay_ms);
    F(goome_ui_config, repeat_ms); F(goome_ui_config, blink_ms); F(goome_ui_config, menu_delay_ms); F(goome_ui_config, toast_ms); F(goome_ui_config, scroll_lines);
    F(goome_ui_config, raise_on_click); F(goome_ui_config, esc_closes); F(goome_ui_config, tab_nav); F(goome_ui_config, focus_on_mouse); F(goome_ui_config, cursor);
    F(goome_ui_config, keys); F(goome_ui_config, ease); F(goome_ui_config, ease_user); F(goome_ui_config, mask);
    S(goome_a11y_node);
    F(goome_a11y_node, role); F(goome_a11y_node, parent); F(goome_a11y_node, states); F(goome_a11y_node, x); F(goome_a11y_node, y);
    F(goome_a11y_node, w); F(goome_a11y_node, h); F(goome_a11y_node, label); F(goome_a11y_node, value); F(goome_a11y_node, id);
    F(goome_a11y_node, num); F(goome_a11y_node, min); F(goome_a11y_node, max); F(goome_a11y_node, caret);
    return rf_len;
}

static const struct {
    const char *name;
    int value;
} rf_consts[] = {
    {"GOOME_AUTO", GOOME_AUTO},
    {"GOOME_WAYLAND", GOOME_WAYLAND},
    {"GOOME_GNOME", GOOME_GNOME},
    {"GOOME_PORTAL", GOOME_PORTAL},
    {"GOOME_X11", GOOME_X11},
    {"GOOME_WIN32", GOOME_WIN32},
    {"GOOME_MACOS", GOOME_MACOS},
    {"GOOME_TEST", GOOME_TEST},
    {"GOOME_OFFSCREEN", GOOME_OFFSCREEN},
    {"GOOME_MODE_OVERLAY", GOOME_MODE_OVERLAY},
    {"GOOME_MODE_WINDOW", GOOME_MODE_WINDOW},
    {"GOOME_MODE_OFFSCREEN", GOOME_MODE_OFFSCREEN},
    {"GOOME_WINDOW_RESIZABLE", GOOME_WINDOW_RESIZABLE},
    {"GOOME_WINDOW_BORDERLESS", GOOME_WINDOW_BORDERLESS},
    {"GOOME_WINDOW_TRANSPARENT", GOOME_WINDOW_TRANSPARENT},
    {"GOOME_WINDOW_ON_TOP", GOOME_WINDOW_ON_TOP},
    {"GOOME_WINDOW_HIDDEN", GOOME_WINDOW_HIDDEN},
    {"GOOME_WINDOW_CENTER", GOOME_WINDOW_CENTER},
    {"GOOME_WINDOW_NO_TITLEBAR", GOOME_WINDOW_NO_TITLEBAR},
    {"GOOME_WINDOW_NO_CONSOLE", GOOME_WINDOW_NO_CONSOLE},
    {"GOOME_OK", GOOME_OK},
    {"GOOME_ENOTSUP", GOOME_ENOTSUP},
    {"GOOME_EDENIED", GOOME_EDENIED},
    {"GOOME_ENODISPLAY", GOOME_ENODISPLAY},
    {"GOOME_EGONE", GOOME_EGONE},
    {"GOOME_EAGAIN", GOOME_EAGAIN},
    {"GOOME_ENOMEM", GOOME_ENOMEM},
    {"GOOME_EARG", GOOME_EARG},
    {"GOOME_CAP_CAPTURE", GOOME_CAP_CAPTURE},
    {"GOOME_CAP_GLOBAL_POINTER", GOOME_CAP_GLOBAL_POINTER},
    {"GOOME_CAP_HOTKEYS", GOOME_CAP_HOTKEYS},
    {"GOOME_CAP_HIDE_SELF", GOOME_CAP_HIDE_SELF},
    {"GOOME_CAP_KEYBOARD", GOOME_CAP_KEYBOARD},
    {"GOOME_CAP_WINDOW", GOOME_CAP_WINDOW},
    {"GOOME_CAP_CLIPBOARD", GOOME_CAP_CLIPBOARD},
    {"GOOME_CAP_CURSOR", GOOME_CAP_CURSOR},
    {"GOOME_CAP_GRAB", GOOME_CAP_GRAB},
    {"GOOME_WANT_MOUSE", GOOME_WANT_MOUSE},
    {"GOOME_WANT_KEYBOARD", GOOME_WANT_KEYBOARD},
    {"GOOME_NONE", GOOME_NONE},
    {"GOOME_DOWN", GOOME_DOWN},
    {"GOOME_UP", GOOME_UP},
    {"GOOME_MOTION", GOOME_MOTION},
    {"GOOME_SCROLL", GOOME_SCROLL},
    {"GOOME_KEY", GOOME_KEY},
    {"GOOME_HOTKEY", GOOME_HOTKEY},
    {"GOOME_CONTROL", GOOME_CONTROL},
    {"GOOME_MONITORS", GOOME_MONITORS},
    {"GOOME_QUIT", GOOME_QUIT},
    {"GOOME_CLOSE", GOOME_CLOSE},
    {"GOOME_FOCUS", GOOME_FOCUS},
    {"GOOME_DROP", GOOME_DROP},
    {"GOOME_TEXT", GOOME_TEXT},
    {"GOOME_PREEDIT", GOOME_PREEDIT},
    {"GOOME_CURSOR_ARROW", GOOME_CURSOR_ARROW},
    {"GOOME_CURSOR_IBEAM", GOOME_CURSOR_IBEAM},
    {"GOOME_CURSOR_HAND", GOOME_CURSOR_HAND},
    {"GOOME_CURSOR_MOVE", GOOME_CURSOR_MOVE},
    {"GOOME_CURSOR_RESIZE_EW", GOOME_CURSOR_RESIZE_EW},
    {"GOOME_CURSOR_RESIZE_NS", GOOME_CURSOR_RESIZE_NS},
    {"GOOME_CURSOR_RESIZE_NWSE", GOOME_CURSOR_RESIZE_NWSE},
    {"GOOME_CURSOR_RESIZE_NESW", GOOME_CURSOR_RESIZE_NESW},
    {"GOOME_CURSOR_WAIT", GOOME_CURSOR_WAIT},
    {"GOOME_CURSOR_NO", GOOME_CURSOR_NO},
    {"GOOME_CURSOR_HIDDEN", GOOME_CURSOR_HIDDEN},
    {"GOOME_EDGE_LEFT", GOOME_EDGE_LEFT},
    {"GOOME_EDGE_RIGHT", GOOME_EDGE_RIGHT},
    {"GOOME_EDGE_TOP", GOOME_EDGE_TOP},
    {"GOOME_EDGE_BOTTOM", GOOME_EDGE_BOTTOM},
    {"GOOME_STATE_NORMAL", GOOME_STATE_NORMAL},
    {"GOOME_STATE_MINIMIZED", GOOME_STATE_MINIMIZED},
    {"GOOME_STATE_MAXIMIZED", GOOME_STATE_MAXIMIZED},
    {"GOOME_STATE_FULLSCREEN", GOOME_STATE_FULLSCREEN},
    {"GOOME_K_BACKSPACE", GOOME_K_BACKSPACE},
    {"GOOME_K_TAB", GOOME_K_TAB},
    {"GOOME_K_ENTER", GOOME_K_ENTER},
    {"GOOME_K_ESC", GOOME_K_ESC},
    {"GOOME_K_SPACE", GOOME_K_SPACE},
    {"GOOME_K_DELETE", GOOME_K_DELETE},
    {"GOOME_K_F1", GOOME_K_F1},
    {"GOOME_K_F12", GOOME_K_F12},
    {"GOOME_K_LEFT", GOOME_K_LEFT},
    {"GOOME_K_RIGHT", GOOME_K_RIGHT},
    {"GOOME_K_UP", GOOME_K_UP},
    {"GOOME_K_DOWN", GOOME_K_DOWN},
    {"GOOME_K_HOME", GOOME_K_HOME},
    {"GOOME_K_END", GOOME_K_END},
    {"GOOME_K_PGUP", GOOME_K_PGUP},
    {"GOOME_K_PGDN", GOOME_K_PGDN},
    {"GOOME_K_INSERT", GOOME_K_INSERT},
    {"GOOME_SHIFT", GOOME_SHIFT},
    {"GOOME_CTRL", GOOME_CTRL},
    {"GOOME_ALT", GOOME_ALT},
    {"GOOME_SUPER", GOOME_SUPER},
    {"GOOME_FMT_BGRA_PREMUL", GOOME_FMT_BGRA_PREMUL},
    {"GOOME_FMT_RGBA_PREMUL", GOOME_FMT_RGBA_PREMUL},
    {"GOOME_FMT_BGRA", GOOME_FMT_BGRA},
    {"GOOME_FMT_RGBA", GOOME_FMT_RGBA},
    {"GOOME_ICON_ARROW_DOWN", GOOME_ICON_ARROW_DOWN},
    {"GOOME_ICON_ARROW_RIGHT", GOOME_ICON_ARROW_RIGHT},
    {"GOOME_ICON_ARROW_UP", GOOME_ICON_ARROW_UP},
    {"GOOME_ICON_ARROW_LEFT", GOOME_ICON_ARROW_LEFT},
    {"GOOME_ICON_CHECK", GOOME_ICON_CHECK},
    {"GOOME_ICON_RADIO_DOT", GOOME_ICON_RADIO_DOT},
    {"GOOME_ICON_CLOSE", GOOME_ICON_CLOSE},
    {"GOOME_ICON_MINIMIZE", GOOME_ICON_MINIMIZE},
    {"GOOME_ICON_MAXIMIZE", GOOME_ICON_MAXIMIZE},
    {"GOOME_ICON_RESTORE", GOOME_ICON_RESTORE},
    {"GOOME_ICON_RESIZE", GOOME_ICON_RESIZE},
    {"GOOME_ICON_SORT_UP", GOOME_ICON_SORT_UP},
    {"GOOME_ICON_SORT_DOWN", GOOME_ICON_SORT_DOWN},
    {"GOOME_ICON_PLUS", GOOME_ICON_PLUS},
    {"GOOME_ICON_MINUS", GOOME_ICON_MINUS},
    {"GOOME_ICON_SLIDER_KNOB", GOOME_ICON_SLIDER_KNOB},
    {"GOOME_ICON_TOGGLE_KNOB", GOOME_ICON_TOGGLE_KNOB},
    {"GOOME_ICON_COLLAPSE", GOOME_ICON_COLLAPSE},
    {"GOOME_ICON_EXPAND", GOOME_ICON_EXPAND},
    {"GOOME_ICON_DRAG_HANDLE", GOOME_ICON_DRAG_HANDLE},
    {"GOOME_ICON_TOAST", GOOME_ICON_TOAST},
    {"GOOME_ICON_PICKER", GOOME_ICON_PICKER},
    {"GOOME_ICON_COUNT", GOOME_ICON_COUNT},
    {"GOOME_WIN_MOVE", GOOME_WIN_MOVE},
    {"GOOME_WIN_RESIZE", GOOME_WIN_RESIZE},
    {"GOOME_WIN_COLLAPSE", GOOME_WIN_COLLAPSE},
    {"GOOME_WIN_NO_TITLE", GOOME_WIN_NO_TITLE},
    {"GOOME_WIN_CLOSE", GOOME_WIN_CLOSE},
    {"GOOME_INPUT_PASSWORD", GOOME_INPUT_PASSWORD},
    {"GOOME_INPUT_READONLY", GOOME_INPUT_READONLY},
    {"GOOME_INPUT_NUMBER", GOOME_INPUT_NUMBER},
    {"GOOME_INPUT_NO_SELECT_ON_FOCUS", GOOME_INPUT_NO_SELECT_ON_FOCUS},
    {"GOOME_INPUT_ENTER_RETURNS", GOOME_INPUT_ENTER_RETURNS},
    {"GOOME_APP_TITLEBAR", GOOME_APP_TITLEBAR},
    {"GOOME_APP_MENUBAR", GOOME_APP_MENUBAR},
    {"GOOME_APP_STATUSBAR", GOOME_APP_STATUSBAR},
    {"GOOME_APP_NO_PAD", GOOME_APP_NO_PAD},
    {"GOOME_EDIT_READONLY", GOOME_EDIT_READONLY},
    {"GOOME_EDIT_LINE_NUMBERS", GOOME_EDIT_LINE_NUMBERS},
    {"GOOME_EDIT_WRAP", GOOME_EDIT_WRAP},
    {"GOOME_EDIT_MONO", GOOME_EDIT_MONO},
    {"GOOME_TABLE_SORTABLE", GOOME_TABLE_SORTABLE},
    {"GOOME_TABLE_RESIZABLE", GOOME_TABLE_RESIZABLE},
    {"GOOME_TABLE_STRIPES", GOOME_TABLE_STRIPES},
    {"GOOME_TABLE_BORDERS", GOOME_TABLE_BORDERS},
    {"GOOME_S_BG", GOOME_S_BG},
    {"GOOME_S_PANEL", GOOME_S_PANEL},
    {"GOOME_S_BORDER", GOOME_S_BORDER},
    {"GOOME_S_TEXT", GOOME_S_TEXT},
    {"GOOME_S_TEXT_DIM", GOOME_S_TEXT_DIM},
    {"GOOME_S_ACCENT", GOOME_S_ACCENT},
    {"GOOME_S_ACCENT_TEXT", GOOME_S_ACCENT_TEXT},
    {"GOOME_S_HOVER", GOOME_S_HOVER},
    {"GOOME_S_ACTIVE", GOOME_S_ACTIVE},
    {"GOOME_S_INPUT_BG", GOOME_S_INPUT_BG},
    {"GOOME_S_SHADOW", GOOME_S_SHADOW},
    {"GOOME_S_RADIUS", GOOME_S_RADIUS},
    {"GOOME_S_PAD", GOOME_S_PAD},
    {"GOOME_S_GAP", GOOME_S_GAP},
    {"GOOME_S_BORDER_WIDTH", GOOME_S_BORDER_WIDTH},
    {"GOOME_S_FONT_SIZE", GOOME_S_FONT_SIZE},
    {"GOOME_S_SHADOW_BLUR", GOOME_S_SHADOW_BLUR},
    {"GOOME_S_ANIM_MS", GOOME_S_ANIM_MS},
    {"GOOME_S_FOCUS", GOOME_S_FOCUS},
    {"GOOME_S_SELECT", GOOME_S_SELECT},
    {"GOOME_S_CARET", GOOME_S_CARET},
    {"GOOME_S_LINK", GOOME_S_LINK},
    {"GOOME_S_BADGE", GOOME_S_BADGE},
    {"GOOME_S_TOAST", GOOME_S_TOAST},
    {"GOOME_S_TOOLTIP", GOOME_S_TOOLTIP},
    {"GOOME_S_DANGER", GOOME_S_DANGER},
    {"GOOME_S_KNOB", GOOME_S_KNOB},
    {"GOOME_S_DIM", GOOME_S_DIM},
    {"GOOME_S_CHECK_SIZE", GOOME_S_CHECK_SIZE},
    {"GOOME_S_CHECK_RADIUS", GOOME_S_CHECK_RADIUS},
    {"GOOME_S_TOGGLE_W", GOOME_S_TOGGLE_W},
    {"GOOME_S_TOGGLE_H", GOOME_S_TOGGLE_H},
    {"GOOME_S_SLIDER_H", GOOME_S_SLIDER_H},
    {"GOOME_S_SLIDER_TRACK", GOOME_S_SLIDER_TRACK},
    {"GOOME_S_KNOB_SIZE", GOOME_S_KNOB_SIZE},
    {"GOOME_S_SCROLL_W", GOOME_S_SCROLL_W},
    {"GOOME_S_SCROLL_MIN", GOOME_S_SCROLL_MIN},
    {"GOOME_S_ITEM_H", GOOME_S_ITEM_H},
    {"GOOME_S_MENU_ITEM_H", GOOME_S_MENU_ITEM_H},
    {"GOOME_S_TAB_H", GOOME_S_TAB_H},
    {"GOOME_S_WIN_TITLE_H", GOOME_S_WIN_TITLE_H},
    {"GOOME_S_APP_TITLE_H", GOOME_S_APP_TITLE_H},
    {"GOOME_S_MENUBAR_H", GOOME_S_MENUBAR_H},
    {"GOOME_S_STATUSBAR_H", GOOME_S_STATUSBAR_H},
    {"GOOME_S_FOCUS_W", GOOME_S_FOCUS_W},
    {"GOOME_S_FOCUS_GAP", GOOME_S_FOCUS_GAP},
    {"GOOME_S_SEP_W", GOOME_S_SEP_W},
    {"GOOME_S_GRIP", GOOME_S_GRIP},
    {"GOOME_S_INDENT", GOOME_S_INDENT},
    {"GOOME_S_PLOT_W", GOOME_S_PLOT_W},
    {"GOOME_S_SPINNER", GOOME_S_SPINNER},
    {"GOOME_S_CARET_W", GOOME_S_CARET_W},
    {"GOOME_S_WIN_MIN_W", GOOME_S_WIN_MIN_W},
    {"GOOME_S_WIN_MIN_H", GOOME_S_WIN_MIN_H},
    {"GOOME_S_POP_GAP", GOOME_S_POP_GAP},
    {"GOOME_S_MENU_GAP", GOOME_S_MENU_GAP},
    {"GOOME_S_TIP_PAD_X", GOOME_S_TIP_PAD_X},
    {"GOOME_S_TIP_PAD_Y", GOOME_S_TIP_PAD_Y},
    {"GOOME_S_TIP_X", GOOME_S_TIP_X},
    {"GOOME_S_TIP_Y", GOOME_S_TIP_Y},
    {"GOOME_S_TIP_RADIUS", GOOME_S_TIP_RADIUS},
    {"GOOME_S_SHADOW_X", GOOME_S_SHADOW_X},
    {"GOOME_S_SHADOW_Y", GOOME_S_SHADOW_Y},
    {"GOOME_S_ALIGN", GOOME_S_ALIGN},
    {"GOOME_S_HEADING_SIZE", GOOME_S_HEADING_SIZE},
    {"GOOME_S_SMALL_SIZE", GOOME_S_SMALL_SIZE},
    {"GOOME_S_EASE", GOOME_S_EASE},
    {"GOOME_S_HOVER_MS", GOOME_S_HOVER_MS},
    {"GOOME_S_PRESS_MS", GOOME_S_PRESS_MS},
    {"GOOME_S_POPUP_MS", GOOME_S_POPUP_MS},
    {"GOOME_S_COLLAPSE_MS", GOOME_S_COLLAPSE_MS},
    {"GOOME_S_TOAST_FADE_MS", GOOME_S_TOAST_FADE_MS},
    {"GOOME_S_COUNT", GOOME_S_COUNT},
    {"GOOME_ALIGN_AUTO", GOOME_ALIGN_AUTO},
    {"GOOME_ALIGN_LEFT", GOOME_ALIGN_LEFT},
    {"GOOME_ALIGN_CENTER", GOOME_ALIGN_CENTER},
    {"GOOME_ALIGN_RIGHT", GOOME_ALIGN_RIGHT},
    {"GOOME_EASE_SMOOTH", GOOME_EASE_SMOOTH},
    {"GOOME_EASE_LINEAR", GOOME_EASE_LINEAR},
    {"GOOME_EASE_OUT", GOOME_EASE_OUT},
    {"GOOME_EASE_IN_OUT", GOOME_EASE_IN_OUT},
    {"GOOME_EASE_BACK", GOOME_EASE_BACK},
    {"GOOME_EASE_CUSTOM", GOOME_EASE_CUSTOM},
    {"GOOME_EASE_IN", GOOME_EASE_IN},
    {"GOOME_EASE_SPRING", GOOME_EASE_SPRING},
    {"GOOME_EASE_BOUNCE", GOOME_EASE_BOUNCE},
    {"GOOME_SKIN_BUTTON", GOOME_SKIN_BUTTON},
    {"GOOME_SKIN_PANEL", GOOME_SKIN_PANEL},
    {"GOOME_SKIN_INPUT", GOOME_SKIN_INPUT},
    {"GOOME_SKIN_CHECKBOX", GOOME_SKIN_CHECKBOX},
    {"GOOME_SKIN_RADIO", GOOME_SKIN_RADIO},
    {"GOOME_SKIN_TOGGLE", GOOME_SKIN_TOGGLE},
    {"GOOME_SKIN_SLIDER_TRACK", GOOME_SKIN_SLIDER_TRACK},
    {"GOOME_SKIN_SLIDER_FILL", GOOME_SKIN_SLIDER_FILL},
    {"GOOME_SKIN_SCROLLBAR", GOOME_SKIN_SCROLLBAR},
    {"GOOME_SKIN_TAB", GOOME_SKIN_TAB},
    {"GOOME_SKIN_TITLEBAR", GOOME_SKIN_TITLEBAR},
    {"GOOME_SKIN_TOOLTIP", GOOME_SKIN_TOOLTIP},
    {"GOOME_SKIN_MENU", GOOME_SKIN_MENU},
    {"GOOME_SKIN_POPUP", GOOME_SKIN_POPUP},
    {"GOOME_SKIN_COUNT", GOOME_SKIN_COUNT},
    {"GOOME_ITEM_DRAG_X", GOOME_ITEM_DRAG_X},
    {"GOOME_ITEM_DRAG_Y", GOOME_ITEM_DRAG_Y},
    {"GOOME_ITEM_FOCUSABLE", GOOME_ITEM_FOCUSABLE},
    {"GOOME_ITEM_NO_HIT", GOOME_ITEM_NO_HIT},
    {"GOOME_ITEM_REPEAT", GOOME_ITEM_REPEAT},
    {"GOOME_PRESET_DARK", GOOME_PRESET_DARK},
    {"GOOME_PRESET_LIGHT", GOOME_PRESET_LIGHT},
    {"GOOME_PRESET_MIDNIGHT", GOOME_PRESET_MIDNIGHT},
    {"GOOME_PRESET_PAPER", GOOME_PRESET_PAPER},
    {"GOOME_PRESET_MINT", GOOME_PRESET_MINT},
    {"GOOME_PRESET_HIGH_CONTRAST", GOOME_PRESET_HIGH_CONTRAST},
    {"GOOME_PRESET_ROSE", GOOME_PRESET_ROSE},
    {"GOOME_PRESET_COUNT", GOOME_PRESET_COUNT},
    {"GOOME_ACT_COPY", GOOME_ACT_COPY},
    {"GOOME_ACT_CUT", GOOME_ACT_CUT},
    {"GOOME_ACT_PASTE", GOOME_ACT_PASTE},
    {"GOOME_ACT_UNDO", GOOME_ACT_UNDO},
    {"GOOME_ACT_REDO", GOOME_ACT_REDO},
    {"GOOME_ACT_SELECT_ALL", GOOME_ACT_SELECT_ALL},
    {"GOOME_ACT_WORD_LEFT", GOOME_ACT_WORD_LEFT},
    {"GOOME_ACT_WORD_RIGHT", GOOME_ACT_WORD_RIGHT},
    {"GOOME_ACT_DEL_WORD_LEFT", GOOME_ACT_DEL_WORD_LEFT},
    {"GOOME_ACT_DEL_WORD_RIGHT", GOOME_ACT_DEL_WORD_RIGHT},
    {"GOOME_ACT_DOC_START", GOOME_ACT_DOC_START},
    {"GOOME_ACT_DOC_END", GOOME_ACT_DOC_END},
    {"GOOME_ACT_COUNT", GOOME_ACT_COUNT},
    {"GOOME_CUR_BUTTON", GOOME_CUR_BUTTON},
    {"GOOME_CUR_INPUT", GOOME_CUR_INPUT},
    {"GOOME_CUR_LINK", GOOME_CUR_LINK},
    {"GOOME_CUR_DRAG_VALUE", GOOME_CUR_DRAG_VALUE},
    {"GOOME_CUR_SPLIT_X", GOOME_CUR_SPLIT_X},
    {"GOOME_CUR_SPLIT_Y", GOOME_CUR_SPLIT_Y},
    {"GOOME_CUR_GRIP", GOOME_CUR_GRIP},
    {"GOOME_CUR_COLUMN", GOOME_CUR_COLUMN},
    {"GOOME_CUR_DRAG_DROP", GOOME_CUR_DRAG_DROP},
    {"GOOME_CUR_CUSTOM_X", GOOME_CUR_CUSTOM_X},
    {"GOOME_CUR_CUSTOM_Y", GOOME_CUR_CUSTOM_Y},
    {"GOOME_CUR_CUSTOM_XY", GOOME_CUR_CUSTOM_XY},
    {"GOOME_CUR_COUNT", GOOME_CUR_COUNT},
    {"GOOME_ROLE_NONE", GOOME_ROLE_NONE},
    {"GOOME_ROLE_WINDOW", GOOME_ROLE_WINDOW},
    {"GOOME_ROLE_BUTTON", GOOME_ROLE_BUTTON},
    {"GOOME_ROLE_CHECKBOX", GOOME_ROLE_CHECKBOX},
    {"GOOME_ROLE_RADIO", GOOME_ROLE_RADIO},
    {"GOOME_ROLE_SLIDER", GOOME_ROLE_SLIDER},
    {"GOOME_ROLE_TEXT", GOOME_ROLE_TEXT},
    {"GOOME_ROLE_INPUT", GOOME_ROLE_INPUT},
    {"GOOME_ROLE_COMBO", GOOME_ROLE_COMBO},
    {"GOOME_ROLE_LIST", GOOME_ROLE_LIST},
    {"GOOME_ROLE_ITEM", GOOME_ROLE_ITEM},
    {"GOOME_ROLE_TAB", GOOME_ROLE_TAB},
    {"GOOME_ROLE_MENU", GOOME_ROLE_MENU},
    {"GOOME_ROLE_MENUITEM", GOOME_ROLE_MENUITEM},
    {"GOOME_ROLE_TABLE", GOOME_ROLE_TABLE},
    {"GOOME_ROLE_TREE", GOOME_ROLE_TREE},
    {"GOOME_ROLE_PROGRESS", GOOME_ROLE_PROGRESS},
    {"GOOME_ROLE_IMAGE", GOOME_ROLE_IMAGE},
    {"GOOME_ROLE_GROUP", GOOME_ROLE_GROUP},
    {"GOOME_ROLE_DIALOG", GOOME_ROLE_DIALOG},
    {"GOOME_A11Y_FOCUSED", GOOME_A11Y_FOCUSED},
    {"GOOME_A11Y_CHECKED", GOOME_A11Y_CHECKED},
    {"GOOME_A11Y_DISABLED", GOOME_A11Y_DISABLED},
    {"GOOME_A11Y_EXPANDED", GOOME_A11Y_EXPANDED},
    {"GOOME_A11Y_SELECTED", GOOME_A11Y_SELECTED},
    {"GOOME_DIR_AUTO", GOOME_DIR_AUTO},
    {"GOOME_DIR_LTR", GOOME_DIR_LTR},
    {"GOOME_DIR_RTL", GOOME_DIR_RTL}
};

int ref_const_count(void)
{
    return (int)(sizeof(rf_consts) / sizeof(rf_consts[0]));
}

const char *ref_const_name(int i)
{
    return rf_consts[i].name;
}

int ref_const_value(int i)
{
    return rf_consts[i].value;
}

static uint32_t rf_fnv(const uint32_t *px, int w, int h, int stride)
{
    uint32_t v = 2166136261u;
    for (int y = 0; y < h; y++)
        for (int x = 0; x < w; x++) {
            uint32_t p = px[(size_t)y * stride + x];
            for (int k = 0; k < 4; k++) v = (v ^ ((p >> (k * 8)) & 255)) * 16777619u;
        }
    return v;
}

uint32_t ref_fnv(const uint32_t *px, int w, int h, int stride)
{
    return rf_fnv(px, w, h, stride);
}

uint32_t ref_basic(void)
{
    static uint32_t px[200 * 100];
    goome_canvas c;
    memset(&c, 0, sizeof(c));
    memset(px, 0, sizeof(px));
    c.px = px, c.w = 160, c.h = 100, c.stride = 200, c.scale = 1.5;
    goome_draw_round_rect(&c, 5, 5, 60, 40, 8, 0xC0203040);
    goome_draw_line(&c, 0, 0, 100, 60, 3, 0xFFFF0000);
    goome_draw_triangle(&c, 10, 50, 60, 50, 35, 10, 0x8000FF00);
    goome_draw_text(&c, 8, 8, "goome!", 1, 0xFFFFFFFF);
    goome_draw_frame(&c, 70, 10, 30, 30, 2, 0xFF00FFFF);
    return rf_fnv(px, 200, 100, 200);
}

static void rf_shape(goome_canvas *c, int k, const goome_bitmap *b)
{
    static const double star[] = {80, 5, 98, 85, 40, 35, 120, 35, 62, 85};
    switch (k) {
    case 0: goome_draw_rrect(c, 10.3, 8.6, 120, 70.2, (goome_radii){4, 20, 0, 33}, 0xE02060C0); break;
    case 1: goome_draw_rrect_line(c, 10.3, 8.6, 120, 70.2, goome_radius(18), 2.5, 0xFF20A060); break;
    case 2: goome_draw_rrect_grad(c, 10, 10, 130, 70, goome_radius(12), 0xFF102040, 0x8080C0FF, 0); break;
    case 3: goome_draw_rrect_grad(c, 10, 10, 130, 70, goome_radius(35), 0xFFFF4080, 0x004080FF, 1); break;
    case 4: goome_draw_shadow(c, 30, 20, 90, 50, 10, 16, 0x80000000); break;
    case 5: goome_draw_circle(c, 50.5, 50.2, 30.7, 0xFFFFC020), goome_draw_circle(c, 120, 30, 1.3, 0xFF000000); break;
    case 6: goome_draw_ring(c, 80, 50, 30, 5, 0xC0208080); break;
    case 7: goome_draw_arc(c, 50, 50, 30, 6, 0.3, 4.5, 0xFF3060E0), goome_draw_arc(c, 120, 50, 20, 30, 2, 2.5, 0xFF905020); break;
    case 8: goome_draw_poly(c, star, 5, 0xFF2040A0); break;
    case 9: goome_draw_bitmap(c, b, 10, 10, 100.5, 75.3, goome_radius(20), 0xFFFFFFFF); break;
    case 10: goome_draw_bitmap(c, b, 10, 10, 24, 16, goome_radius(0), 0x80FF8080); break;
    case 11: goome_draw_bitmap(c, b, 10, 10, 9, 5, goome_radius(2), 0xFFFFFFFF); break;
    case 12: goome_draw_bitmap_part(c, b, 4, 2, 8, 8, 100, 10, 40, 40, 0xFFFFFFFF); break;
    default:
        goome_clip_push(c, 20, 15, 80, 50);
        goome_draw_rrect(c, 0, 0, 160, 100, goome_radius(30), 0xFF4080FF);
        goome_clip_push(c, 50, 0, 100, 40);
        goome_draw_circle(c, 70, 40, 30, 0xFFFF4040);
        goome_draw_text(c, 30, 30, "clip", 1, 0xFF000000);
        goome_draw_shadow(c, 40, 20, 60, 30, 8, 10, 0xFF000000);
        goome_draw_bitmap(c, b, 40, 10, 48, 32, goome_radius(6), 0xFFFFFFFF);
        goome_clip_pop(c);
        goome_draw_line(c, 0, 0, 160, 100, 3, 0xFF00A000);
        goome_draw_poly(c, star, 5, 0x80000000);
        goome_clip_pop(c);
        goome_draw_frame(c, 20, 15, 80, 50, 1, 0x80000000);
        break;
    }
}

uint32_t ref_shape(int s, int k)
{
    static const double sc[3] = {1, 1.25, 2};
    static uint32_t px[323 * 200], bp[27 * 16];
    goome_bitmap b = {bp, 24, 16, 27, 0};
    for (int y = 0; y < 16; y++)
        for (int x = 0; x < 24; x++) {
            uint32_t a = (uint32_t)(x * 255 / 23), v = (x / 4 + y / 4) & 1 ? 0xFFFFFFu : 0x2050C0u;
            bp[y * 27 + x] = a << 24 | ((v >> 16 & 255) * a / 255) << 16 | ((v >> 8 & 255) * a / 255) << 8 | (v & 255) * a / 255;
        }
    goome_canvas c;
    memset(&c, 0, sizeof(c));
    memset(px, 0, sizeof(px));
    c.px = px, c.scale = sc[s], c.w = (int)(160 * sc[s]), c.h = (int)(100 * sc[s]), c.stride = c.w + 3;
    rf_shape(&c, k, &b);
    return rf_fnv(px, c.stride, c.h, c.stride);
}

static void rf_push(goome *g, int type, double x, double y, int key, const char *text)
{
    goome_event e;
    memset(&e, 0, sizeof(e));
    e.type = type, e.x = x, e.y = y, e.button = type == GOOME_DOWN || type == GOOME_UP ? 1 : 0, e.key = key;
    if (text) snprintf(e.text, sizeof(e.text), "%s", text);
    goome_push(g, &e);
}

int ref_frame(char *out, int cap)
{
    int err;
    goome *g = goome_open(&(goome_opts){.mode = GOOME_MODE_OFFSCREEN, .title = "engine", .width = 200, .height = 200, .scale = 2}, &err);
    if (!g) return -1;
    goome_canvas *c = goome_begin(g, 0);
    goome_draw_rect(c, 10, 10, 20, 20, 0x80FF0000);
    goome_draw_circle(c, 120, 50, 30, 0xFF3A7AFE);
    goome_draw_rrect(c, 40, 60, 70, 30, goome_radius(8), 0xCC20C060);
    goome_draw_text(c, 44, 66, "offscreen", 0, 0xFFFFFFFF);
    goome_rect hit = {10, 10, 20, 20};
    goome_end(g, c, &hit, 1);
    goome_image f;
    if (goome_frame(g, &f) != GOOME_OK) return -1;
    uint32_t draw = rf_fnv(f.px, f.w, f.h, f.stride);

    goome_ui *u = goome_ui_new(g);
    goome_ui_theme(u)->anim_ms = 0;
    goome_ui_theme(u)->font = NULL;
    goome_ui_theme(u)->font_size = 14;
    double wx = 6, wy = 6, ww = 180, wh = 0, val = 0.25;
    int check = 0;
    char name[32] = "hi";
    for (int frame = 0; frame < 6; frame++) {
        if (frame == 1) rf_push(g, GOOME_MOTION, 25, 64, 0, NULL), rf_push(g, GOOME_DOWN, 25, 64, 0, NULL), rf_push(g, GOOME_UP, 25, 64, 0, NULL);
        if (frame == 2) rf_push(g, GOOME_DOWN, 90, 168, 0, NULL), rf_push(g, GOOME_UP, 90, 168, 0, NULL);
        if (frame == 3) rf_push(g, GOOME_KEY, 0, 0, 'z', "z");
        goome_event ev;
        while (goome_poll(g, &ev, 0) > 0) goome_ui_feed(u, &ev);
        if (!goome_ui_begin(u, 0)) continue;
        if (goome_ui_window(u, "engine", &wx, &wy, &ww, &wh, GOOME_WIN_MOVE, NULL)) {
            goome_ui_checkbox(u, "Check", &check);
            goome_ui_slider(u, "Val", &val, 0, 1, "%.2f");
            goome_ui_input(u, "Name", name, sizeof(name));
        }
        goome_ui_window_end(u);
        goome_ui_end(u);
    }
    if (goome_frame(g, &f) != GOOME_OK) return -1;
    int n = snprintf(out, (size_t)cap, "draw %08x ui %08x %d %s %d", draw, rf_fnv(f.px, f.w, f.h, f.stride), check, name, goome_hit(g, 10, 10));
    goome_ui_free(u);
    goome_close(g);
    return n;
}

uint32_t ref_style(void)
{
    int err;
    goome *g = goome_open(&(goome_opts){.mode = GOOME_MODE_OFFSCREEN, .title = "style", .width = 320, .height = 260, .scale = 1.5}, &err);
    if (!g) return 0;
    goome_ui *u = goome_ui_new(g);
    goome_theme *t = goome_ui_theme(u);
    goome_ui_theme_preset(t, GOOME_PRESET_MINT);
    t->font = NULL, t->font_size = 14, t->anim_ms = 0;
    static uint32_t argb[8 * 8];
    for (int i = 0; i < 64; i++) argb[i] = i % 9 ? 0xFF20A0E0u : 0x80FFFFFFu;
    goome_bitmap bm;
    goome_bitmap_from_argb(argb, 8, 8, &bm);
    double wx = 10, wy = 10, ww = 280, wh = 0, drag = 0;
    int on = 1, open = 1, sel = 1;
    static const char *const tabs[] = {"One", "Two"};
    float plot[6] = {1, 3, 2, 5, 4, 6};
    for (int frame = 0; frame < 4; frame++) {
        if (frame == 1) rf_push(g, GOOME_MOTION, 60, 185, 0, NULL), rf_push(g, GOOME_DOWN, 60, 185, 0, NULL);
        if (frame == 2) rf_push(g, GOOME_MOTION, 90, 185, 0, NULL);
        if (frame == 3) rf_push(g, GOOME_UP, 90, 185, 0, NULL);
        goome_event ev;
        while (goome_poll(g, &ev, 0) > 0) goome_ui_feed(u, &ev);
        if (!goome_ui_begin(u, 0)) continue;
        if (goome_ui_window(u, "style", &wx, &wy, &ww, &wh, GOOME_WIN_MOVE, &open)) {
            goome_ui_push_color(u, GOOME_S_ACCENT, 0xFFFF6B8B);
            goome_ui_push_num(u, GOOME_S_RADIUS, 2);
            goome_ui_button(u, "Styled");
            goome_ui_pop(u, 2);
            goome_ui_push_icon(u, GOOME_ICON_CHECK, &bm);
            goome_ui_checkbox(u, "Icon", &on);
            goome_ui_pop(u, 1);
            goome_ui_tabs(u, "tabs", tabs, 2, &sel);
            goome_ui_item it;
            goome_ui_custom(u, "knob", 200, 30, GOOME_ITEM_DRAG_X, &it);
            drag += it.dx;
            goome_canvas *c = goome_ui_canvas(u);
            goome_draw_rrect(c, it.x, it.y, it.w, it.h, goome_radius(6), it.held ? 0xFF6FDC5Fu : 0xFF404858u);
            goome_draw_circle(c, it.x + 15 + drag, it.y + 15, 10, 0xFFFFFFFFu);
            goome_ui_plot_bars(u, "bars", plot, 6, 0, 6, 40);
            goome_ui_badge(u, "new", 0xFFE04040u);
            goome_ui_progress(u, 0.4, NULL);
        }
        goome_ui_window_end(u);
        goome_ui_end(u);
    }
    goome_image f;
    uint32_t h = goome_frame(g, &f) == GOOME_OK ? rf_fnv(f.px, f.w, f.h, f.stride) ^ (uint32_t)(int)drag : 0;
    goome_ui_free(u);
    goome_bitmap_free(&bm);
    goome_close(g);
    return h;
}

typedef struct {
    goome *g;
    goome_ui *u;
    int on, dropped;
    double box, sp, last;
    uint32_t col, hash;
} rf_am;

static void rf_am_frame(rf_am *k)
{
    goome_event ev;
    while (goome_poll(k->g, &ev, 0) > 0) goome_ui_feed(k->u, &ev);
    goome_canvas *c = goome_ui_begin(k->u, 0);
    if (!c) return;
    goome_canvas *r = goome_ui_canvas(k->u);
    k->box = goome_ui_anim(k->u, "box", k->on ? 200 : 10, 160, GOOME_EASE_OUT);
    k->sp = goome_ui_anim(k->u, "spring", k->on ? 1 : 0, 320, GOOME_EASE_SPRING);
    k->col = goome_ui_anim_color(k->u, "col", k->on ? 0xFF20D060u : 0x00000000u, 200, GOOME_EASE_LINEAR);
    if (!k->dropped) k->last = goome_ui_anim(k->u, "gone", k->on ? 5 : 0, 100, GOOME_EASE_LINEAR);
    if (r) {
        goome_draw_rect(r, k->box, 20, 60, 40, 0xFFE0A030u);
        goome_draw_rrect(r, 20 + k->sp * 180, 90, 50, 30, goome_radius(15), 0xFF4FB3F7u);
        goome_draw_rect(r, 20, 140, 260, 40, k->col);
    }
    goome_ui_end(k->u);
    k->hash = rf_fnv(c->px, c->w, c->h, c->stride);
}

int ref_anim(uint32_t *out)
{
    int err;
    rf_am k;
    memset(&k, 0, sizeof(k));
    k.g = goome_open(&(goome_opts){.backend = GOOME_TEST, .mode = GOOME_MODE_WINDOW, .width = 300, .height = 200, .test_script = "end 100000"}, &err);
    if (!k.g) return -1;
    k.u = goome_ui_new(k.g);
    rf_am_frame(&k);
    rf_am_frame(&k);
    k.on = 1;
    for (int frames = 0; frames < 60; frames++) {
        rf_am_frame(&k);
        if (frames == 1) out[0] = k.hash;
        if (frames == 4) out[1] = k.hash;
        if (frames == 8) out[2] = k.hash;
    }
    out[3] = k.hash;
    int done = k.box == 200 && k.sp == 1 && k.col == 0xFF20D060u && k.last == 5;
    goome_ui_free(k.u);
    goome_close(k.g);
    return done;
}

static uint32_t rf_txt(goome_font *f, double size, const char *s, uint32_t argb, int dir, int w, int h)
{
    static uint32_t px[400 * 120];
    goome_canvas c;
    memset(&c, 0, sizeof(c));
    memset(px, 0, sizeof(px));
    c.px = px, c.w = w, c.h = h, c.stride = 400, c.scale = 1, c.kx1 = w, c.ky1 = h;
    goome_text_dir(dir);
    goome_text(&c, f, size, 4, 4, s, argb);
    goome_text_dir(GOOME_DIR_AUTO);
    return rf_fnv(px, 400, 120, 400);
}

int ref_emoji(const unsigned char *colr, size_t ncolr, const unsigned char *cbdt, size_t ncbdt, const unsigned char *sbix, size_t nsbix,
              const unsigned char *sub, size_t nsub, uint32_t *out)
{
    goome_font *f = goome_font_load_mem(colr, ncolr, 0), *cb = goome_font_load_mem(cbdt, ncbdt, 0);
    goome_font *sb = goome_font_load_mem(sbix, nsbix, 0), *d = goome_font_load_mem(sub, nsub, 0);
    int ok = f && cb && sb && d;
    if (ok) {
        out[0] = rf_txt(f, 30, "ab \xF0\x9F\x98\x80 \xF0\x9F\x98\x80\xF0\x9F\x8F\xBB \xF0\x9F\x98\x80\xE2\x80\x8D\xF0\x9F\x98\x80 \xE2\x9D\xA4 \xE2\x9D\xA4\xEF\xB8\x8F", 0xFF2040A0, 0, 400, 60);
        static uint32_t px[400 * 120];
        goome_canvas c;
        memset(&c, 0, sizeof(c));
        memset(px, 0, sizeof(px));
        c.px = px, c.w = 400, c.h = 120, c.stride = 400, c.scale = 1;
        goome_text(&c, cb, 16, 4, 4, "\xF0\x9F\x90\xB1", 0xFFFFFFFF);
        goome_text(&c, cb, 64, 30, 4, "\xF0\x9F\x90\xB1", 0xFFFFFFFF);
        goome_text(&c, sb, 40, 120, 4, "\xF0\x9F\x90\xB1\xF0\x9F\x90\xB1", 0x80FFFFFF);
        out[1] = rf_fnv(px, 400, 120, 400);
        goome_font_fallback(d, f);
        out[2] = rf_txt(d, 20, "goome! \xF0\x9F\x98\x80 Hi", 0xFFFFFFFF, 0, 400, 40);
        out[3] = rf_txt(f, 24, "ab \xD7\x90\xD7\x91\xD7\x92 (12) c", 0xFFFFFFFF, 0, 400, 40);
        out[4] = rf_txt(f, 24, "ab \xD7\x90\xD7\x91\xD7\x92 (12) c", 0xFFFFFFFF, GOOME_DIR_RTL, 400, 40);
        out[5] = rf_txt(f, 30, "\xD8\xA8\xD9\x84\xD8\xA7\xD9\x85 \xD8\xA8\xD8\xA8\xD8\xA8 \xD9\x84\xD9\x85 \xD8\xA8\xD9\x8E\xD8\xA8", 0xFFFFFFFF, 0, 400, 50);
    }
    goome_font_free(d);
    goome_font_free(sb);
    goome_font_free(cb);
    goome_font_free(f);
    return ok;
}

typedef struct {
    goome *g;
    goome_ui *u;
    char in[64], ed[96];
    uint32_t hash;
} rf_im;

static void rf_im_frame(rf_im *k, int focus)
{
    goome_event ev;
    while (goome_poll(k->g, &ev, 0) > 0) goome_ui_feed(k->u, &ev);
    goome_canvas *c = goome_ui_begin(k->u, 0);
    if (!c) return;
    if (focus == 1) goome_ui_set_focus(k->u);
    goome_ui_input_ex(k->u, "in", k->in, sizeof(k->in), GOOME_INPUT_NO_SELECT_ON_FOCUS, NULL);
    if (focus == 2) goome_ui_set_focus(k->u);
    goome_ui_text_edit(k->u, "ed", k->ed, sizeof(k->ed), 80, 0);
    goome_ui_end(k->u);
    k->hash = rf_fnv(c->px, c->w, c->h, c->stride);
}

static int rf_im_open(rf_im *k, const char *in, const char *ed, int focus)
{
    int err;
    memset(k, 0, sizeof(*k));
    k->g = goome_open(&(goome_opts){.backend = GOOME_TEST, .mode = GOOME_MODE_WINDOW, .width = 300, .height = 200, .test_script = "end 100000"}, &err);
    if (!k->g) return 0;
    k->u = goome_ui_new(k->g);
    if (!k->u) return goome_close(k->g), 0;
    goome_theme *t = goome_ui_theme(k->u);
    t->font = NULL, t->font_size = 14, t->anim_ms = 0;
    goome_ui_cfg(k->u)->blink_ms = 0;
    snprintf(k->in, sizeof(k->in), "%s", in);
    snprintf(k->ed, sizeof(k->ed), "%s", ed);
    rf_im_frame(k, focus);
    rf_im_frame(k, 0);
    return 1;
}

static void rf_im_push(rf_im *k, int type, const char *s, int cur)
{
    goome_event e;
    memset(&e, 0, sizeof(e));
    e.type = type, e.id = cur;
    snprintf(e.msg, sizeof(e.msg), "%s", s);
    goome_push(k->g, &e);
    rf_im_frame(k, 0);
}

int ref_ime(uint32_t *out, char *in, int cap)
{
    rf_im k;
    if (!rf_im_open(&k, "ab", "", 1)) return -1;
    out[0] = k.hash;
    rf_im_push(&k, GOOME_PREEDIT, "xyz", 1);
    out[1] = k.hash;
    rf_im_push(&k, GOOME_PREEDIT, "xyz", 3);
    out[2] = k.hash;
    rf_im_push(&k, GOOME_PREEDIT, "", 0);
    rf_im_push(&k, GOOME_TEXT, "\xE6\x97\xA5\xE6\x9C\xAC", 0);
    snprintf(in, (size_t)cap, "%s", k.in);
    goome_ui_free(k.u);
    goome_close(k.g);
    if (!rf_im_open(&k, "", "line", 2)) return -1;
    out[3] = k.hash;
    rf_im_push(&k, GOOME_PREEDIT, "kana", 2);
    out[4] = k.hash;
    goome_ui_free(k.u);
    goome_close(k.g);
    return 0;
}
