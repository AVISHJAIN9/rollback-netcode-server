/// Circular ring buffer of fixed capacity (default 128 frames).
/// Complies with FR-005, FR-006, and FR-007.
pub struct RingBuffer<T: Clone + Default, const CAP: usize = 128> {
    slots: [T; CAP],
    checksums: [u64; CAP],
    head_frame: u64,
}

impl<T: Clone + Default, const CAP: usize> Default for RingBuffer<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Default, const CAP: usize> RingBuffer<T, CAP> {
    pub fn new() -> Self {
        Self {
            slots: std::array::from_fn(|_| T::default()),
            checksums: [0u64; CAP],
            head_frame: 0,
        }
    }

    #[inline(always)]
    fn slot_index(frame: u64) -> usize {
        (frame as usize) & (CAP - 1)
    }

    pub fn insert(&mut self, frame: u64, state: T, checksum: u64) {
        let idx = Self::slot_index(frame);
        self.slots[idx] = state;
        self.checksums[idx] = checksum;
        if frame > self.head_frame {
            self.head_frame = frame;
        }
    }

    pub fn get(&self, frame: u64) -> Option<&T> {
        if frame + (CAP as u64) <= self.head_frame || frame > self.head_frame {
            None
        } else {
            Some(&self.slots[Self::slot_index(frame)])
        }
    }

    pub fn get_checksum(&self, frame: u64) -> Option<u64> {
        if frame + (CAP as u64) <= self.head_frame || frame > self.head_frame {
            None
        } else {
            Some(self.checksums[Self::slot_index(frame)])
        }
    }
}
