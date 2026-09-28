use crate::math::FixedI32;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityState {
    pub entity_id: u32,
    pub pos_x: FixedI32,
    pub pos_y: FixedI32,
    pub vel_x: FixedI32,
    pub vel_y: FixedI32,
    pub health: u16,
    pub flags: u16,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldState {
    pub frame_index: u64,
    pub entities: [EntityState; 4],
    pub rng_seed: u64,
}

impl WorldState {
    pub fn step_physics(&mut self, inputs: &[u16; 4]) {
        // Deterministic simulation update
        todo!("Implement deterministic kinematic update compliant with FR-001/FR-003")
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}
