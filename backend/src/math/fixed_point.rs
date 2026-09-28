/// Fixed-point Q16.16 signed integer arithmetic primitive.
/// Complies with FR-001 for cross-platform deterministic physics simulation.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedI32(pub i32);

impl FixedI32 {
    pub const FRACTIONAL_BITS: u32 = 16;
    pub const ONE: Self = Self(1 << 16);
    pub const ZERO: Self = Self(0);

    #[inline(always)]
    pub fn from_int(v: i32) -> Self {
        Self(v << 16)
    }

    #[inline(always)]
    pub fn to_int(self) -> i32 {
        self.0 >> 16
    }

    #[inline(always)]
    pub fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub fn raw(self) -> i32 {
        self.0
    }

    #[inline(always)]
    pub fn add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    #[inline(always)]
    pub fn sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    #[inline(always)]
    pub fn mul(self, rhs: Self) -> Self {
        let p = (self.0 as i64) * (rhs.0 as i64);
        Self((p >> 16) as i32)
    }

    #[inline(always)]
    pub fn div(self, rhs: Self) -> Self {
        assert!(rhs.0 != 0, "FixedI32 division by zero");
        let d = (self.0 as i64) << 16;
        Self((d / (rhs.0 as i64)) as i32)
    }
}
