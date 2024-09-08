use super::Matrix;

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32, 4, 4> {
    let range = near - far;
    let fov = fov * std::f32::consts::PI / 180.0;
    let half_fov = fov / 2.;
    let tan_half_fov = half_fov.tan();
    let d = 1. / tan_half_fov;
    let a = (-far - near) / range;
    let b = 2. * far * near / range;
    [
        [d/ratio, 0., 0., 0.].into(),
        [0.     , d,  0., 0.].into(),
        [0.     , 0., a , b ].into(),
        [0.     , 0., 1., 0.].into(),
    ].into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_projection() {
        let mut projection_matrix = projection(45., 16. / 9., 2., 50.);
        projection_matrix.transpose();
        println!("{projection_matrix:?}");
    }

}
