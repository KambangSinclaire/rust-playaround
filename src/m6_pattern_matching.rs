#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pattern_matching_literals_test() {
        let number = 45;
        let tests = match number {
            20 => 1,
            30..=50 => 4,
            70 | 90 | 60 => 2,
            _ => 0,
        };

        let test_enum: Option<i32> = Some(45);
        let result = match test_enum {
            Some(data) => data,
            _ => panic!("Failed here"),
        };

        if let Some(data) = test_enum {
            println!("The data is {}", data);
        } else {
            println!("No data found")
        }

        let my_data = if let Some(data) = test_enum { data } else { 1 };
    }
}
