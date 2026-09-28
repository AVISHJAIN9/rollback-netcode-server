use crate::simulation::state::WorldState;

/// Blake3 SIMD state checksum verifier and desync incident detector.
/// Complies with FR-014 and FR-015.
pub struct DesyncDetector;

impl DesyncDetector {
    pub fn compute_checksum(state: &WorldState) -> u64 {
        let bytes = state.as_bytes();
        let hash = blake3::hash(bytes);
        let mut out = [0u8; 8];
        out.copy_from_slice(&hash.as_bytes()[0..8]);
        u64::from_le_bytes(out)
    }

    pub fn verify_state(
        frame: u64,
        server_state: &WorldState,
        client_checksum: u64,
    ) -> Result<(), String> {
        let server_checksum = Self::compute_checksum(server_state);
        if server_checksum != client_checksum {
            Err(format!(
                "Desync at frame {}: Server checksum {:X} != Client checksum {:X}",
                frame, server_checksum, client_checksum
            ))
        } else {
            Ok(())
        }
    }
}
