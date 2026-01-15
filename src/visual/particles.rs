use bevy::prelude::*;

use crate::gameplay::combat::HitEvent;

#[derive(Component)]
pub struct SparkParticle {
    pub lifetime: f32,
    pub velocity: Vec3,
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_impact_sparks, update_particles));
    }
}

fn spawn_impact_sparks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hit_events: EventReader<HitEvent>,
) {
    for event in hit_events.read() {
        let spark_count = if event.blocked { 15 } else { 8 };
        let color = if event.blocked {
            Color::srgb(0.8, 0.8, 1.0)
        } else {
            Color::srgb(1.0, 0.6, 0.2)
        };

        for _ in 0..spark_count {
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let elevation = rand::random::<f32>() * 0.5 + 0.3;
            let speed = rand::random::<f32>() * 3.0 + 2.0;

            let velocity = Vec3::new(
                angle.cos() * speed * (1.0 - elevation),
                elevation * speed,
                angle.sin() * speed * (1.0 - elevation),
            );

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.05, 0.05, 0.05))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    emissive: color.into(),
                    ..default()
                })),
                Transform::from_translation(event.position),
                SparkParticle {
                    lifetime: 0.5,
                    velocity,
                },
            ));
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut SparkParticle)>,
) {
    let dt = time.delta_secs();
    let gravity = Vec3::new(0.0, -15.0, 0.0);

    for (entity, mut transform, mut particle) in particles.iter_mut() {
        particle.lifetime -= dt;

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            particle.velocity += gravity * dt;
            transform.translation += particle.velocity * dt;

            let scale = particle.lifetime * 2.0;
            transform.scale = Vec3::splat(scale.min(1.0));
        }
    }
}
