/// Fixed point trait for computing fixed point calculations with native rust types.
pub trait FixedPoint: Sized {
    /// Safely calculates floor(x * y / denominator). Returns None if a phantom overflow
    /// occurs or if the denominator is 0.
    fn fixed_mul_floor(self, y: Self, denominator: Self) -> Option<Self>;

    /// Safely calculates ceil(x * y / denominator). Returns None if a phantom overflow
    /// occurs or if the denominator is 0.
    fn fixed_mul_ceil(self, y: Self, denominator: Self) -> Option<Self>;

    /// Safely calculates floor(x * denominator / y). Returns None if a phantom overflow
    /// occurs or if the denominator is 0.
    fn fixed_div_floor(self, y: Self, denominator: Self) -> Option<Self>;
}

impl FixedPoint for i128 {
    fn fixed_mul_floor(self, y: i128, denominator: i128) -> Option<i128> {
        mul_div_floor(self, y, denominator)
    }

    fn fixed_mul_ceil(self, y: i128, denominator: i128) -> Option<i128> {
        mul_div_ceil(self, y, denominator)
    }

    fn fixed_div_floor(self, y: i128, denominator: i128) -> Option<i128> {
        mul_div_floor(self, denominator, y)
    }
}

/// Performs floor(x * y / z)
pub(crate) fn mul_div_floor(x: i128, y: i128, z: i128) -> Option<i128> {
    let r = x.checked_mul(y)?;
    div_floor(r, z)
}

/// Performs floor(r / z)
fn div_floor(r: i128, z: i128) -> Option<i128> {
    if r < 0 || (r > 0 && z < 0) {
        // ceiling is taken by default for a negative result
        let remainder = r.checked_rem_euclid(z)?;
        (r / z).checked_sub(if remainder > 0 { 1 } else { 0 })
    } else {
        // floor taken by default for a positive or zero result
        r.checked_div(z)
    }
}

/// Performs ceil(x * y / z)
pub(crate) fn mul_div_ceil(x: i128, y: i128, z: i128) -> Option<i128> {
    let r = x.checked_mul(y)?;
    div_ceil(r, z)
}

/// Performs ceil(r / z)
fn div_ceil(r: i128, z: i128) -> Option<i128> {
    if r <= 0 || (r > 0 && z < 0) {
        // ceiling is taken by default for a negative or zero result
        r.checked_div(z)
    } else {
        // floor taken by default for a positive result
        let remainder = r.checked_rem_euclid(z)?;
        (r / z).checked_add(if remainder > 0 { 1 } else { 0 })
    }
}