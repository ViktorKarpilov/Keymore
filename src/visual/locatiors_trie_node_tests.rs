#[cfg(test)]
mod tests {
    use crate::{locator::locator::Locator, visual::locators_trie_node::LocatorTrieNode};
    use fake::{Fake, Faker};
    use serde_json::json;
    use windows::Win32::Foundation::POINT;
    use crate::visual::test_helpers::{get_test_locators, KeyQueueLengths};

    #[derive(Clone)]
    enum LocatorSize {
        Small = 1,
        Medium,
        Big,
    }

    #[test]
    fn node_initialize() {
        let actual = LocatorTrieNode::new(get_test_locators(10));
        check_root(&actual, LocatorSize::Small);
    }

    #[test]
    fn construct_small() {
        let actual = LocatorTrieNode::new(get_test_locators(14));
        check_root(&actual, LocatorSize::Small);
    }

    #[test]
    fn construct_very_small() {
        let actual = LocatorTrieNode::new(get_test_locators(0));

        // Root shout not have nodes
        assert!(actual.node.is_none());

        // But should have children
        assert!(actual.children.is_none());
    }

    #[test]
    fn construct_medium() {
        let actual = LocatorTrieNode::new(get_test_locators(196));
        check_root(&actual, LocatorSize::Medium);
    }

    #[test]
    fn construct_big() {
        let actual = LocatorTrieNode::new(get_test_locators(2744));
        check_root(&actual, LocatorSize::Big);
    }

    #[test]
    fn construct_very_big() {
        let actual = LocatorTrieNode::new(get_test_locators(3000));
        check_root(&actual, LocatorSize::Big);
    }

    #[test]
    fn get_children_returns_all() {
        let locator_root = LocatorTrieNode::new(get_test_locators(12));

        let children = locator_root.get_children();

        assert_eq!(children.len(), 12);
    }
    
    #[test]
    fn accessible_children_contains_given_keys() {
        let locator_root = LocatorTrieNode::new(get_test_locators(196));

        let single_locator = LocatorTrieNode::accessible_children(locator_root.clone(), "ff");
        let locator_small_group = LocatorTrieNode::accessible_children(locator_root.clone(), "f");

        println!("Single locator: {}", json!(single_locator));
        assert_eq!(single_locator.clone().unwrap().len(), 1);
        assert_eq!(single_locator.unwrap()[0].1, "ff");
        println!("Small group: {}", json!(locator_small_group));
        assert_eq!(locator_small_group.unwrap().len(), 14);
    }

    #[test]
    fn search_accessible_children_returns_expected_group() {
        let locator_root = LocatorTrieNode::new(get_test_locators(KeyQueueLengths::TRIPLE_CHAR));
        
        let found_group  = LocatorTrieNode::accessible_children(locator_root.clone(), "ff");

        println!("Found group: {}", json!(found_group));
        assert_eq!(found_group.clone().unwrap().len(), 14);
        assert!(found_group.unwrap().into_iter().all(|group| group.1.len() == 3));
    }

    #[test]
    fn search_accessible_children_when_between_char_returns_expected_group() {
        let locator_root = LocatorTrieNode::new(get_test_locators(50));

        println!("Root: {}", json!(locator_root));
        let found_group  = LocatorTrieNode::accessible_children(locator_root.clone(), "t");

        println!("Found group: {}", json!(found_group));
        assert_eq!(found_group.clone().unwrap().len(), 14);
        assert!(found_group.unwrap().into_iter().all(|group| group.1.len() == 2));
    }
    
    fn check_root(root: &LocatorTrieNode, size: LocatorSize) {
        // Root should not have nodes
        assert!(root.node.is_none());
        // But should have children
        assert!(root.children.is_some());

        let mut children = root.children.as_ref();
        let mut buffer_children:Option<Vec<LocatorTrieNode>> = None;

        // go down a level
        for _ in 1..(size.clone() as usize) {
            let new_children = match children {
                Some(childs) => childs
                    .iter()
                    .map(|node| node.children.clone())
                    .filter_map(|childs_option| childs_option)
                    .reduce(|mut acc, mut childs_value| {
                        acc.append(&mut childs_value);
                        acc
                    }),
                None => None,
            };

            buffer_children = new_children;
            children = buffer_children.as_ref();
        }

        match size {
            LocatorSize::Small => {
                // Children for small should have node
                assert!(root
                    .children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|child| child.node.is_some()));

                // Children for small should not have children
                assert!(root
                    .children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|child| child.children.is_none()));
            },
            LocatorSize::Medium => {
                println!("Root: {}", json!(root));

                // Children of children for medium should have node
                assert!(root
                    .children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|child| {
                        child.children
                            .as_ref()
                            .unwrap()
                            .iter()
                            .all(|child_l2| child_l2.node.is_some())
                    }));

                // Children of children for medium should not have children
                assert!(root
                    .children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|child| {
                        child.children
                            .as_ref()
                            .unwrap()
                            .iter()
                            .all(|child_l2| child_l2.children.is_none())
                    }));
            },
            LocatorSize::Big => {
                println!("Root: {}", json!(root));
                
                // Children of children of children for big should have node
                assert!(root
                    .children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|child| {
                        child.children
                            .as_ref()
                            .unwrap()
                            .iter()
                            .all(|child_l2| {
                                child_l2.children
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .inspect(|child_l3| {
                                        println!("child_l3: {}", json!(child_l3));
                                    })
                                    .all(|child_l3| {
                                        match child_l3.node {
                                            Some(_) => (),
                                            None => println!("child_l3 has none node: {}", json!(child_l3)),
                                        }
                                        
                                        child_l3.node.is_some()
                                    })
                            })
                    }));

                // Children of children of children for big should not have children
                assert!(root
                    .children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .all(|child| {
                        child.children
                            .as_ref()
                            .unwrap()
                            .iter()
                            .all(|child_l2| {
                                child_l2.children
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .all(|child_l3| {
                                        match child_l3.node {
                                            Some(_) => (),
                                            None => println!("child_l3 has some child: {}", json!(child_l3)),
                                        }
                                        
                                        child_l3.children.is_none()
                                    })
                            })
                    }));
            }
        }
    }
}
