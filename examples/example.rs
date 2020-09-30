use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = flandres::window();
    let _inp = flandres::inout_widget::<input::Input>(200, 200, 200, 100, "");
    let _but: button::Button = flandres::widget(200, 600, 200, 100, "Click Me!");
    win.end();
    win.show();
    app.run().unwrap();
}