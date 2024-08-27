use nalgebra::{Matrix1x4, Matrix4, Matrix4x2, Matrix4x3};

type Mat4 = Matrix4<f32>;
type Mat1x4 = Matrix1x4<f32>;
type Mat4x2 = Matrix4x2<f32>;
type Mat4x3 = Matrix4x3<f32>;

fn main() {

    // Questão 1

    let g_matrix: Mat4x2 = Mat4x2::new(
        10.0, 20.0, 
        30.0, 5.0, 
        -5.0, 8.0, 
        7.0, -3.0,
    );
    let c_matrix: Mat4x2 = hermite_matrix() * g_matrix;

    println!("Questão 1");

    print!("{}", t_matrix(0.2) * c_matrix);
    print!("{}", t_matrix(0.35) * c_matrix);
    print!("{}", t_matrix(0.5) * c_matrix);
    print!("{}", t_matrix(0.75) * c_matrix);
    print!("{}", t_matrix(0.9) * c_matrix);

    // Questão 2
    
    let g_matrix: Mat4x2 = Mat4x2::new(
        13.0, 19.0, 
        2.0, -5.0, 
        -4.0, 20.0, 
        32.0, 11.0,
    );

    println!("Questão 2 Bezier");

    let c_matrix: Mat4x2 = bezier_matrix() * g_matrix;

    print!("{}", t_matrix(0.1) * c_matrix);
    print!("{}", t_matrix(0.3) * c_matrix);
    print!("{}", t_matrix(0.55) * c_matrix);
    print!("{}", t_matrix(0.7) * c_matrix);
    print!("{}", t_matrix(0.85) * c_matrix);
    print!("{}", t_matrix(1.0) * c_matrix);

    println!("Questão 2 B-Spline");

    // Questão 3

    println!("Questão 3");
}

fn t_matrix(t: f32) -> Mat1x4 {
    Mat1x4::new(t.powi(3), t.powi(2), t, 1.0)
}

fn hermite_matrix() -> Mat4 {
    Mat4::new(
        2.0, -2.0, 1.0, 1.0,
        -3.0, 3.0, -2.0, -1.0,
        0.0, 0.0, 1.0, 0.0,
        1.0, 0.0, 0.0, 0.0
    )
}

fn bezier_matrix() -> Mat4 {
    Mat4::new(
        -1.0, 3.0, -3.0, 1.0,
        3.0, -6.0, 3.0, 0.0,
        -3.0, 3.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0
    )
}

fn b_spline_matrix() -> Mat4 {
    Mat4::new(
        -1.0, 3.0, -3.0, 1.0,
        3.0, -6.0, 3.0, 0.0,
        -3.0, 0.0, 3.0, 0.0,
        1.0, 4.0, 1.0, 0.0
    )
}