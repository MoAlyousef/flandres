use fltk::*;

pub fn window() -> window::Window {
    let (w, h) = app::screen_size();
    // let (w, h) = (600, 800);
    let mut win = fltk::window::Window::new(0, 30, w as i32, h as i32 - 30, "");
    win.set_color(Color::White);
    win
}

pub fn widget<W: fltk::WidgetExt>(x: i32, y: i32, w: i32, h: i32, label: &str) -> W {
    let mut w = W::new(
        (x - w/2) as i32,
        (y - h/2) as i32,
        w * 2,
        h * 2,
        label,
    );
    w.set_label_size(36);
    w.clear_visible_focus();
    w.set_frame(FrameType::RFlatBox);
    w.set_selection_color(Color::from_u32(0x1d0c6b));
    w.set_label_color(Color::White);
    w.set_color(Color::from_u32(0x311B92));
    w
}

pub fn inout_widget<IO: fltk::InputExt>(x: i32, y: i32, w: i32, h: i32, label: &str) -> IO {
    let mut w = IO::new(
        (x - w/2) as i32,
        (y - h/2) as i32,
        w * 2,
        h * 2,
        label,
    );
    w.set_frame(FrameType::RoundedBox);
    w.set_label_size(36);
    w.set_text_size(36);
    w
}

pub fn display_widget<Disp: fltk::DisplayExt>(x: i32, y: i32, w: i32, h: i32, label: &str) -> Disp {
    let mut w = Disp::new(
        (x - w/2) as i32,
        (y - h/2) as i32,
        w * 2,
        h * 2,
        label,
    );
    w.set_frame(FrameType::RoundedBox);
    w.set_label_size(36);
    w.set_text_size(36);
    w
}

