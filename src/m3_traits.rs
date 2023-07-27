trait Scholar {
    fn choose_style(&self) -> String;
    fn choose_strategy(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Pastor,
    Teacher,
}

impl Scholar for Character {
    fn choose_strategy(&self) -> String {
        match self {
            Self::Teacher => String::from("Teach"),
            Self::Pastor => "Pray".to_string(),
            Self::Warrior => String::from("Fight"),
        }
    }
    fn choose_style(&self) -> String {
        match self {
            Character::Pastor => "Shout".to_string(),
            Character::Teacher => "Quite".to_string(),
            _ => String::from("Nothing to do"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests_traits() {

        let my_character = Character::Pastor;
        let style = my_character.choose_style();
        dbg!(style);
    }
}
