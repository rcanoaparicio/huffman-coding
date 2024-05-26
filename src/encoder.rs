use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct TreeNode {
    value: u8,
    weight: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        other.weight == self.weight
    }
}

impl Eq for TreeNode {}

fn get_frequencies(data: &Vec<u8>) -> Vec<(u8, u32)> {
    let mut freqs = HashMap::<u8, u32>::new();
    for &b in data {
        *freqs.entry(b).or_default() += 1;
    }
    let mut freqs_vec = freqs.into_iter().collect::<Vec<(u8, u32)>>();
    freqs_vec.sort_by(|a, b| a.1.cmp(&b.1));
    freqs_vec
}

fn construct_tree(frequencies: Vec<(u8, u32)>) -> TreeNode {
    if frequencies.len() == 0 {
        panic!("No frequencies");
    }
    let mut heap = BinaryHeap::new();
    for frequency in frequencies {
        let node = TreeNode {
            value: frequency.0,
            weight: frequency.1,
            left: None,
            right: None,
        };
        heap.push(node);
    }

    while heap.len() > 1 {
        let first_node = heap.pop().unwrap();
        let second_node = heap.pop().unwrap();
        let new_node = TreeNode {
            value: 0,
            weight: first_node.weight + second_node.weight,
            left: Some(Box::new(second_node)),
            right: Some(Box::new(first_node)),
        };
        heap.push(new_node);
    }
    heap.pop().unwrap()
}

fn construct_dictionary(root: TreeNode, dict: &mut HashMap<u8, Vec<u8>>, path: Vec<u8>) {
    if root.left.is_none() && root.right.is_none() {
        dict.insert(root.value, path);
        return;
    }
    if root.left.is_some() {
        let mut new_path = path.clone();
        new_path.push(0);
        construct_dictionary(*root.left.unwrap(), dict, new_path);
    }

    if root.right.is_some() {
        let mut new_path = path.clone();
        new_path.push(1);
        construct_dictionary(*root.right.unwrap(), dict, new_path);
    }
}

pub fn encode_data(data: &Vec<u8>) -> Vec<u8> {
    let frequencies = get_frequencies(&data);
    let tree = construct_tree(frequencies);
    let mut dictionary: HashMap<u8, Vec<u8>> = HashMap::new();

    construct_dictionary(tree, &mut dictionary, Vec::new());
    let mut result: Vec<u8> = Vec::new();

    // Encode table
    let table_size: u8 = dictionary.len() as u8;
    result.push(table_size);
    for (key, val) in dictionary.iter() {
        result.push(*key);
        result.push(val.len() as u8);
        for v in val {
            result.push(*v);
        }
    }

    // Encode data
    let mut mask: u8 = 128;
    let mut temp_byte: u8 = 0;
    for c in data {
        let entry = dictionary.get(&c).unwrap();
        for n in entry {
            if *n == 1 {
                temp_byte |= mask;
            }
            mask /= 2;
            if mask == 0 {
                mask = 128;
                result.push(temp_byte);
                temp_byte = 0;
            }
        }
    }
    result
}
