
pub mod trilaterate;
pub mod measure;

use crate::measure::signal_dbm_from_networks_scan;
use crate::trilaterate::trilaterate;
use nalgebra::Point3;

fn main() {
    signal_dbm_from_networks_scan();
    println!("trilateration result: {:?}", trilaterate(&Vec::from([
        Point3::new(0., 0., 0.),
        Point3::new(3., 0., 0.),
        Point3::new(0., 3., 0.),
        Point3::new(3., 3., 0.),
        Point3::new(0., 0., 3.),
        Point3::new(3., 0., 3.),
        Point3::new(0., 3., 3.),
        Point3::new(3., 3., 3.),
    ]), &vec![1.5f64; 8]));
}
