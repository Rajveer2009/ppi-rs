use colored::*;
use std::env::args;

fn main() {
    let width = args().nth(1).expect("Error");
    let height = args().nth(2).expect("Error");
    let diagonal = args().nth(3).expect("Error");

    let ppi = ppi(
        width.parse::<f32>().unwrap(),
        height.parse::<f32>().unwrap(),
        diagonal.parse::<f32>().unwrap(),
    );
    // "{}X{}@{}.0in -> {} ppi",
    println!(
        "{}X{}@{} -> {}",
        width.to_string().bright_yellow(),
        height.to_string().bright_yellow(),
        format!("{}.0in", diagonal.to_string()).bright_green(),
        format!("{} ppi", ppi.to_string()).bright_red()
    );
}

fn ppi(width: f32, height: f32, diagonal: f32) -> f32 {
    let dip = ((width * width) + (height * height)).sqrt();
    let ppi = (dip / diagonal).ceil();
    return ppi;
}
