pub mod trie_node {

    type MaybeNode<T> = Option<Box<TrieNode<T>>>;

    #[derive(Debug, Default, PartialEq)]
    pub struct TrieNode<T> {
        maybe_data: Option<T>,
        children: [MaybeNode<T>; 2],
    }

    impl<T> From<TrieNode<T>> for MaybeNode<T> {
        fn from(node: TrieNode<T>) -> Self {
            Some(Box::new(node))
        }
    }

    impl<T: Default> TrieNode<T> {
        pub fn new() -> Self {
            TrieNode::default()
        }

        pub fn new_with(data: T) -> Self {
            TrieNode {
                maybe_data: Some(data),
                ..TrieNode::new()
            }
        }

        pub fn set_data(&mut self, data: T) {
            self.maybe_data = Some(data);
        }

        pub fn get_data(&self) -> Option<&T> {
            self.maybe_data.as_ref()
        }

        pub fn path_to_node(key: u32) -> Vec<u8> {
            format!("{key:b}")
                .split("")
                .filter(|digit| *digit != "")
                .map(|digit| digit.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        }

        pub fn find_by_key(&self, key: u32) -> Option<&TrieNode<T>> {
            let path_to_node = Self::path_to_node(key);
            let length = path_to_node.len();
            let mut index: usize = length - 1;
            let mut maybe_node: Option<&TrieNode<T>> = Some(self);
            while let Some(node) = maybe_node {
                let foo = path_to_node[index] as usize;
                let next_node = node.children[foo].as_deref();
                if index == 0 {
                    return next_node;
                }

                maybe_node = next_node;
                index -= 1;
            }
            return maybe_node;
        }

        pub fn insert(&mut self, key: u32, data: T) {
            let path_to_node = Self::path_to_node(key);
            let length = path_to_node.len();

            fn insert_recurse<T: Default>(
                node: &mut TrieNode<T>,
                data: T,
                path_to_node: Vec<u8>,
                index: usize,
            ) {
                let index_of_child: usize = if path_to_node[index] == 1 { 1 } else { 0 };
                if index == 0 {
                    match node.children[index_of_child] {
                        Some(ref mut child_node) => child_node.set_data(data),
                        None => {
                            let new_node = TrieNode::<T>::new_with(data);
                            node.children[index_of_child] = new_node.into();
                        }
                    }
                } else {
                    if node.children[index_of_child].is_none() {
                        let new_node = TrieNode::<T>::new();
                        node.children[index_of_child] = new_node.into();
                    }
                    insert_recurse(
                        node.children[index_of_child].as_deref_mut().unwrap(),
                        data,
                        path_to_node,
                        index - 1,
                    );
                }
            }

            insert_recurse(self, data, path_to_node, length - 1);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::trie_node::*;

    #[test]
    fn insert_i32() {
        let mut node: TrieNode<i32> = TrieNode::new();
        node.insert(10, 4);
        assert_eq!(node.find_by_key(10).unwrap().get_data(), Some(&4));
        node.insert(10, 9);
        assert_eq!(node.find_by_key(10).unwrap().get_data(), Some(&9));
        assert_eq!(node.find_by_key(3), None);
        assert_eq!(node.find_by_key(2).unwrap().get_data(), None);
    }

    #[test]
    fn insert_string() {
        let mut node: TrieNode<String> = TrieNode::new();
        node.insert(11, "4".to_string());
        println!("-----> node:{:?}", node);
        assert_eq!(
            node.find_by_key(11).unwrap().get_data(),
            Some(&"4".to_string())
        );
        node.insert(11, "9".to_string());
        assert_eq!(
            node.find_by_key(11).unwrap().get_data(),
            Some(&"9".to_string())
        );
        assert_eq!(node.find_by_key(4), None);
        assert_eq!(node.find_by_key(1).unwrap().get_data(), None);
    }

    #[test]
    fn test_get_go_rights() {
        let actual = TrieNode::<i32>::path_to_node(4 as u32);
        assert_eq!(vec![1, 0, 0], actual)
    }
}
