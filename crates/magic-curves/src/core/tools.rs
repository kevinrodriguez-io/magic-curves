pub fn float_to_fixed_point(value: f64, decimals: u8) -> u64 {
    let scale = 10u64.pow(decimals as u32);
    let scaled_value = value * scale as f64;
    scaled_value as u64
}

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
