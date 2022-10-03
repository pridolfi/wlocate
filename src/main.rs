use std::{process::Command};
use std::str;

use nalgebra::{distance, Point3, DMatrix};

fn signal_dbm_to_distance_m(dbm: f64, freq_mhz: f64) -> f64 {
    let fspl = 27.55f64; // Free-Space Path Loss adapted avarage constant for home WiFI routers and following units
    10f64.powf((fspl - 20f64 * freq_mhz.log10() + dbm.abs()) / 20f64)
}

#[test]
fn test_distance_m_from_dbm() {
    assert_eq!(signal_dbm_to_distance_m(-64., 2417.), 15.639517472147746);
    assert_eq!(signal_dbm_to_distance_m(-40., 5660.), 0.42138936315637915);
}

fn field_from_str<'a>(text: &'a str, field_str: &str, end_str: &str) -> Option<&'a str> {
    let field_len = field_str.len();
    if let Some(i) = text.find(field_str) {
        if let Some(j) = &text[i + field_len..].find(end_str) {
            return Some(&text[i + field_len..i + field_len + j]);
        }
    }
    None
}

#[test]
fn test_field_from_str() {
    assert_eq!(
        field_from_str("bla sarasa\nkey: value \npepe = paco;", "key: ", " \n"),
        Some("value")
    );
    assert_eq!(
        field_from_str("bla sarasa\nkey: value \npepe = paco;", "pepe = ", ";"),
        Some("paco")
    );
}

fn signal_dbm_from_networks_scan() {
    println!("Scanning...");

    let command = Command::new("bash")
        .args(["-c", "sudo iwlist scan"])
        .output()
        .expect("failed to execute process");

    let output = str::from_utf8(&command.stdout).expect("failed to parse stdout");

    output.split("          Cell ").for_each(|cell| {
        println!("***** ");
        let mut f_mhz: Option<f64> = None;
        let mut s_dbm: Option<f64> = None;
        // println!("{}", cell);
        if let Some(essid) = field_from_str(cell, "ESSID:\"", "\"") {
            println!("ESSID: {}", essid);
        }
        if let Some(bssid) = field_from_str(cell, "Address: ", "\n") {
            println!("BSSID: {}", bssid);
        }
        if let Some(freq) = field_from_str(cell, "Frequency:", " ") {
            f_mhz = Some(freq.parse::<f64>().unwrap() * 1000.);
            println!("Freq:  {}", f_mhz.unwrap());
        }
        if let Some(signal) = field_from_str(cell, "Signal level=", " ") {
            s_dbm = Some(signal.parse::<f64>().unwrap());
            println!("dBm:   {}", s_dbm.unwrap());
        }
        if let (Some(f), Some(s)) = (f_mhz, s_dbm) {
            println!("dist:  {}", signal_dbm_to_distance_m(s, f));
        }
    });
}

fn trilaterate(references: &[Point3<f64>], distances: &[f64]) -> Point3<f64> {
    let n = references.len()-1;
    assert_eq!(references.len(), distances.len());
    let mut dist2ref0 = vec![0.; n];
    for (i, v) in dist2ref0.iter_mut().enumerate() {
        *v = distance(&references[i+1], &references[0]);
    }
    let mut a = DMatrix::<f64>::zeros(n, 3);
    for (i, mut row) in a.row_iter_mut().enumerate() {
        row[0] = references[i+1].x - references[0].x;
        row[1] = references[i+1].y - references[0].y;
        row[2] = references[i+1].z - references[0].z;
    }
    let mut b = DMatrix::<f64>::zeros(n, 1);
    for (i, mut row) in b.row_iter_mut().enumerate() {
        row[0] = (distances[0].powi(2) - distances[i+1].powi(2) + dist2ref0[i].powi(2))/2.;
    }
    let a_t = a.transpose();
    let x = (a_t.clone() * a).try_inverse().unwrap() * a_t * b;
    Point3::new(x[0]+references[0].x, x[1]+references[0].y, x[2]+references[0].z)
}

#[test]
fn test_trilaterate() {
    let p = Point3::new(-1.,1.3,3.);
    let references: Vec<Point3<f64>> = Vec::from([
        Point3::new(0.,0.,0.),
        Point3::new(3.,0.,0.),
        Point3::new(0.,3.,0.),
        Point3::new(3.,3.,0.),
        Point3::new(0.,0.,3.),
        Point3::new(3.,0.,3.),
        Point3::new(0.,3.,3.),
        Point3::new(3.,3.,3.)
    ]);
    let mut distances = vec![0.; references.len()];
    for (i, d) in distances.iter_mut().enumerate() {
        *d = distance(&references[i], &p);
    }
    let p_new = trilaterate(&references, &distances);
    assert_eq!(p.x, (p_new.x * 10000.).round() / 10000.);
    assert_eq!(p.y, (p_new.y * 10000.).round() / 10000.);
    assert_eq!(p.z, (p_new.z * 10000.).round() / 10000.);
}

fn main() {
    signal_dbm_from_networks_scan();
}
