use toolbox::ds::matrix::Matrix;
use std::fmt::Debug;

#[test]
fn matrix_integration_test() {
    
    let mut m = Matrix::<i32>::new(3, 5);
    m.fill(5);
    print_matrix::<i32>(&m);

    let x = m.clone();
    m.fill(3);
    print_matrix(&x);
    print_matrix(&m);

    assert_eq_all(&m, 3);
    assert_eq_all(&x, 5);
}

fn print_matrix<T: Default + Clone + Debug>(m: &Matrix<T>) {
    for i in 0..m.rows() {
        for j in 0..m.cols() {
            print!("{:?} ", m[i][j]);
        }
        println!();
    }
    println!();
}

fn assert_eq_all<T: Default + Clone + PartialEq + Debug>(m: &Matrix<T>, value: T) {
    for i in 0..m.rows() {
        for j in 0..m.cols() {
            assert_eq!(m.get(i, j), &value);
        }
    }
}