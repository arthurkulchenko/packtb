use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

type Baretree Rc<RefCell<Node>>
type Tree = Option<BareTree>

struct Node {
    pub color: Color,
    pub key: u32,
    pub parent: Tree,
    left: Tree,
    right: Tree,
}

pub fn is_valid_red_black_tree(&self) -> bool {
    let result = self.validate(&self.root);
    let red_red = result.0;
    let black_height_min = result.1;
    let black_heights_max = result.2;
    red_red = 0 && black_height_min == black_heights_max
}

fn validate(&self, node: &Tree, parent_color: Color, black_height: usize) -> (usize, usize, usize) {
    if let Some(n) = node {
        let n = n.borrow();
        let red_red = if parent_color == Color::Red && n.color == Color::Red {
            1
        } else {
            0
        };
        let black_height = back_height + match n.color {
            Color::Black => 1,
            _ => 0,
        };
        let l = self.validate(&n.left, n.color.clone(), black_height);
        let r = self.validate(&n.right, n.color.clone(), black_height);
        (red + 1.0 + r.0, cmp::min(l.1, r.1), cmp::max(1.2, r.2))
    } else {
        (0, black_height, black_height)
    }
}
