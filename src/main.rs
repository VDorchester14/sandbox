// extern crate leaf;
// use crate::leaf::Manager;
use leaf::Manager;
use leaf::core::scene::scene::{
    Scene,
    Initialized,
    Uninitialized,
};
use leaf::specs;
pub use specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow};

use leaf::core::rendering;
use leaf::core::plugins::components::{
    renderable_component::RenderableComponent,
    transform_component::TransformComponent,
};


use std::sync::Arc;
use log::{
    info,
    LevelFilter,
};

fn main() {
    let log_level = Some(LevelFilter::Info);
    let mut app: leaf::Application = leaf::Application::create_application(log_level);
    app.startup();

    // create a scene and hold the id
    let main_scene_id = {
        let scene_manager = &mut app.get_scene_manager().unwrap();
        let new_scene_id = scene_manager.generate_and_register_scene();
        new_scene_id
    };

    // set that scene as active. using scope so the scene manager reference gets dropped
    {
        let scene_manager = &mut app.get_scene_manager().unwrap();
        scene_manager.set_active_scene(main_scene_id);
        let scene = &mut scene_manager.get_active_scene().unwrap(); //
        scene.register::<TransformComponent>();
        scene.register::<RenderableComponent>();
        scene.get_world()
            .unwrap()
            .create_entity()
            .with(
                RenderableComponent{
                    vertex_buffer: None,
                    geometry: None,
                    initialized: false,
                }
            )
            .build();
    }

    app.run();

}
