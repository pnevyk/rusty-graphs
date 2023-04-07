use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Rem, Sub},
};

use num_traits::{One, Zero};

// OrderedFloat does not implement num_traits::Unsigned
#[derive(Debug, Default, Clone, Copy)]
pub struct UF32(f32);

impl UF32 {
    pub fn new(x: f32) -> Option<Self> {
        x.is_sign_positive().then_some(Self(x))
    }

    pub fn get(self) -> f32 {
        self.0
    }
}

impl PartialOrd for UF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for UF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for UF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Eq for UF32 {}

impl One for UF32 {
    fn one() -> Self {
        UF32(1.0)
    }
}

impl Zero for UF32 {
    fn zero() -> Self {
        UF32(0.0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
}

impl Add for UF32 {
    type Output = UF32;

    fn add(self, rhs: Self) -> Self::Output {
        UF32(self.0 + rhs.0)
    }
}

impl Mul for UF32 {
    type Output = UF32;

    fn mul(self, rhs: Self) -> Self::Output {
        UF32(self.0 * rhs.0)
    }
}

impl Sub for UF32 {
    type Output = UF32;

    fn sub(self, rhs: Self) -> Self::Output {
        UF32(self.0 - rhs.0)
    }
}

impl Div for UF32 {
    type Output = UF32;

    fn div(self, rhs: Self) -> Self::Output {
        UF32(self.0 / rhs.0)
    }
}

impl Rem for UF32 {
    type Output = UF32;

    fn rem(self, rhs: Self) -> Self::Output {
        UF32(self.0 % rhs.0)
    }
}

impl num_traits::Num for UF32 {
    type FromStrRadixErr = <f32 as num_traits::Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        f32::from_str_radix(str, radix).map(UF32)
    }
}

impl num_traits::Unsigned for UF32 {}
