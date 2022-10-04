use nalgebra::{distance, DMatrix, Point3};

pub fn trilaterate(references: &[Point3<f64>], distances: &[f64]) -> Option<Point3<f64>> {
    let n = references.len() - 1;
    assert_eq!(references.len(), distances.len());
    let mut dist2ref0 = vec![0.; n];
    for (i, v) in dist2ref0.iter_mut().enumerate() {
        *v = distance(&references[i + 1], &references[0]);
    }
    let mut a = DMatrix::<f64>::zeros(n, 3);
    for (i, mut row) in a.row_iter_mut().enumerate() {
        row[0] = references[i + 1].x - references[0].x;
        row[1] = references[i + 1].y - references[0].y;
        row[2] = references[i + 1].z - references[0].z;
    }
    let mut b = DMatrix::<f64>::zeros(n, 1);
    for (i, mut row) in b.row_iter_mut().enumerate() {
        row[0] = (distances[0].powi(2) - distances[i + 1].powi(2) + dist2ref0[i].powi(2)) / 2.;
    }
    let a_t = a.transpose();
    if let Some(inv) = (&a_t * a).try_inverse() {
        let x = inv * a_t * b;
        Some(Point3::new(
            x[0] + references[0].x,
            x[1] + references[0].y,
            x[2] + references[0].z,
        ))
    } else {
        None
    }
}

#[test]
fn test_trilaterate() {
    let p = Point3::new(-1., 1.3, 3.);
    let references: Vec<Point3<f64>> = Vec::from([
        Point3::new(0., 0., 0.),
        Point3::new(3., 0., 0.),
        Point3::new(0., 3., 0.),
        Point3::new(3., 3., 0.),
        Point3::new(0., 0., 3.),
        Point3::new(3., 0., 3.),
        Point3::new(0., 3., 3.),
        Point3::new(3., 3., 3.),
    ]);
    let mut distances = vec![0.; references.len()];
    for (i, d) in distances.iter_mut().enumerate() {
        *d = distance(&references[i], &p);
    }
    if let Some(p_new) = trilaterate(&references, &distances) {
        println!("p        {}", p);
        println!("p_trilat {}", p_new);
        assert_eq!(p.x, (p_new.x * 10000.).round() / 10000.);
        assert_eq!(p.y, (p_new.y * 10000.).round() / 10000.);
        assert_eq!(p.z, (p_new.z * 10000.).round() / 10000.);
    } else {
        println!("Trilateration failed.");
        assert!(false);
    }
}
