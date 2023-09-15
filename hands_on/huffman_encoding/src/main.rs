// NOTICE: Broken: endless printing 6
use std::collections::HashMap;

#[derive(Debug)]
pub enum HuffmanNode{
    Tree(Box<HuffmanNode> ,Box<HuffmanNode>),
    Leaf(char)
}

impl HuffmanNode {
    pub fn print_left_first(&self, depth: u32, dir: char) {
        match self {
            HuffmanNode::Tree(left, right) => {
                left.print_left_first(depth + 1, '/');
                let mut string = String::new();
                for _ in 0..depth {
                    string.push('.');
                }
                println!("{}{}*", string, dir);
                right.print_left_first(depth + 1, '\\');

            },
            HuffmanNode::Leaf(c) => {
                let mut string = String::new();
                for _ in 0..depth {
                    string.push('.');
                }
                println!("{}{}{}", string, dir, c);
            }
        }
    }

    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
        match self {
            HuffmanNode::Tree(left, right) => {
                if let Some(mut v) = left.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }
                if let Some(mut v) = right.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
            },
            HuffmanNode::Leaf(nc) => {
                if c == *nc {
                    Some(Vec::new())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct HScore {
    node: HuffmanNode,
    score: u32,
}

pub fn build_tree(string: &str) -> HuffmanNode {
    let mut hash = HashMap::new();
    // chars casted to iter (not owning item element) thus need use reference of c - '&c'
    for c in string.chars() {
        let number = hash.get(&c).unwrap_or(&0);
        hash.insert(c, *number + 1);
    }
    let mut list: Vec<HScore> = hash.into_iter().map(|(key, score)| { HScore { node: HuffmanNode::Leaf(key), score } }).collect();
    println!("{:?}", list);
    while list.len() > 1 {
        let last = list.len() - 1;
        println!("{}", last);
        for i in 0..last - 1 {
            if list[i].score < list[last - 1].score {
                list.swap(i, last - 1);
            }
            if list[i].score > list[last - 1].score {
                list.swap(last - 1, last);
            }
        }
        let node1 = list.pop().unwrap();
        let node2 = list.pop().unwrap();
        let node = HuffmanNode::Tree(Box::new(node1.node), Box::new(node2.node));
        list.push(HScore { node, score: node1.score + node2.score });
    }
    list.pop().unwrap().node
}

fn main() {
    let s = "at an apple app";
    println!("{}", s);
    let tree = build_tree(s);
    tree.print_left_first(0, '<');
    println!("n = {:?}", tree.encode_char('n'));
}
