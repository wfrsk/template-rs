pub fn greet(who: &str) -> String {
    format!("Hello, {who}!")
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn should_greet_whoever() {
        assert_eq!(greet("Amanda"), "Hello, Amanda!");
    }
}
