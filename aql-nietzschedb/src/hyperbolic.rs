//! Hyperbolic geometry utilities for DESCEND/ASCEND/ORBIT.
//! Native Poincaré ball operations.

/// Calculate magnitude (distance from origin) in Poincaré ball.
pub fn magnitude(coords: &[f32]) -> f32 {
    coords.iter().map(|x| x * x).sum::<f32>().sqrt()
}

/// Check if coordinates are within the Poincaré ball (magnitude < 1.0).
pub fn is_valid_poincare(coords: &[f32]) -> bool {
    magnitude(coords) < 1.0
}

/// Poincaré distance between two points.
pub fn poincare_distance(u: &[f32], v: &[f32]) -> f32 {
    assert_eq!(u.len(), v.len(), "dimension mismatch");

    let norm_u_sq: f32 = u.iter().map(|x| x * x).sum();
    let norm_v_sq: f32 = v.iter().map(|x| x * x).sum();
    let diff_sq: f32 = u.iter().zip(v.iter()).map(|(a, b)| (a - b).powi(2)).sum();

    let numerator = 2.0 * diff_sq;
    let denominator = (1.0 - norm_u_sq) * (1.0 - norm_v_sq);

    if denominator <= 0.0 {
        return f32::INFINITY;
    }

    (1.0 + numerator / denominator).acosh()
}

/// Filter nodes that are deeper (higher magnitude) than source.
pub fn filter_descendants(source_mag: f32, candidate_mag: f32, max_depth_delta: f32) -> bool {
    candidate_mag > source_mag && candidate_mag <= source_mag + max_depth_delta
}

/// Filter nodes that are shallower (lower magnitude) than source.
pub fn filter_ancestors(source_mag: f32, candidate_mag: f32) -> bool {
    candidate_mag < source_mag
}

/// Filter nodes at similar depth (magnitude within radius).
pub fn filter_orbit(source_mag: f32, candidate_mag: f32, radius: f32) -> bool {
    (candidate_mag - source_mag).abs() < radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        assert!((magnitude(&[0.3, 0.4]) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_poincare_valid() {
        assert!(is_valid_poincare(&[0.3, 0.4]));
        assert!(!is_valid_poincare(&[0.8, 0.8]));
    }

    #[test]
    fn test_orbit_filter() {
        assert!(filter_orbit(0.5, 0.52, 0.1));
        assert!(!filter_orbit(0.5, 0.8, 0.1));
    }
}
