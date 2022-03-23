
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