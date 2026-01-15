use bevy::{gltf::Gltf, prelude::*};

use super::components::Player;
use crate::gameplay::ai::Enemy;

pub const WALK_ANIMATION: &str = "Walking_A";
pub const RUN_ANIMATION: &str = "Running_B";
pub const ATTACK_ANIMATION: &str = "1H_Melee_Attack_Chop";
pub const JUMP_ANIMATION: &str = "Jump_Full_Short";
pub const IDLE_ANIMATION: &str = "Idle";
pub const HIT_ANIMATION: &str = "Hit_A";
pub const DEATH_ANIMATION: &str = "Death_A";
pub const BLOCK_ANIMATION: &str = "Blocking";

#[derive(Resource)]
pub struct KnightGltf(pub Handle<Gltf>);

#[derive(Resource)]
pub struct GameAnimations {
    pub graph: Handle<AnimationGraph>,
    pub idle_index: AnimationNodeIndex,
    pub walk_index: AnimationNodeIndex,
    pub run_index: AnimationNodeIndex,
    pub attack_index: AnimationNodeIndex,
    pub jump_index: AnimationNodeIndex,
    pub hit_index: AnimationNodeIndex,
    pub death_index: AnimationNodeIndex,
    pub block_index: AnimationNodeIndex,
}

#[derive(Component)]
pub struct AnimationSetupDone;

pub fn load_animations(
    mut commands: Commands,
    knight_gltf: Option<Res<KnightGltf>>,
    gltf_assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    existing_animations: Option<Res<GameAnimations>>,
) {
    if existing_animations.is_some() {
        return;
    }

    let Some(knight_gltf) = knight_gltf else {
        return;
    };

    let Some(gltf) = gltf_assets.get(&knight_gltf.0) else {
        return;
    };

    let get_clip = |name: &str| -> Option<Handle<AnimationClip>> {
        gltf.named_animations.get(name).cloned().or_else(|| {
            warn!(
                "Animation '{}' not found in Knight.glb. Available: {:?}",
                name,
                gltf.named_animations.keys().collect::<Vec<_>>()
            );
            None
        })
    };

    let Some(idle_clip) = get_clip(IDLE_ANIMATION) else { return };
    let Some(walk_clip) = get_clip(WALK_ANIMATION) else { return };
    let Some(run_clip) = get_clip(RUN_ANIMATION) else { return };
    let Some(attack_clip) = get_clip(ATTACK_ANIMATION) else { return };
    let Some(jump_clip) = get_clip(JUMP_ANIMATION) else { return };
    let Some(hit_clip) = get_clip(HIT_ANIMATION) else { return };
    let Some(death_clip) = get_clip(DEATH_ANIMATION) else { return };
    let Some(block_clip) = get_clip(BLOCK_ANIMATION) else { return };

    let mut graph = AnimationGraph::new();
    let idle_index = graph.add_clip(idle_clip, 1.0, graph.root);
    let walk_index = graph.add_clip(walk_clip, 1.0, graph.root);
    let run_index = graph.add_clip(run_clip, 1.0, graph.root);
    let attack_index = graph.add_clip(attack_clip, 1.0, graph.root);
    let jump_index = graph.add_clip(jump_clip, 1.0, graph.root);
    let hit_index = graph.add_clip(hit_clip, 1.0, graph.root);
    let death_index = graph.add_clip(death_clip, 1.0, graph.root);
    let block_index = graph.add_clip(block_clip, 1.0, graph.root);
    let graph_handle = graphs.add(graph);

    info!("Loaded animations from Knight.glb");

    commands.insert_resource(GameAnimations {
        graph: graph_handle,
        idle_index,
        walk_index,
        run_index,
        attack_index,
        jump_index,
        hit_index,
        death_index,
        block_index,
    });
}

pub fn setup_character_animations(
    mut commands: Commands,
    animations: Option<Res<GameAnimations>>,
    mut anim_players: Query<(Entity, &mut AnimationPlayer), Without<AnimationSetupDone>>,
    characters: Query<Entity, Or<(With<Player>, With<Enemy>)>>,
    children: Query<&Children>,
) {
    let Some(animations) = animations else {
        return;
    };

    for character_entity in characters.iter() {
        for entity in std::iter::once(character_entity).chain(children.iter_descendants(character_entity)) {
            if let Ok((anim_entity, _)) = anim_players.get_mut(entity) {
                commands.entity(anim_entity).insert((
                    AnimationGraphHandle(animations.graph.clone()),
                    AnimationSetupDone,
                    super::CurrentAnimation::default(),
                ));
                break;
            }
        }
    }
}
