// extern crate leaf;
// use crate::leaf::Manager;
use leaf::Manager;
use leaf::core::scene::scene::Scene;
use leaf::specs;
use leaf::glium;
pub use specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow};

use leaf::core::rendering;
use rendering::renderables::triangle::Triangle;
use leaf::core::plugins::components::renderable_component::RenderableComponent;


use glium::glutin;
pub struct DisplayWrapper(glium::Display);
use std::sync::Arc;
use log::{info};

fn main() {
    let mut app: leaf::Application = leaf::Application::create_application();
    app.startup();
    app.run();
}
