#[cfg(test)]
mod tests {
    use crate::windows::locator::actions::locator_finder::get_root_locators;

    #[test]
    #[ignore = "Only for local use"]
    fn test_add() {
        println!("Start locator");
        let locators = get_root_locators().ok().unwrap();
        assert_ne!(locators.len(), 0);
    }
}
