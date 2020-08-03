#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(true != true, false);
        assert_eq!(String::from("Hello").contains("o"), true);
    }
}
