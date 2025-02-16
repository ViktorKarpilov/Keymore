#[cfg(test)]
mod tests {
    use crate::locator::actions::locator_finder::get_root_locators;

    #[test]
    fn test_add() {
        println!("Start locator");
        let locators = get_root_locators(Some(true)).ok().unwrap();
        assert_ne!(locators.len(), 0);
    }
}
