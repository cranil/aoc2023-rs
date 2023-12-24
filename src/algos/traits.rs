pub trait MinMax {
    const MIN: Self;
    const MAX: Self;

    fn min() -> Self
    where
        Self: Sized,
    {
        Self::MIN
    }
    fn max() -> Self
    where
        Self: Sized,
    {
        Self::MAX
    }
}

impl MinMax for i8 {
    const MIN: Self = std::i8::MIN;
    const MAX: Self = std::i8::MAX;
}
impl MinMax for i16 {
    const MIN: Self = std::i16::MIN;
    const MAX: Self = std::i16::MAX;
}
impl MinMax for i32 {
    const MIN: Self = std::i32::MIN;
    const MAX: Self = std::i32::MAX;
}
impl MinMax for i64 {
    const MIN: Self = std::i64::MIN;
    const MAX: Self = std::i64::MAX;
}
impl MinMax for i128 {
    const MIN: Self = std::i128::MIN;
    const MAX: Self = std::i128::MAX;
}
impl MinMax for isize {
    const MIN: Self = std::isize::MIN;
    const MAX: Self = std::isize::MAX;
}

impl MinMax for u8 {
    const MIN: Self = std::u8::MIN;
    const MAX: Self = std::u8::MAX;
}

impl MinMax for u16 {
    const MIN: Self = std::u16::MIN;
    const MAX: Self = std::u16::MAX;
}

impl MinMax for u32 {
    const MIN: Self = std::u32::MIN;
    const MAX: Self = std::u32::MAX;
}

impl MinMax for u64 {
    const MIN: Self = std::u64::MIN;
    const MAX: Self = std::u64::MAX;
}

impl MinMax for u128 {
    const MIN: Self = std::u128::MIN;
    const MAX: Self = std::u128::MAX;
}

impl MinMax for usize {
    const MIN: Self = std::usize::MIN;
    const MAX: Self = std::usize::MAX;
}

impl MinMax for f32 {
    const MIN: Self = std::f32::MIN;
    const MAX: Self = std::f32::MAX;
}

impl MinMax for f64 {
    const MIN: Self = std::f64::MIN;
    const MAX: Self = std::f64::MAX;
}
