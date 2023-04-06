use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

struct Player {
    velocity: Vec2,
}

struct Enemy;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_player.system())
        .add_system(move_enemy.system())
        .add_system(detect_collision.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the sprite texture
    let texture_handle = asset_server.load("sprite.png");

    // Spawn the player sprite
    commands.spawn_bundle(SpriteBundle {
        material: materials::Texture::from(texture_handle),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: Sprite::new(Vec2::new(64.0, 64.0)),
        ..Default::default()
    })
    .insert(Player { velocity: Vec2::new(0.0, 0.0) });

    // Spawn the enemy sprite
    commands.spawn_bundle(SpriteBundle {
        material: materials::Texture::from(texture_handle),
        transform: Transform::from_translation(Vec3::new(200.0, 0.0, 0.0)),
        sprite: Sprite::new(Vec2::new(64.0, 64.0)),
        ..Default::default()
    })
    .insert(Enemy);
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player, With<Player>>,
) {
    for mut player in query.iter_mut() {
        player.velocity = Vec2::new(0.0, 0.0);

        if keyboard_input.pressed(KeyCode::Left) {
            player.velocity.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player.velocity.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            player.velocity.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            player.velocity.y += 1.0;
        }
    }
}

fn move_enemy(mut query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 2.0;
    }
}

fn detect_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &Sprite, &Player), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
) {
    for (player_entity, player_transform, player_sprite, player) in player_query.iter_mut() {
        for (enemy_entity, enemy_transform, enemy_sprite) in enemy_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                enemy_transform.translation,
                enemy_sprite.size,
            );

            if let Some(collision) = collision {
                commands.entity(player_entity).despawn();

                commands.entity(enemy_entity).despawn();

                println!("Collision detected! {:?}", collision);
            }
        }
    }
}