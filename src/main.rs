// extern crate leaf;
// use crate::leaf::Manager;
use leaf::Manager;
use leaf::core::scene::scene::{
    Scene,
    Initialized,
    Uninitialized,
};
use leaf::specs;
pub use leaf::specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow};

use leaf::core::rendering::geometries::{
    triangle::TriangleGeometry,
    plane::PlaneGeometry,
    cube::CubeGeometry,
};
use leaf::core::rendering::geometries::geometry::Geometry;
use leaf::core::plugins::components::{
    InputComponent,
    DebugUiComponent,
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
    let mut app: leaf::Application = leaf::Application::create_application(log_level);
    app.startup();

    // create a scene and hold the id
    let main_scene_id = app.create_scene();

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
        scene.register::<InputComponent>();
        scene.register::<DebugUiComponent>();

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
            .with(RenderableComponent::create(Box::new(CubeGeometry::create(0.5, -0.5, 0.0, 0.5))))
            .with(TransformComponent::create_empty())
            .build();

        // camera
        scene.get_world()
            .unwrap()
            .create_entity()
            .with(CameraComponent::create_default())
            .with(InputComponent::create())
            .build();

        // ui panel
        scene.get_world()
            .unwrap()
            .create_entity()
            .with(DebugUiComponent::create())
            .build();
    }

    app.run();

}
