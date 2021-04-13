#[macro_use]
extern crate leaf;

use crate::leaf::System;

fn main() {
    let mut app: leaf::Application = leaf::Application::create_application();
    app.startup();
}
