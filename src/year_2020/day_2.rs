pub mod day_2 {
    use std::cmp;
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use std::path::Path;

    pub struct Dimension {
        length: i32,
        width: i32,
        height: i32,
    }
    impl Dimension {
        pub fn new(length: i32, width: i32, height: i32) -> Dimension {
           Dimension {
               length, width, height
           }
        }
    }

    pub fn elves_total_wrapping_paper() {
        let path = Path::new("src/year_2020/test_files/day_2_test_input.txt");
        let mut total = 0;
        let file = File::open(path)
            .expect("Missing file");
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(l) = line {
                // Split String by x
                let box_dimensions: Vec<i32> = l.split('x').map(|c|
                    c.parse::<i32>().unwrap()
                ).collect();
                let l = box_dimensions.get(0).expect("Missing length");
                let w = box_dimensions.get(1).expect("Missing width");
                let h = box_dimensions.get(2).expect("Missing height");
                total = total + get_total_square_feet_needed(Dimension::new(*l, *w, *h))
            }
        }
        println!("Total Needed: {}", total)
    }

    pub fn get_total_square_feet_needed(dimension: Dimension) -> i32 {
        let lw_area = dimension.length * dimension.width;
        let lh_area = dimension.length * dimension.height;
        let wh_area = dimension.width * dimension.height;

        let min_area = cmp::min(lw_area, cmp::min(lh_area, wh_area));
        return ((lw_area + lh_area + wh_area) * 2) + min_area;
    }
}

#[cfg(test)]
mod test {
    use crate::year_2020::day_2::day_2::{Dimension, get_total_square_feet_needed};

    #[test]
    fn test_get_total_square_feet_needed_test_1() {
        let test_input = Dimension::new(2,3,4);
        assert_eq!(get_total_square_feet_needed(test_input), 58)
    }
}