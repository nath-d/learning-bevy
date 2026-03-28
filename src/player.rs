use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn 
}



fn game_setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("@"),
        TextFont {
            font_size: 12.0,
            font: default(),
            ..default()
        },
        TextColor(Color::srgba(0.0, 1.0, 0.0, 1.0)), //(r, g, b, alpha)
        Transform::from_translation(Vec3::ZERO),
        Player,
        ));
}

#[derive(Component)]
struct Player;

fn move_player(
    input: Res<ButtonInput<KeyCode>>, 
    time: Res<Time>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    ) {

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let speed = 300.0; //pixels per second
        let delta = direction.normalize() * speed * time.delta_secs(); 
        //because direction is a vector, multiplication produces vector
        
        player_transform.translation.x += delta.x;
        player_transform.translation.y += delta.y;
    }


}