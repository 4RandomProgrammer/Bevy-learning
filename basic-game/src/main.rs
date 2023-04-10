use bevy::{input::mouse::MouseMotion,prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(start_camera)
    .add_startup_system(init_system)
    .add_system(update_system)
    // .add_system(mouse_motion)
    .run();
}

fn init_system(mut commands: Commands){
    commands.spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default());
}

fn update_system(mut controllers: Query<&mut KinematicCharacterController>){
    for mut controler in controllers.iter_mut(){
        controler.translation = Some(Vec3::new(1.0, -0.5, 1.0))
    }
}

fn read_result(controllers: Query<Entity, &KinematicCharacterControllerOutput>) {
    for (entity, output) in controllers.iter() {
        println!("Entity {:?} moved by {:?} and touches the ground: {:?}", entity, output.effective_translation, output.grounded)
    }
}

// fn mouse_motion(
//     mut motion_evr: EventReader<MouseMotion>,
// ) {
//     for ev in motion_evr.iter() {
//         println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
//     }
// }

fn start_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}