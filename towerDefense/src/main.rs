use bevy::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    //Default plugins tem tudo que é básico e ja vem com a Bevy. Porém a bevy não tem um motor de física.
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(WindowDescriptor {
        width: WIDTH,
        height: HEIGHT;
        title: "Bevy Tower Defense".to_string(),
        resizable: false,
        ..Default::default()

    })
    .add_plugins(DefaultPlugins)
    .run();
}
