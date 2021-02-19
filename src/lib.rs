//! # flandres
//! fltk widgets adapted for Android.
//!
//! This crate provides public wrapper functions which return fltk widgets. These widgets are styled and adapted to Android screen resolution.
//!
//! # Usage
//! Add flandres to your fltk applications.
//! ```toml
//! [dependencies]
//! fltk = "0.15"
//! flandres = "0.2"
//! ```
//!
//! ```rust
//!     let mut win = flandres::window();
//!     let _inp = flandres::inout_widget::<fltk::input::Input>(200, 200, 200, 100, "");
//!     let _but: fltk::button::Button = flandres::widget(200, 600, 200, 100, "Click Me!");
//! ```

use fltk::*;

/// Returns an Android Window
pub fn window() -> window::Window {
    let (w, h) = app::screen_size();
    // let (w, h) = (600, 800);
    let mut win = fltk::window::Window::new(0, 30, w as i32, h as i32 - 30, "");
    win.set_color(Color::White);
    win
}

/// Returns a widget styled and adapted to Android screen resolution
pub fn widget<W: fltk::WidgetExt + WidgetBase>(x: i32, y: i32, w: i32, h: i32, label: &str) -> W {
    let mut w = W::new((x - w / 2) as i32, (y - h / 2) as i32, w * 2, h * 2, label);
    w.set_label_size(36);
    w.clear_visible_focus();
    w.set_frame(FrameType::RFlatBox);
    w.set_selection_color(Color::from_u32(0x12005e));
    w.set_label_color(Color::White);
    w.set_color(Color::from_u32(0x4a148c));
    w
}

/// Returns an input or output widget styled and adapted to Android screen resolution
pub fn inout_widget<IO: fltk::WidgetBase + fltk::InputExt>(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    label: &str,
) -> IO {
    let mut w = IO::new((x - w / 2) as i32, (y - h / 2) as i32, w * 2, h * 2, label);
    w.set_label_size(36);
    w.set_text_size(36);
    w
}

/// Returns a widget implementing DisplayExt (TextDisplay, TextEditor) styled and adapted to Android screen resolution
pub fn display_widget<Disp: fltk::WidgetBase + fltk::DisplayExt>(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    label: &str,
) -> Disp {
    let mut w = Disp::new((x - w / 2) as i32, (y - h / 2) as i32, w * 2, h * 2, label);
    w.set_label_size(36);
    w.set_text_size(36);
    w
}
