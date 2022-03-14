mod depth;

use std::fs::File;
use std::io;
use std::io::BufRead;

use crate::utils::depth::Depth;
use anyhow::bail;

pub fn depth_from_vec(depths: Vec<i32>) -> usize {
    depths[1..]
        .iter()
        .enumerate()
        .filter(|(i, _)| depths[*i] < depths[i + 1])
        .count()
}

pub fn depth_from_file(file_name: &str) -> anyhow::Result<usize> {
    let mut depth = Depth {
        previous_depth: 0,
        current_depth: 0,
        depth_increase_count: 0,
        first: true,
    };
    let file = File::open(file_name)?;
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                let new_depth: i32 = line[..line.len() - 1].parse()?;
                depth = depth.evaluate(new_depth);

                line.clear();
            }
            Err(_) => bail!("Error reading file"),
        }
    }
    Ok(depth.depth_increase_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1_part_1_depth_from_vec() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, depth_from_vec(depths))
    }

    #[test]
    fn day_1_part_1_depth_from_sample_file() {
        assert_eq!(7, depth_from_file("./sonar_sweep_sample.txt").unwrap());
    }

    #[test]
    fn day_1_part_1_depth_from_file() {
        assert_eq!(1121, depth_from_file("./sonar_sweep.txt").unwrap());
    }
}
