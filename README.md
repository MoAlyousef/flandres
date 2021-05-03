# flandres
fltk widgets adapted for Android.

This crate provides public wrapper functions which return fltk widgets. These widgets are styled and adapted to Android screen resolution.

# Usage
Add flandres to your fltk applications.
```toml
[dependencies]
fltk = "1"
flandres = "0.3"
```

```rust
    use fltk::{prelude::*, *};
    let mut win = flandres::window();
    let _inp = flandres::inout_widget::<input::Input>(200, 200, 200, 100, "");
    let _but: button::Button = flandres::widget(200, 600, 200, 100, "Click Me!");
```