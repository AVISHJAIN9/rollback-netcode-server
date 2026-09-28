use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

/// Fixed-point Q16.16 signed integer arithmetic primitive.
/// Complies with FR-001 for cross-platform deterministic physics simulation.
#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct FixedI32(pub i32);

impl FixedI32 {
    pub const FRACTIONAL_BITS: u32 = 16;
    pub const ONE: Self = Self(1 << 16);
    pub const ZERO: Self = Self(0);
    pub const HALF: Self = Self(1 << 15);

    #[inline(always)]
    pub const fn from_int(v: i32) -> Self {
        Self(v << 16)
    }

    #[inline(always)]
    pub const fn to_int(self) -> i32 {
        self.0 >> 16
    }

    #[inline(always)]
    pub const fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub const fn raw(self) -> i32 {
        self.0
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        if self.0 < min.0 {
            min
        } else if self.0 > max.0 {
            max
        } else {
            self
        }
    }
}

impl Add for FixedI32 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }
}

impl Sub for FixedI32 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }
}

impl Mul for FixedI32 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        let p = (self.0 as i64) * (rhs.0 as i64);
        Self((p >> 16) as i32)
    }
}

impl Div for FixedI32 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            return Self::ZERO;
        }
        let d = (self.0 as i64) << 16;
        Self((d / (rhs.0 as i64)) as i32)
    }
}

/// 2D Vector with FixedI32 coordinates
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2Fixed {
    pub x: FixedI32,
    pub y: FixedI32,
}

impl Vec2Fixed {
    pub const ZERO: Self = Self {
        x: FixedI32::ZERO,
        y: FixedI32::ZERO,
    };

    #[inline(always)]
    pub const fn new(x: FixedI32, y: FixedI32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2Fixed {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2Fixed {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
