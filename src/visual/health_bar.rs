use bevy::prelude::*;

use crate::core::camera::FollowCamera;
use crate::gameplay::ai::Enemy;
use crate::gameplay::combat::Stamina;
use crate::gameplay::health::Health;
use crate::gameplay::player::Player;

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct PlayerStaminaBar;

#[derive(Component)]
pub struct EnemyHealthBar {
    pub enemy: Entity,
}

#[derive(Component)]
pub struct EnemyHealthBarFill {
    pub enemy: Entity,
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player_health_ui)
            .add_systems(
                Update,
                (
                    update_player_health_bar,
                    update_player_stamina_bar,
                    spawn_enemy_health_bars,
                    update_enemy_health_bars,
                ),
            );
    }
}

fn update_player_stamina_bar(
    player_query: Query<&Stamina, With<Player>>,
    mut stamina_bar_query: Query<&mut Node, With<PlayerStaminaBar>>,
) {
    let Ok(stamina) = player_query.get_single() else {
        return;
    };
    let Ok(mut bar_node) = stamina_bar_query.get_single_mut() else {
        return;
    };

    let stamina_percent = (stamina.current / stamina.max * 100.0).max(0.0);
    bar_node.width = Val::Percent(stamina_percent);
}

fn setup_player_health_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Health"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|bar_bg| {
                    bar_bg.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                        PlayerHealthBar,
                    ));
                });

            // Stamina label
            parent.spawn((
                Text::new("Stamina"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Stamina bar
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        margin: UiRect::top(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|bar_bg| {
                    bar_bg.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.85, 0.65, 0.1)),
                        PlayerStaminaBar,
                    ));
                });
        });
}

fn update_player_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Node, With<PlayerHealthBar>>,
) {
    let Ok(health) = player_query.get_single() else {
        return;
    };
    let Ok(mut bar_node) = health_bar_query.get_single_mut() else {
        return;
    };

    let health_percent = (health.current / health.max * 100.0).max(0.0);
    bar_node.width = Val::Percent(health_percent);
}

fn spawn_enemy_health_bars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemies: Query<Entity, (With<Enemy>, Without<EnemyHealthBar>)>,
    health_bars: Query<&EnemyHealthBar>,
) {
    for enemy_entity in enemies.iter() {
        let has_bar = health_bars.iter().any(|bar| bar.enemy == enemy_entity);
        if has_bar {
            continue;
        }

        let bg_mesh = meshes.add(Cuboid::new(1.0, 0.1, 0.05));
        let bg_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.2),
            unlit: true,
            ..default()
        });

        let fill_mesh = meshes.add(Cuboid::new(1.0, 0.1, 0.05));
        let fill_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            unlit: true,
            ..default()
        });

        commands.spawn((
            Mesh3d(bg_mesh),
            MeshMaterial3d(bg_material),
            Transform::from_xyz(0.0, 2.5, 0.0),
            EnemyHealthBar { enemy: enemy_entity },
        ));

        commands.spawn((
            Mesh3d(fill_mesh),
            MeshMaterial3d(fill_material),
            Transform::from_xyz(0.0, 2.5, 0.03),
            EnemyHealthBarFill { enemy: enemy_entity },
        ));
    }
}

fn update_enemy_health_bars(
    mut bar_query: Query<(&mut Transform, &EnemyHealthBar), Without<EnemyHealthBarFill>>,
    mut fill_query: Query<(&mut Transform, &EnemyHealthBarFill), Without<EnemyHealthBar>>,
    enemy_query: Query<
        (&Transform, &Health),
        (With<Enemy>, Without<EnemyHealthBar>, Without<EnemyHealthBarFill>),
    >,
    camera_query: Query<
        &Transform,
        (With<FollowCamera>, Without<Enemy>, Without<EnemyHealthBar>, Without<EnemyHealthBarFill>),
    >,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    for (mut bar_transform, health_bar) in bar_query.iter_mut() {
        if let Ok((enemy_transform, _health)) = enemy_query.get(health_bar.enemy) {
            bar_transform.translation = enemy_transform.translation + Vec3::Y * 2.5;
            bar_transform.look_at(camera_transform.translation, Vec3::Y);
        }
    }

    for (mut fill_transform, health_bar_fill) in fill_query.iter_mut() {
        if let Ok((enemy_transform, health)) = enemy_query.get(health_bar_fill.enemy) {
            let health_percent = health.current / health.max;
            fill_transform.translation = enemy_transform.translation + Vec3::Y * 2.5;
            fill_transform.look_at(camera_transform.translation, Vec3::Y);
            fill_transform.scale.x = health_percent.max(0.0);
            let local_forward = fill_transform.forward();
            let local_right = local_forward.cross(Vec3::Y).normalize();
            fill_transform.translation -= local_right * (1.0 - health_percent) * 0.5;
        }
    }
}
