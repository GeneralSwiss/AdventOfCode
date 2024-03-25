trait FromCalibrationData {
    fn from_data(&self) -> Result<i32, Box<dyn std::error::Error + 'static>>;
}

impl FromCalibrationData for &str {
    fn from_data(&self) -> Result<i32, Box<dyn std::error::Error + 'static>> {
        let mut digits = self.chars()
            .filter_map(|char| char.to_digit(10));
        let first = digits.next().unwrap();
        let last = digits.next_back().unwrap_or(first);
        Ok((first * 10 + last).try_into().expect("never negative"))
    }
}

mod test {
    use super::*;
    #[test]
    fn read_file_from_disk() {
        let x = include_str!("./input.txt");
        assert_eq!(x.trim().split("\n").count(), 1000);
    }

    #[test]
    fn parse_numbers_from_str() {
        let sample = "asf3asdga4asfa";
        assert_eq!(sample.from_data().unwrap(), 34i32);
    }

    #[test]
    fn main_day_one_challenge() {
        let input = include_str!("./input.txt");
        println!("Number: {:?}", input.lines()
                 .map(|l| l.from_data())
                 .collect::<Result<Vec<_>,_>>()
                 .map(|r| r.into_iter().sum::<i32>()))
    }
}
