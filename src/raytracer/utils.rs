
use rand::prelude::*;
use rand_distr::StandardNormal;
use na::Vector3;

pub fn uniform_sampling_hemisphere() -> Vector3<f64> {
    // generate random vectors uniformly on "normalized" hemisphere: 
    // - centered on origin
    // - norm(x, y, z) = 1
    // - x, y, z >= 0
    // multivariate standard normal distribution is spherically symmetric
    // so we get uniform distribution if we sample x y z with standard normal distribution
    // and normalize the generated vector
    let x: f64 = thread_rng().sample(StandardNormal);
    let z: f64 = thread_rng().sample(StandardNormal);

    // we flip y if negative to get hemisphere
    let rand_value: f64 = thread_rng().sample(StandardNormal);
    let y = if rand_value < 0.0 { -rand_value } else { rand_value };

    let length = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    Vector3::<f64>::new(x / length, y / length, z / length)
}

pub fn create_coordinate_system_from_up_vector(up_vector: &Vector3<f64>) -> [Vector3<f64>; 3] {
    let nt = if up_vector.x.abs() > up_vector.y.abs() {
        let nt_length = (up_vector.x.powi(2) + up_vector.z.powi(2)).sqrt();
        Vector3::<f64>::new(up_vector.z / nt_length, 0.0, -up_vector.x / nt_length) 
    } else {
        let nt_length = (up_vector.y.powi(2) + up_vector.z.powi(2)).sqrt();
        Vector3::<f64>::new(0.0,-up_vector.z / nt_length,  up_vector.y / nt_length)
    };

    let nb = up_vector.cross(&nt);
    [nt, up_vector.clone(), nb]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_uniform_sampling_hemisphere_returns_unit_vector() {
        for _ in 0..100 {
            let v = uniform_sampling_hemisphere();
            assert!((v.norm() - 1.0).abs() < EPSILON, "Vector should be normalized");
        }
    }

    #[test]
    fn test_uniform_sampling_hemisphere_y_is_non_negative() {
        for _ in 0..100 {
            let v = uniform_sampling_hemisphere();
            assert!(v.y >= 0.0, "Y component should be non-negative for hemisphere");
        }
    }

    #[test]
    fn test_coordinate_system_vectors_are_orthogonal() {
        let up = Vector3::new(0.0, 1.0, 0.0);
        let [nt, n, nb] = create_coordinate_system_from_up_vector(&up);
        
        assert!(nt.dot(&n).abs() < EPSILON, "nt and n should be orthogonal");
        assert!(nt.dot(&nb).abs() < EPSILON, "nt and nb should be orthogonal");
        assert!(n.dot(&nb).abs() < EPSILON, "n and nb should be orthogonal");
    }

    #[test]
    fn test_coordinate_system_vectors_are_normalized() {
        let up = Vector3::new(0.0, 1.0, 0.0).normalize();
        let [nt, _n, nb] = create_coordinate_system_from_up_vector(&up);
        
        assert!((nt.norm() - 1.0).abs() < EPSILON, "nt should be normalized");
        assert!((nb.norm() - 1.0).abs() < EPSILON, "nb should be normalized");
    }

    #[test]
    fn test_coordinate_system_with_z_up() {
        let up = Vector3::new(0.0, 0.0, 1.0).normalize();
        let [nt, n, nb] = create_coordinate_system_from_up_vector(&up);
        
        // Check orthogonality
        assert!(nt.dot(&n).abs() < EPSILON);
        assert!(nt.dot(&nb).abs() < EPSILON);
        assert!(n.dot(&nb).abs() < EPSILON);
    }

    #[test]
    fn test_coordinate_system_with_arbitrary_up() {
        let up = Vector3::new(1.0, 1.0, 1.0).normalize();
        let [nt, n, nb] = create_coordinate_system_from_up_vector(&up);
        
        // Check orthogonality
        assert!(nt.dot(&n).abs() < EPSILON);
        assert!(nt.dot(&nb).abs() < EPSILON);
        assert!(n.dot(&nb).abs() < EPSILON);
    }

    #[test]
    fn test_coordinate_system_preserves_up_vector() {
        let up = Vector3::new(0.5, 0.5, 0.707).normalize();
        let [_nt, n, _nb] = create_coordinate_system_from_up_vector(&up);
        
        assert!((n - up).norm() < EPSILON, "Middle vector should equal input up vector");
    }
}