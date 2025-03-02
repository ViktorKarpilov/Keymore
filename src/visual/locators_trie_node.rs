use crate::{key_qeue_14, key_qeue_196, key_qeue_2744, locator::locator::Locator};

pub const DEFAULT_IDENTIFIER: char = '*';

// Note * - start identifier
#[derive(Debug, Clone)]
pub struct LocatorTrieNode {
    pub node: Option<Locator>,
    identifier: char,
    pub children: Option<Vec<LocatorTrieNode>>,
}

impl LocatorTrieNode {
    pub fn new(locators: Vec<Locator>) -> LocatorTrieNode {
        let mut keys = match locators.len() {
            0..=14 => key_qeue_14!(),
            15..=196 => key_qeue_196!(),
            _ => key_qeue_2744!(),
        };

        let mut root = LocatorTrieNode {
            node: None,
            identifier: DEFAULT_IDENTIFIER,
            children: None,
        };

        locators.into_iter().for_each(|locator| {
            let key = keys.pop().unwrap_or("!!!");
            // root.find_child(key).node = Some(locator);
            root.append_child_locator(key, locator);
        });

        root
    }

    pub fn get_children(self) -> Vec<(LocatorTrieNode, String)> {
        let identifier = self.identifier;
        let current_id = match self.identifier {
            DEFAULT_IDENTIFIER => String::from(""),
            non_trivial => non_trivial.to_string(),
        };

        match self.children {
            Some(children) => children
                .into_iter()
                .map(|child| child.get_children())
                .fold(Vec::new(), |mut acc, mut other| {
                    acc.append(&mut other);
                    acc
                })
                .into_iter()
                .map(|child| (child.0, format!("{}{}", current_id, child.1)))
                .collect(),
            None => vec![(self, identifier.to_string())],
        }
    }

    pub fn accessible_children(
        locators_trie_root: LocatorTrieNode,
        key: &str,
    ) -> Option<Vec<(LocatorTrieNode, String)>> {
        let identifier = locators_trie_root.identifier;

        if key.len() < 1 {
            return Some(vec![(locators_trie_root, identifier.to_string())]);
        }

        let current_id = match identifier {
            DEFAULT_IDENTIFIER => String::from(""),
            non_trivial => non_trivial.to_string(),
        };
        let target_identifier = key.chars().next().unwrap();
        let left_key = key.get(1..).unwrap();

        match locators_trie_root.children {
            Some(children) => Some(
                children
                    .into_iter()
                    .filter_map(|child| match child.identifier == target_identifier {
                        true => Some(LocatorTrieNode::accessible_children(child, left_key)),
                        false => None,
                    })
                    .fold(Vec::new(), |mut acc, other| {
                        if let Some(mut other_value) = other {
                            acc.append(&mut other_value);
                        }
                        acc
                    })
                    .into_iter()
                    .map(|child| (child.0, format!("{}{}", current_id, child.1)))
                    .collect(),
            ),
            None => {
                if key.len() == 1 && identifier == target_identifier {
                    return Some(vec![(locators_trie_root, identifier.to_string())]);
                }
                None
            }
        }
    }

    fn append_child_locator(&mut self, identifier: &str, locator: Locator) {
        let mut tail = self;

        for identifier_char in identifier.chars() {
            let need_new_child = tail.children.is_none()
                || !tail
                    .children
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|child| child.identifier == identifier_char);

            if need_new_child {
                if tail.children.is_none() {
                    tail.children = Some(Vec::new());
                }

                tail.children.as_mut().unwrap().push(LocatorTrieNode {
                    node: None,
                    identifier: identifier_char,
                    children: None,
                });
            }

            let children = tail.children.as_mut().unwrap();

            let child_index_option = children
                .iter()
                .position(|child| child.identifier == identifier_char);

            let child_index = child_index_option
                .expect("One and only one node always expected with any identifier");

            tail = &mut children[child_index];
        }

        tail.node = Some(locator);
    }
}
