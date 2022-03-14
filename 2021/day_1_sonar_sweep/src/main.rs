mod utils;

use crate::utils::{depth_from_file, depth_from_vec};

fn main() {
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    println!("Sample response {}", depth_from_vec(depths));
    println!("Response: {}", depth_from_file("./sonar_sweep.txt"));
}
