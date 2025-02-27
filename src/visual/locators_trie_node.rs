use crate::{key_qeue_14, key_qeue_196, key_qeue_2744, locator::locator::Locator};

pub const DEFAULT_IDENTIFIER: char = '*';

// Note * - start identifier
pub struct LocatorTrieNode {
    pub node: Option<Locator>,
    identifier: char,
    children: Option<Vec<LocatorTrieNode>>,
    key_len: Option<usize>,
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
            key_len: Some(keys[0].len()),
        };

        locators.into_iter().for_each(|locator| {
            let key = keys.pop().unwrap_or("!!!");
            root.find_child(key).node = Some(locator);
        });

        root
    }

    pub fn get_children(&self) -> Vec<(&LocatorTrieNode, String)> {
        let mut buffer: Vec<(&LocatorTrieNode, String)> = Vec::new();

        if let Some(children) = &self.children {
            for child in children {
                let mut idefier = String::new();
                if self.identifier != DEFAULT_IDENTIFIER {
                    idefier.push_str(self.identifier.to_string().as_str());
                }
                idefier.push_str(&child.identifier.to_string());
                buffer.push((&child, idefier));
            }
        }

        if buffer.first().unwrap().0.children.is_some() {
            let mut temp_buffer: Vec<(&LocatorTrieNode, String)> = Vec::new();
            buffer.iter().for_each(|buf| {
                let mut buf_accesible: Vec<(&LocatorTrieNode, String)> = buf
                    .0
                    .get_children()
                    .iter()
                    .map(|child| {
                        let child_id = child.1.clone();
                        let current_id = match self.identifier {
                            DEFAULT_IDENTIFIER => String::new(),
                            non_trivial => non_trivial.to_string(),
                        };
                        (child.0, format!("{current_id}{child_id}"))
                    })
                    .collect();

                temp_buffer.append(&mut buf_accesible);
            });
        }

        buffer
    }

    pub fn accessible_children<'a>(
        locators_trie_root: &'a LocatorTrieNode,
        key: &str,
    ) -> Option<Vec<(&'a LocatorTrieNode, String)>> {
        println!("Key: {:?}", key);

        let mut buffer: Option<Vec<(&LocatorTrieNode, String)>> = Some(vec![(
            &locators_trie_root,
            locators_trie_root.identifier.to_string(),
        )]);

        if key.len() < 1 {
            return buffer;
        }

        let current_id = match locators_trie_root.identifier {
            DEFAULT_IDENTIFIER => String::from(""),
            non_trivial => non_trivial.to_string(),
        };
        let target_identifier = key.chars().next().unwrap();
        let left_key = key.get(1..).unwrap();
        buffer = match &locators_trie_root.children {
            Some(children) => Some(
                children
                    .iter()
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
                    .iter()
                    .map(|child| (child.0, format!("{}{}", current_id, child.1)))
                    .collect(),
            ),
            None => None,
        };

        buffer
    }

    fn find_child(&mut self, identifier: &str) -> &mut LocatorTrieNode {
        let search_chars: Vec<char> = identifier.chars().map(|id| id as char).collect();
        let mut current_target = self;

        if current_target.children.is_none() {
            current_target.children = Some(Vec::new());
        }

        for search_target in search_chars {
            let children = current_target.children.as_mut().unwrap();

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
                        key_len: None,
                    });
                    children.len() - 1
                }
            };

            current_target = &mut children[index];
        }

        current_target
    }
}
