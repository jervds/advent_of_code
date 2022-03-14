use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn depth_from_vec(depths: Vec<i32>) -> usize {
    depths[1..]
        .iter()
        .enumerate()
        .filter(|(i, _)| depths[*i] < depths[i + 1])
        .count()
}

pub fn depth_from_file(file_name: &str) -> usize {
    let mut response = 0;
    if let Ok(lines) = read_lines(file_name) {
        let mut first = true;
        let mut previous_depth = 0;
        for line in lines {
            if let Ok(depth) = line {
                let current_depth: i32 = depth.parse().unwrap();
                if first {
                    first = false;
                } else {
                    if current_depth > previous_depth {
                        response += 1;
                    }
                }
                previous_depth = current_depth
            }
        }
    }
    response
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
    fn day_1_part_1_depth_from_file() {
        assert_eq!(1121, depth_from_file("./sonar_sweep.txt"))
    }
}
