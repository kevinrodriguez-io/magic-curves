/// Converts a floating-point number to a fixed-point representation.
///
/// This function takes a floating-point value and converts it to a fixed-point
/// representation with the specified number of decimal places.
///
/// # Arguments
///
/// * `value` - The floating-point value to convert.
/// * `decimals` - The number of decimal places to use in the fixed-point representation.
///
/// # Returns
///
/// A `u64` representing the fixed-point value.
///
/// # Examples
///
/// ```
/// use magic_curves::float_to_fixed_point;
///
/// let fixed = float_to_fixed_point(3.14159, 2);
/// assert_eq!(fixed, 314);
/// ```
pub fn float_to_fixed_point(value: f64, decimals: u8) -> u64 {
    let scale = 10u64.pow(decimals as u32);
    let scaled_value = value * scale as f64;
    scaled_value as u64
}

/// Converts a fixed-point number to a floating-point representation.
///
/// This function takes a fixed-point value and converts it to a floating-point
/// representation with the specified number of decimal places.
///
/// # Arguments
///
/// * `value` - The fixed-point value to convert.
/// * `decimals` - The number of decimal places used in the fixed-point representation.
///
/// # Returns
///
/// A `f64` representing the floating-point value.
///
/// # Examples
///
/// ```
/// use magic_curves::fixed_point_to_float;
/// 
/// let floating = fixed_point_to_float(314, 2);
/// assert_eq!(floating, 3.14);
/// ```
pub fn fixed_point_to_float(value: u64, decimals: u8) -> f64 {
    value as f64 / 10u64.pow(decimals as u32) as f64
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_float_to_fixed_point() {
        assert_eq!(crate::float_to_fixed_point(3.14159, 2), 314);
        assert_eq!(crate::float_to_fixed_point(0.0, 4), 0);
        assert_eq!(crate::float_to_fixed_point(1.23456789, 1), 12);
        assert_eq!(crate::float_to_fixed_point(0.123456789, 5), 12345);
    }

    #[test]
    fn test_fixed_point_to_float() {
        assert_eq!(crate::fixed_point_to_float(314, 2), 3.14);
        assert_eq!(crate::fixed_point_to_float(2718, 3), 2.718);
        assert_eq!(crate::fixed_point_to_float(0, 4), 0.0);
        assert_eq!(crate::fixed_point_to_float(12, 1), 1.2);
        assert_eq!(crate::fixed_point_to_float(12345, 5), 0.12345);
    }
}
