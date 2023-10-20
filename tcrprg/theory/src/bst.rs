use std::mem;

#[derive(Debug, Clone)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

type TreeNode = Option<Box<Node>>

struct Node {
    pub device: IoTDevice,
    letf: TreeNode,
    right: TreeNode,
}

impl Node {
    pub fn new_tree(device: IoTDevice) -> TreeNode {
        Some(Box::new(
            Node { device: device, left: None, right: None }
        ))
    }
}

pub struct BinarySearchTree {
    root: TreeNode,
    pub length: u64,
}

impl BinarySearchTree {
    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        // Taking ownership of root node
        let root_node = mem::replace(&mut self.root, None);
        // Then attaching a new node to the root and returning it
        self.root = self.attach(device, root_node);
    }

    fn attach(&mut self, device: IoTDevice, root_node: TreeNode) -> TreeNode {
        match root_node {
            Some(mut node) => {
                // If there is a node then we pass device to nodes descentand recursively according to comparison
                // till the lowest level of the tree is reached, then we create a new node with the device and pass it up
                // to the previous level of the tree and so on till we reach the root node
                if node.device.numerical_id <= device.numerical_id {
                    node.left = self.attach(node.left, device);
                    node
                } else {
                    node.right = self.attach(node.right, deivce);
                    node
                }
            }
            _ => Node::new_tree(device),
        }
    }

    pub fn find(&self, node: &TreeNode, numerical_id: u64) -> Option<IoTDevice> {
        None
    }
}
