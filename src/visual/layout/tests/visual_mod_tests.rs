#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use crate::windows::locator::locator::Locator;
    use crate::visual::layout::locators::locators_trie_node::LocatorTrieNode;
    use crate::visual::layout::tests::test_helpers::{get_test_locators, KeyQueueLengths};
    use crate::visual::layout::transparent_layout::Message::UpdateChosenKey;
    use crate::visual::layout::transparent_layout::TransparentLayout;

    #[test]
    fn created_with_canvas() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let (tx, rx) = channel::<Locator>();

        let layout = TransparentLayout::new(locator_root, tx);

        assert_eq!(layout.locators_canvas.location_key, None);
        assert_eq!(layout.locators_canvas.locations_paths.unwrap().len(), KeyQueueLengths::DOUBLE_CHAR);
    }

    #[test]
    fn set_first_chosen_key_update_layout() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let (tx, rx) = channel::<Locator>();
        let target_key = smol_str::SmolStr::from("f");

        let mut layout = TransparentLayout::new(locator_root, tx);
        assert_eq!(layout.locators_canvas.location_key, None);

        _ = layout.update(UpdateChosenKey(target_key));
        assert_eq!(layout.locators_canvas.location_key, Some(String::from("f")));
        assert_eq!(layout.locators_canvas.locations_paths.clone().unwrap().len(), KeyQueueLengths::SINGLE_CHAR);
        assert!(layout.locators_canvas.locations_paths.unwrap().into_iter().all(|path| path.1.contains('f')));
    }

    #[test]
    fn update_chosen_key_update_layout() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let (tx, rx) = channel::<Locator>();
        let first_target_key = smol_str::SmolStr::from("f");
        let second_target_key = smol_str::SmolStr::from("g");

        let mut layout = TransparentLayout::new(locator_root, tx);
        _ = layout.update(UpdateChosenKey(first_target_key));
        _ = layout.update(UpdateChosenKey(second_target_key));

        assert_eq!(layout.locators_canvas.location_key, Some(String::from("fg")));
        assert_eq!(layout.locators_canvas.locations_paths.clone().unwrap().len(), 1);
        assert!(layout.locators_canvas.locations_paths.unwrap().into_iter().all(|path| path.1.contains("fg")));
    }

    #[test]
    fn reset_chosen_key_update_layout() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let (tx, rx) = channel::<Locator>();
        let first_target_key = smol_str::SmolStr::from("q");

        let mut layout = TransparentLayout::new(locator_root, tx);
        _ = layout.update(UpdateChosenKey(first_target_key));
        assert_eq!(layout.locators_canvas.location_key, None);

        assert_eq!(layout.locators_canvas.locations_paths.clone().unwrap().len(), KeyQueueLengths::DOUBLE_CHAR);
    }

    #[test]
    fn over_choose_key_should_reset() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::DOUBLE_CHAR));
        let (tx, rx) = channel::<Locator>();
        let target_key = smol_str::SmolStr::from("f");

        let mut layout = TransparentLayout::new(locator_root, tx);
        _ = layout.update(UpdateChosenKey(target_key.clone()));
        _ = layout.update(UpdateChosenKey(target_key.clone()));
        _ = layout.update(UpdateChosenKey(target_key.clone()));
        
        assert_eq!(layout.locators_canvas.location_key, None);
        assert_eq!(layout.locators_canvas.locations_paths.clone().unwrap().len(), KeyQueueLengths::DOUBLE_CHAR);
    }
}
