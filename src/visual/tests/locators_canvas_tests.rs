mod test_helpers;

#[cfg(test)]
mod tests {
    use serde_json::json;
    use visual::locators_canvas::LocatorsCanvas;
    use visual::locators_canvas::locators_trie_node::LocatorTrieNode;
    use crate::test_helpers::{get_test_locators, KeyQueueLengths};

    #[test]
    fn create_canvas_canvas_valid() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let canvas = LocatorsCanvas::new(locator_root, None);
        
        assert_eq!(canvas.location_key, None);
        assert_eq!(canvas.locations_paths.clone().unwrap().len(), KeyQueueLengths::DOUBLE_CHAR);
        assert_eq!(canvas.root.children.unwrap().len(), KeyQueueLengths::SINGLE_CHAR);
        assert!(canvas.locations_paths.unwrap().into_iter().all(|path| path.1.len() == 2 ));
    }

    #[test]
    fn create_canvas_not_full_double_char_canvas_valid_with_double_char() {
        let locator_root = LocatorTrieNode::new(get_test_locators(50));
        let canvas = LocatorsCanvas::new(locator_root, Some(String::from("t")));

        println!("Canvas: {}", json!(canvas));
        
        assert_eq!(canvas.location_key, Some("t".to_string()));
        assert_eq!(canvas.locations_paths.clone().unwrap().len(), KeyQueueLengths::SINGLE_CHAR);
        assert!(canvas.locations_paths.unwrap().into_iter().all(|path| path.1.len() == 2 ));
    }

    #[test]
    fn create_canvas_for_key_contains_key() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let canvas = LocatorsCanvas::new(locator_root, Some(String::from("f")));

        assert_eq!(canvas.location_key, Some(String::from("f")));
        assert_eq!(canvas.clone().locations_paths.unwrap().len(), KeyQueueLengths::SINGLE_CHAR);
        assert!(canvas
            .clone()
            .locations_paths
            .unwrap()
            .into_iter()
            .all(|path| path.1.contains(&String::from("f"))));
    }
    
    #[test]
    fn create_concrete_canvas_for_key_contains_key() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let canvas = LocatorsCanvas::new(locator_root, Some(String::from("ff")));

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
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let mut canvas = LocatorsCanvas::new(locator_root, None);
        
        canvas.update(Some(String::from("f")));

        assert_eq!(canvas.location_key, Some(String::from("f")));
        assert_eq!(canvas.clone().locations_paths.unwrap().len(), KeyQueueLengths::SINGLE_CHAR);
        assert!(canvas
            .clone()
            .locations_paths
            .unwrap()
            .into_iter()
            .all(|path| path.1.contains(&String::from("f"))));
    }
    
    #[test]
    fn update_should_filter_out_without_key(){
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let mut canvas = LocatorsCanvas::new(locator_root, None);
        
        canvas.update(None);

        assert_eq!(canvas.location_key, None);
        assert_eq!(canvas.clone().locations_paths.unwrap().len(), KeyQueueLengths::DOUBLE_CHAR);
    }

    #[test]
    fn update_should_reset_updated(){
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let mut canvas = LocatorsCanvas::new(locator_root, None);

        canvas.update(Some(String::from("f")));
        canvas.update(None);

        assert_eq!(canvas.location_key, None);
        assert_eq!(canvas.clone().locations_paths.unwrap().len(), KeyQueueLengths::DOUBLE_CHAR);
    }

    #[test]
    fn update_should_set_new_key_value(){
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let mut canvas = LocatorsCanvas::new(locator_root, None);

        canvas.update(Some(String::from("fa")));

        assert_eq!(canvas.location_key, Some(String::from("fa")));
    }

    #[test]
    fn update_should_set_new_key_none(){
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let mut canvas = LocatorsCanvas::new(locator_root, Some(String::from("ff")));

        canvas.update(None);

        assert_eq!(canvas.location_key, None);
    }
}
