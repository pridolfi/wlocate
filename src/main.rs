use std::process::Command;
use std::str;

use nalgebra::{Matrix3, distance, Point3};

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
        if let Some(j) = &text[i+field_len..].find(end_str) {
            return Some(&text[i+field_len..i+field_len+j])
        }
    }
    None
}

#[test]
fn test_field_from_str() {
    assert_eq!(field_from_str("bla sarasa\nkey: value \npepe = paco;", "key: ", " \n"), Some("value"));
    assert_eq!(field_from_str("bla sarasa\nkey: value \npepe = paco;", "pepe = ", ";"), Some("paco"));
}

fn signal_dbm_from_networks_scan() {
    println!("Scanning...");

    let command = Command::new("bash")
        .args(["-c", "sudo iwlist wlp1s0 scan"])
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

#[test]
fn test_distance() {
    let p1 = Point3::new(7., 4., 3.);
    let p2 = Point3::new(17., 6., 2.);
    assert_eq!(distance(&p1, &p2), 10.246950765959598);
}

fn play_with_nalgebra() {
    let m = Matrix3::new(
        1., -2., 3.,
       -4.,  5., 6.,
        7.,  8., 9.);    
   println!("{}", m);
   println!("{}", m.transpose());
   println!("{}", m.try_inverse().unwrap());
   let p1 = Point3::new(7., 4., 3.);
   let p2 = Point3::new(17., 6., 2.);
   println!("{}", distance(&p1, &p2));
}

fn main() {
    signal_dbm_from_networks_scan();
    play_with_nalgebra();
}
