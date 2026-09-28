use crate::simulation::ring_buffer::RingBuffer;
use crate::simulation::state::WorldState;

/// Rollback resimulation manager for late out-of-order remote inputs.
/// Complies with FR-008 and FR-009.
pub struct RollbackEngine {
    pub ring_buffer: RingBuffer<WorldState, 128>,
    pub current_frame: u64,
    pub confirmed_frame: u64,
    pub input_history: Vec<[u16; 2]>,
}

impl Default for RollbackEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RollbackEngine {
    pub fn new() -> Self {
        Self {
            ring_buffer: RingBuffer::new(),
            current_frame: 0,
            confirmed_frame: 0,
            input_history: Vec::with_capacity(512),
        }
    }

    pub fn process_remote_input(
        &mut self,
        remote_frame: u64,
        player_id: usize,
        input_bitmask: u16,
    ) -> Result<(), &'static str> {
        if remote_frame + 128 < self.current_frame {
            return Err("Input exceeds 128 frame rollback window");
        }
        if player_id >= 2 {
            return Err("Invalid player index");
        }

        while self.input_history.len() <= remote_frame as usize {
            self.input_history.push([0, 0]);
        }
        self.input_history[remote_frame as usize][player_id] = input_bitmask;
        Ok(())
    }
}
