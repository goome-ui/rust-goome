use crate::{sys, Goome, Pacer, Result, Ui, CLOSE, QUIT};
use std::ops::{Deref, DerefMut};

/// A ready made app window: open it, loop on `frame`, build your ui inside. Handles pacing, redraws and close for you.
pub struct App {
    ui: Ui,
    /// Window title, also the title of the root panel.
    pub title: String,
    /// `APP_*` flags for the root panel.
    pub flags: i32,
    /// How long to sleep between polls while nothing needs a redraw.
    pub idle_ms: i32,
    /// Keep redrawing every frame, for things that always move.
    pub animate: bool,
    /// How many frames it has drawn.
    pub frames: u64,
    /// The events from the current frame.
    pub events: Vec<crate::Event>,
    pace: Pacer,
    running: bool,
    dirty: i32,
    every: bool,
    close_asked: bool,
    keep_open: bool,
}

impl App {
    /// Opens a normal resizable window with that title and logical size.
    pub fn new(title: &str, w: i32, h: i32) -> Result<App> {
        App::from_goome(Goome::app(title, w, h)?, title)
    }

    /// Wraps a goome you opened yourself, like an offscreen or test one.
    pub fn from_goome(g: Goome, title: &str) -> Result<App> {
        let every = g.backend_name().split_whitespace().next() == Some("test");
        Ok(App {
            ui: Ui::new(g)?,
            title: title.to_string(),
            flags: 0,
            idle_ms: 500,
            animate: false,
            frames: 0,
            events: Vec::new(),
            pace: Pacer::new(60),
            running: true,
            dirty: 2,
            every,
            close_asked: false,
            keep_open: false,
        })
    }

    /// Sets the target frame rate while redrawing.
    pub fn fps(&mut self, fps: i32) {
        self.pace.fps = fps;
    }

    /// The [`Ui`] under it.
    pub fn ui(&mut self) -> &mut Ui {
        &mut self.ui
    }

    /// Asks for a redraw on the next frame.
    pub fn redraw(&mut self) {
        self.dirty = self.dirty.max(1);
    }

    /// Ends the `frame` loop.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// False once something asked to quit.
    pub fn running(&self) -> bool {
        self.running
    }

    /// If on, the close button doesnt quit, check `close_asked` and decide yourself.
    pub fn keep_open_on_close(&mut self, on: bool) {
        self.keep_open = on;
    }

    /// True once after the close button was pressed.
    pub fn close_asked(&mut self) -> bool {
        std::mem::replace(&mut self.close_asked, false)
    }

    /// Waits for the next frame to draw and starts it. `None` = the app is done. The frame ends when it drops.
    pub fn frame(&mut self) -> Option<AppFrame<'_>> {
        loop {
            if !self.running {
                return None;
            }
            let busy = self.every || self.animate || self.dirty > 0 || self.ui.animating();
            if self.dirty > 0 {
                self.dirty -= 1;
            }
            let wait = if busy { self.pace.wait() } else { self.idle_ms };
            self.events.clear();
            let mut t = wait;
            while let Some(ev) = self.ui.poll(t) {
                t = 0;
                self.ui.feed(&ev);
                if ev.kind == QUIT {
                    self.running = false;
                } else if ev.kind == CLOSE {
                    self.close_asked = true;
                    if !self.keep_open {
                        self.running = false;
                    }
                }
                self.events.push(ev);
                self.dirty = self.dirty.max(1);
                if !self.running {
                    return None;
                }
            }
            if busy || self.dirty > 0 {
                break;
            }
        }
        if unsafe { sys::goome_ui_begin(self.ui.raw(), 0) }.is_null() {
            return None;
        }
        let t = crate::cstr(&self.title);
        let shown = unsafe { sys::goome_ui_app(self.ui.raw(), t.as_ptr(), self.flags) } != 0;
        self.frames += 1;
        Some(AppFrame { app: self, shown })
    }

    /// Gives the [`Ui`] back.
    pub fn into_ui(self) -> Ui {
        self.ui
    }
}

/// One frame of an [`App`]. Derefs to [`Ui`], the frame ends when it drops.
pub struct AppFrame<'a> {
    app: &'a mut App,
    shown: bool,
}

impl AppFrame<'_> {
    /// False if the root panel isnt on screen, you can skip your widgets.
    pub fn shown(&self) -> bool {
        self.shown
    }

    /// The events that came in for this frame.
    pub fn events(&self) -> &[crate::Event] {
        &self.app.events
    }

    /// Ends the app loop.
    pub fn quit(&mut self) {
        self.app.running = false;
    }
}

impl Deref for AppFrame<'_> {
    type Target = Ui;

    fn deref(&self) -> &Ui {
        &self.app.ui
    }
}

impl DerefMut for AppFrame<'_> {
    fn deref_mut(&mut self) -> &mut Ui {
        &mut self.app.ui
    }
}

impl Drop for AppFrame<'_> {
    fn drop(&mut self) {
        unsafe { sys::goome_ui_app_end(self.app.ui.raw()) };
        self.app.ui.end_frame();
    }
}
