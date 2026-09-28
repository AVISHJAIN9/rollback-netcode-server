use serde::{Serialize, Deserialize};

/// Binary input datagram sent via UDP from client to server.
/// Complies with FR-011 (bundles current frame input + 4 historical frames).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientInputPacket {
    pub session_token: u32,
    pub player_id: u32,
    pub target_frame: u32,
    pub input_bitmask: u16,
    pub analog_dx_q16: i32,
    pub analog_dy_q16: i32,
    pub history_inputs: u64,
    pub state_checksum: u64,
}

impl ClientInputPacket {
    pub fn encode_to_bytes(&self) -> Vec<u8> {
        // Staged for zero-alloc byte packing
        todo!("Implement byte packing encoder compliant with FR-011")
    }

    pub fn decode_from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        // Staged for zero-alloc byte unpacking
        todo!("Implement byte unpacking decoder compliant with FR-011")
    }
}
