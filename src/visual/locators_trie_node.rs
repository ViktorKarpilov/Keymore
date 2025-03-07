use serde::Serialize;
use serde_json::json;
use crate::{key_qeue_14, key_qeue_196, key_qeue_2744, locator::locator::Locator};

pub const DEFAULT_IDENTIFIER: char = '*';

// Note * - start identifier
#[derive(Debug, Clone, Serialize)]
pub struct LocatorTrieNode {
    pub node: Option<Locator>,
    identifier: char,
    pub children: Option<Vec<LocatorTrieNode>>,
    pub key_len: usize,
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
            key_len: keys[0].len(),
        };

        locators.into_iter().for_each(|locator| {
            let key = keys.pop().unwrap_or("!!!");
            root.find_child(key).node = Some(locator);
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
        if key.len() > locators_trie_root.key_len{
            return None;
        }
        
        let identifier = locators_trie_root.identifier;

        if key.len() < 1 {
            return match locators_trie_root.children{
                None => Some(vec![(locators_trie_root, identifier.to_string())]),
                Some(children) => {
                    children
                    .into_iter()
                    .map(|child| {
                        let child_identifier = child.identifier;
                        match child.children {
                            Some(_) => LocatorTrieNode::accessible_children(child, key).unwrap(),
                            None => vec!((child, format!("{}{}", locators_trie_root.identifier.clone(), child_identifier))),
                        }
                    })
                    .reduce(|mut acc, mut other| {
                        acc.append(&mut other);
                        acc
                    })
                }
            };
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

    fn find_child(&mut self, identifier: &str) -> &mut LocatorTrieNode {
        let search_chars: Vec<char> = identifier.chars().map(|id| id as char).collect();
        let mut current_target = self;

        if current_target.children.is_none() {
            current_target.children = Some(Vec::new());
        }

        for search_target in search_chars {
            if current_target.children.is_none(){
                current_target.children = Some(Vec::new());
            }

            let children =  current_target.children.as_mut().unwrap();

            let index = match children
                .iter()
                .position(|child| child.identifier == search_target)
            {
                Some(idx) => idx,
                None => {
                    children.push(LocatorTrieNode {
                        node: None,
                        identifier: search_target,
                        children: None,
                        key_len: current_target.key_len,
                    });
                    children.len() - 1
                }
            };

            current_target = &mut children[index];
        }

        current_target
    }
}
