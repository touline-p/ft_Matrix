use matrix::{vector::Vector, matrix::Matrix};

fn main() {
    let vec: Vector<f32, 4> = [1., 1., 1., 1.].into();
    println!("first point homogeneous is :{vec}");

    let matrix: Matrix<f32, 4, 4> = [
        [1., 0., 0., 0.].into(),
        [0., 1., 0., 0.].into(),
        [0., 0., 1., 0.].into(),
        [0., 0., 0., 1.].into(),
    ].into();

    let translated = vec.clone() * matrix;
    println!("translated by unity matrix : {translated}");
    /*
    let translation: Matrix<f32, 4, 4> = [
        [cos(scale), 0., sin(scale), 1.].into(),
        [0.,         1., 0.,         1.].into(),
        [sin(scale), 0., cos(scale), 1.].into(),
        [0.,         0., 0.,         1.].into(),
    ].into();
    */

    //ration == w / h
    //le rasterizer va toujours de - 1 / -1 a 1 / 1
    let ratio = 1920.0 / 1080.;
    let near = 1.0;
    let far = 10.0;
    let range = near - far;


    let fov = 90. * std::f32::consts::PI / 180.0;
    let half_fov = fov / 2.;
    let tan_half_fov = half_fov.tan();
    let d = 1. / tan_half_fov;

    let a = (-far - near) / range;
    let b = 2. * far * near / range;
    let projection: Matrix<f32, 4, 4> = [
        [d/ratio, 0., 0., 0.].into(),
        [0.,      d,  0., 0.].into(),
        [0.,      0., a , b ].into(),
        [0.,      0., 1., 0.].into(),
    ].into();
    let translated = vec.clone() * projection.clone();
    println!("translated by projection matrix {projection:?} : {translated}");
}
