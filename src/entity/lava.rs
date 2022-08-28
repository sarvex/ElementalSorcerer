use bevy::{
    prelude::{Bundle, Commands, Component, Entity, EventReader, Query, Res, With},
    sprite::SpriteSheetBundle,
};
use bevy_ecs_ldtk::LdtkEntity;
use heron::CollisionEvent;

use crate::{
    abilities::{Element, Equipment},
    animation::Animated,
    damage::Killed,
    debug::DebugSettings,
    physics::PhysicsObjectBundle,
};

use super::player::Player;

#[derive(Component, Default)]
pub struct Lava;

#[derive(Bundle, LdtkEntity)]
pub struct LavaBundle {
    lava: Lava,
    #[bundle]
    #[sprite_sheet_bundle("sprites/lava.png", 16.0, 16.0, 9, 1, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[bundle]
    #[from_entity_instance]
    pub physics_bundle: PhysicsObjectBundle,
    #[from_entity_instance]
    pub animated: Animated,
}

pub fn check_collision(
    mut commands: Commands,
    lava: Query<Entity, With<Lava>>,
    player: Query<&Player>,
    mut collision_events: EventReader<CollisionEvent>,
    debug_settings: Res<DebugSettings>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|e| {
            let (e1, e2) = e.rigid_body_entities();
            if let Ok(player) = player.get(e1) {
                if lava.get(e2).is_ok()
                    && !debug_settings.imortality
                    && !(player.has_equipt(Equipment::Cloak) && player.has_infused(Element::Fire))
                {
                    return Some(e1);
                }
            } else if let Ok(player) = player.get(e2) {
                if lava.get(e1).is_ok()
                    && !debug_settings.imortality
                    && !(player.has_equipt(Equipment::Cloak) && player.has_infused(Element::Fire))
                {
                    return Some(e2);
                }
            }
            None
        })
        .for_each(|entity| {
            commands.entity(entity).insert(Killed {});
        });
}
