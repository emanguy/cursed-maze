use std::f64::consts::PI;
use std::ops::Range;

pub const TWO_PI: f64 = 2.0 * PI;

pub fn normalize_range(original_angle: f64, range: Range<f64>) -> f64 {
    let range_width = range.end - range.start;
    let aligned_angle = original_angle - range.start;
    let normalized_angle = aligned_angle % range_width;
    let normalized_angle = if normalized_angle > 0.0 { normalized_angle } else { normalized_angle + range_width };

    return normalized_angle + range.start;
}

#[cfg(test)]
mod tests {
    #[test]
    fn wraps_around_end() {
        assert_eq!(2.0, super::normalize_range(22.0, 1.0..21.0))
    }

    #[test]
    fn wraps_around_beginning() {
        assert_eq!(20.0, super::normalize_range(0.0, 1.0..21.0))
    }
}
