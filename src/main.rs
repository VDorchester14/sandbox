// extern crate ember;
// use crate::ember::Manager;
use ember::Manager;
use ember::core::scene::scene::{
    Scene,
    Initialized,
    Uninitialized,
};
use ember::specs;
pub use ember::specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow};

use ember::core::rendering::geometries::{
    triangle::TriangleGeometry,
    plane::PlaneGeometry,
    cube::CubeGeometry,
};
use ember::core::rendering::geometries::geometry::Geometry;
use ember::core::plugins::components::{
    renderable_component::RenderableComponent,
    transform_component::TransformComponent,
    camera_component::CameraComponent,
};

use std::sync::Arc;
use log::{
    info,
    LevelFilter,
};

use cgmath::{Vector3, Matrix4};

fn main() {
    let log_level = Some(LevelFilter::Debug);
    let mut app: ember::Application = ember::Application::create_application(log_level);
    app.startup();

    // create a scene and hold the id
    let main_scene_id = {
        let scene_manager = &mut app.get_scene_manager().unwrap();
        let new_scene_id = scene_manager.generate_and_register_scene();
        new_scene_id
    };

    //
    // TODO: take position off of geometry. it should be on transform
    //
    // set that scene as active. using scope so the scene manager reference gets dropped
    {
        let scene_manager = &mut app.get_scene_manager().unwrap();
        scene_manager.set_active_scene(main_scene_id);
        let scene = &mut scene_manager.get_active_scene().unwrap(); //
        scene.register::<TransformComponent>();
        scene.register::<RenderableComponent>();
        scene.register::<CameraComponent>();
        let geometry = TriangleGeometry::create(0.0, 0.0, 0.0, 0.1);
        scene.get_world()
            .unwrap()
            .create_entity()
            .with(RenderableComponent::create(Box::new(geometry)))
            .with(TransformComponent::create_empty())
            .build();

        scene.get_world()
            .unwrap()
            .create_entity()
            .with(RenderableComponent::create(Box::new(PlaneGeometry::create(-0.5, 0.5, 0.0, 0.2))))
            .with(TransformComponent::create_empty())
            .build();

        scene.get_world()
            .unwrap()
            .create_entity()
            .with(RenderableComponent::create(Box::new(CubeGeometry::create(0.5, -0.5, 0.0, 0.2))))
            .with(TransformComponent::create_empty())
            .build();
    }

    app.run();

}
