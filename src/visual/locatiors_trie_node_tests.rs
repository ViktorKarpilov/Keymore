#[cfg(test)]
mod tests {
    use crate::{locator::locator::Locator, visual::locators_trie_node::LocatorTrieNode};
    use fake::{Fake, Faker};
    use windows::Win32::Foundation::POINT;

    enum LocatorSize {
        Small = 1,
        Medium,
        Big,
    }

    #[test]
    fn node_initialize() {
        let actual = LocatorTrieNode::new(get_test_locators(10));
        check_root(&actual);
    }

    #[test]
    fn construct_small() {
        let actual = LocatorTrieNode::new(get_test_locators(14));
        check_root(&actual);
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
        check_root(&actual);
    }

    #[test]
    fn construct_big() {
        let actual = LocatorTrieNode::new(get_test_locators(2744));
        check_root(&actual);
    }

    #[test]
    fn construct_very_big() {
        let actual = LocatorTrieNode::new(get_test_locators(3000));
        check_root(&actual);
    }

    #[test]
    fn get_children_returns_all() {
        let locator_root = LocatorTrieNode::new(get_test_locators(12));

        let children = locator_root.get_children();

        // assert_eq!(children.len(), 256);

        println!("Children: {:?}", children);
    }

    fn get_test_locators(count: usize) -> Vec<Locator> {
        let mut test_locators: Vec<Locator> = Vec::new();
        let point = || Locator {
            physical_point: POINT {
                x: Faker.fake(),
                y: Faker.fake(),
            },
            resolution_point: POINT {
                x: Faker.fake(),
                y: Faker.fake(),
            },
        };
        for _ in 0..count {
            test_locators.push(point());
        }

        test_locators
    }

    fn check_root(root: &LocatorTrieNode, size: LocatorSize) {
        // Root shout not have nodes
        assert!(root.node.is_none());
        // But should have children
        assert!(root.children.is_some());

        let mut children = root.children.as_ref();

        // go down a level
        for _ in 1..(size as usize) {
            let new_children = children.map(|childs| {
                let test = childs
                    .iter()
                    .map(|node| node.children.clone())
                    .filter_map(|childs_option| childs_option)
                    .reduce(|mut acc, mut childs_value| {
                        acc.append(&mut childs_value);
                        acc
                    })
            });
        }

        // Children for 10 should have node
        assert!(root
            .children
            .as_ref()
            .unwrap()
            .iter()
            .all(|child| child.node.is_some()));

        // Children for 10 should not have children
        assert!(root
            .children
            .as_ref()
            .unwrap()
            .iter()
            .all(|child| child.children.is_none()));
    }
}
