use bevy::{
    input::{keyboard::KeyCode, system::exit_on_esc_system, Input},
    prelude::*,
};

#[derive(Component)]
struct Selected;

#[derive(Component)]
struct TroopUnit;

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(30.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::SALMON,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TroopUnit);
}

fn select_troop_keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    troop_unit_query: Query<(Entity, &TroopUnit)>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        for (entity, _troop) in troop_unit_query.iter() {
            let selected_entity = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.3, 0.5, 1.0),
                        custom_size: Some(Vec2::new(1.25, 1.25)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id();
            commands.entity(entity).push_children(&[selected_entity]);
            commands.entity(entity).insert(Selected);
        }
    }
}

fn deselect_troop_keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    selected_query: Query<(Entity, &Children)>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Key2) {
        for (entity, children) in selected_query.iter() {
            commands.entity(entity).remove::<Selected>();
            commands.entity(entity).remove_children(children);
            for child in children.iter() {
                commands.entity(*child).despawn();
            }
        }
    }
}

fn move_selected_click_system(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut selected_query: Query<(&Selected, &mut Transform)>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Right) {
        let size = Vec2::new(win.width() as f32, win.height() as f32);
        let default_orthographic_pos = size / 2.0;

        let mouse_pos = win.cursor_position().unwrap() - default_orthographic_pos;

        for (_selected, mut transform) in selected_query.iter_mut() {
            transform.translation.x = mouse_pos.x;
            transform.translation.y = mouse_pos.y;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.9, 0.4)))
        .add_startup_system(startup)
        .add_system(select_troop_keyboard_system)
        .add_system(deselect_troop_keyboard_system)
        .add_system(move_selected_click_system)
        .add_system(exit_on_esc_system)
        .run();
}
