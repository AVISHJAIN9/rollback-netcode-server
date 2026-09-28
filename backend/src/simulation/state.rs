use crate::math::{FixedI32, Vec2Fixed};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntityState {
    pub entity_id: u32,
    pub position: Vec2Fixed,
    pub velocity: Vec2Fixed,
    pub health: u16,
    pub attack_cooldown: u8,
    pub flags: u16,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorldState {
    pub frame_index: u64,
    pub entities: [EntityState; 2],
    pub rng_seed: u64,
}

impl WorldState {
    pub fn step_physics(&mut self, inputs: &[u16; 2]) {
        self.frame_index += 1;
        for (i, entity) in self.entities.iter_mut().enumerate() {
            let input = inputs[i];
            let mut move_x = FixedI32::ZERO;
            if (input & (1 << 0)) != 0 {
                // Left
                move_x = move_x - FixedI32::from_int(4);
            }
            if (input & (1 << 1)) != 0 {
                // Right
                move_x = move_x + FixedI32::from_int(4);
            }
            entity.velocity.x = move_x;
            entity.position.x = entity.position.x + entity.velocity.x;
        }
    }

    pub fn compute_checksum(&self) -> u32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.frame_index.to_be_bytes());
        for entity in &self.entities {
            hasher.update(&entity.entity_id.to_be_bytes());
            hasher.update(&entity.position.x.raw().to_be_bytes());
            hasher.update(&entity.position.y.raw().to_be_bytes());
            hasher.update(&entity.velocity.x.raw().to_be_bytes());
            hasher.update(&entity.velocity.y.raw().to_be_bytes());
            hasher.update(&entity.health.to_be_bytes());
            hasher.update(&entity.flags.to_be_bytes());
        }
        hasher.update(&self.rng_seed.to_be_bytes());
        let hash = hasher.finalize();
        let bytes = hash.as_bytes();
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}
