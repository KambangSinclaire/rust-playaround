#[cfg(test)]
mod test {
    // use super::*;
    // Enums - Group related values under a single type
    #[derive(Debug)]
    #[allow(dead_code)]
    enum CarColour {
        Red,
        Green,
        Blue,
        Silver,
    }
    
    fn create_car_color() -> CarColour {
        let my_car_color: CarColour = CarColour::Blue;
        my_car_color
    }
    
    #[derive(Debug)]
    enum GivenResult<T, E> {
        Ok(T),
        Err(E),
    }
    
    #[derive(Debug)]
    enum GivenOption<T> {
        None,
        Some(T),
    }
    
    fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
        if num_check < 5 {
            GivenResult::Ok(num_check)
        } else {
            GivenResult::Err("Not under 5".to_string())
        }
    }
    
    fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
        if num_check < 5 {
            Ok(num_check)
        } else {
            Err("Not under 5".to_string())
        }
    }
    
    fn remainder_zero(num_check: f32) -> GivenOption<f32> {
        let remainder: f32 = num_check % 10.0;
        if remainder != 0.0 {
            GivenOption::Some(num_check)
        } else {
            GivenOption::None
        }
    }
    
    fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
        let remainder: f32 = num_check % 10.0;
        if remainder != 0.0 {
            Some(num_check)
        } else {
            None
        }
    }

    #[test]
    fn tests_enums() {
        let car_color = create_car_color();
        dbg!(car_color);

        let under5_result = check_under_five(2);
        dbg!(under5_result);

        let under5_result = check_under_five(7);
        dbg!(under5_result);

        let remainder = remainder_zero(23.3);
        dbg!(remainder);

        let remainder = remainder_zero(10.0);
        dbg!(remainder);

        // Testing built-in enums
        let under5_result_built_in = check_under_five_built_in(7);
        dbg!(under5_result_built_in);

        let remainder_built_in = remainder_zero_built_in(10.0);
        dbg!(remainder_built_in);
    }
}
