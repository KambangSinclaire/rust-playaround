#[cfg(test)]
mod tests {

    macro_rules! advance_macro {
        ($x:ty) => {
            match stringify!($x) {
                "i32" => format!("hello"),
                _ => format!("Tested"),
            }
        };
    }

    #[test]
    fn test_macro() {
        let x = advance_macro!(i32);
        dbg!(x);
    }
}
