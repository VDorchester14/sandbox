// extern crate leaf;
// use crate::leaf::Manager;
use std::env;
use std::collections::HashMap;
use leaf::Manager;
pub use leaf::specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow, saveload::*};
use cgmath::Vector3;
use leaf::core::plugins::components::{
    GeometryComponent,
    geometry_component::GeometryType,
};
use leaf::core::plugins::components::{
    InputComponent,
    DebugUiComponent,
    renderable_component::RenderableComponent,
    transform_component::TransformComponent,
    TransformUiComponent,
    camera_component::CameraComponent,
    light_components::DirectionalLightComponent,
    light_components::AmbientLightingComponent,
    terrain_component::TerrainComponent,
    terrain_component::TerrainUiComponent,
    SerializerFlag,
};

use log::{
    LevelFilter,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut log_level = Some(LevelFilter::Info);
    if args.len() > 1 {
        if args[1] == "debug"{
            log_level = Some(LevelFilter::Debug);
        }else if args[1] == "brooke"{
            println!("hi brooke");
        }else{
            println!("Get fucked that's not a valid log level");
        }
    }
    let mut app: leaf::Application = leaf::Application::create_application(log_level);
    app.startup();

    // // create a scene and hold the id
    // let main_scene_id = app.create_scene();

    // app.stage_scene(main_scene_id);

    
    // // TODO: take position off of geometry. it should be on transform
    
    // set that scene as active. using scope so the scene manager reference gets dropped
    // {
    //     let scene_manager = &mut app.get_scene_manager().unwrap();
    //     // scene_manager.set_active_scene(main_scene_id);
    //     let scene = &mut scene_manager.get_staged_scene().unwrap(); //
    //     scene.register::<TransformComponent>();
    //     scene.register::<TransformUiComponent>();
    //     scene.register::<RenderableComponent>();
    //     scene.register::<CameraComponent>();
    //     scene.register::<InputComponent>();
    //     scene.register::<DebugUiComponent>();
    //     scene.register::<DirectionalLightComponent>();
    //     scene.register::<AmbientLightingComponent>();
    //     scene.register::<TerrainComponent>();
    //     scene.register::<TerrainUiComponent>();
    //     scene.register::<GeometryComponent>();
    //     scene.register::<SimpleMarker<SerializerFlag>>();
    //     scene.insert_resource(SimpleMarkerAllocator::<SerializerFlag>::new());

    //     scene.get_world()
    //         .unwrap()
    //         .create_entity()
    //         .with(TerrainComponent::new(2 as usize))
    //         .with(
    //             TransformComponent::start()
    //                 .with_global_position(Vector3::new(0.0, 0.0, -0.5))
    //                 .build()
    //         )
    //         .build();

    //     scene.get_world()
    //         .unwrap()
    //         .create_entity()
    //         .with(RenderableComponent::create())
    //         .with(GeometryComponent::create(GeometryType::Box))
    //         .with(
    //             TransformComponent::start()
    //                 .with_global_position(Vector3::new(0.0, 0.0, 0.25))
    //                 .with_scale(0.2)
    //                 .build()
    //         )
    //         .marked::<SimpleMarker<SerializerFlag>>()
    //         .build();

    //     // camera
    //     scene.get_world()
    //         .unwrap()
    //         .create_entity()
    //         .with(CameraComponent::create_default())
    //         .with(InputComponent::create())
    //         .marked::<SimpleMarker<SerializerFlag>>()
    //         .build();

    //     // ui panel
    //     scene.get_world()
    //         .unwrap()
    //         .create_entity()
    //         .with(DebugUiComponent::create())
    //         .marked::<SimpleMarker<SerializerFlag>>()
    //         .build();

    //     // // dir light
    //     scene.get_world()
    //         .unwrap()
    //         .create_entity()
    //         .with(AmbientLightingComponent::new([1.0, 1.0, 1.0]))
    //         .marked::<SimpleMarker<SerializerFlag>>()
    //         .build();
        
    //         // // dir light
    //     scene.get_world()
    //         .unwrap()
    //         .create_entity()
    //         .with(DirectionalLightComponent::new(Vector3::new(-0.5, -0.2, -0.8), [1.0, 1.0, 1.0]))
    //         .build();
    // }
    // app.activate_staged_scene();
    app.run();

}
