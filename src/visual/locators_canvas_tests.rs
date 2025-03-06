#[cfg(test)]
mod tests {
    use crate::visual::locators_canvas::LocatorCanvas;
    use crate::visual::test_helpers::get_test_locators;
    use crate::visual::locators_trie_node::LocatorTrieNode;

    #[test]
    fn create_canvas_canvas_valid() {
        let locator_root = LocatorTrieNode::new(get_test_locators(196));
        let canvas = LocatorCanvas::new(locator_root, None);
        
        assert_eq!(canvas.location_key, None);
        assert_eq!(canvas.locations_paths.unwrap().len(), 196);
        assert_eq!(canvas.root.children.unwrap().len(), 14);
    }

    #[test]
    fn create_canvas_for_key_contains_key() {
        let locator_root = LocatorTrieNode::new(get_test_locators(196));
        let canvas = LocatorCanvas::new(locator_root, Some(String::from("f")));

        assert_eq!(canvas.location_key, Some(String::from("f")));
        assert_eq!(canvas.clone().locations_paths.unwrap().len(), 14);
        assert!(canvas
            .clone()
            .locations_paths
            .unwrap()
            .into_iter()
            .all(|path| path.1.contains(&String::from("f"))));
    }
    
    #[test]
    fn create_concrete_canvas_for_key_contains_key() {
        let locator_root = LocatorTrieNode::new(get_test_locators(196));
        let canvas = LocatorCanvas::new(locator_root, Some(String::from("ff")));

        assert_eq!(canvas.location_key, Some(String::from("ff")));
        assert_eq!(canvas.clone().locations_paths.unwrap().len(), 1);
        assert!(canvas
            .clone()
            .locations_paths
            .unwrap()
            .into_iter()
            .all(|path| path.1.contains(&String::from("ff"))));
    }

    #[test]
    fn update_should_filter_out_children() {
        let locator_root = LocatorTrieNode::new(get_test_locators(196));
        let mut canvas = LocatorCanvas::new(locator_root, None);
        
        canvas.update(Some(String::from("f")));

        assert_eq!(canvas.location_key, Some(String::from("f")));
        assert_eq!(canvas.clone().locations_paths.unwrap().len(), 14);
        assert!(canvas
            .clone()
            .locations_paths
            .unwrap()
            .into_iter()
            .all(|path| path.1.contains(&String::from("f"))));
    }
}
