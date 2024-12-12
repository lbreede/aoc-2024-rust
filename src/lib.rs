use std::io::BufRead;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

// Additional common functions

#[derive(Debug)]
pub struct Grid {
    pub data: Vec<Vec<u8>>,
}

impl<R: BufRead> From<R> for Grid {
    fn from(reader: R) -> Self {
        Grid {
            data: reader
                .lines()
                .map_while(Result::ok)
                .map(|line| line.bytes().collect::<Vec<u8>>())
                .collect(),
        }
    }
}

impl Grid {
    pub fn height(&self) -> usize {
        self.data.len()
    }
    pub fn width(&self) -> usize {
        self.data[0].len()
    }
    pub fn get_value(&self, x: i32, y: i32) -> Option<u8> {
        if x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32 {
            Some(self.data[y as usize][x as usize])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
