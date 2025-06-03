use super::SharedNode;

pub struct NodeIterator {
    stack: Vec<SharedNode>,
}

impl Iterator for NodeIterator {
    type Item = SharedNode;

    fn next(&mut self) -> Option<Self::Item> {
        // remove the current node from the stack, which will then be returned
        let node = self.stack.pop()?;

        // add the nodes's children to the stack
        let children = node.read().unwrap().children.clone();
        self.stack.extend(children);

        Some(node)
    }
}

impl From<&SharedNode> for NodeIterator {
    fn from(node: &SharedNode) -> Self {
        NodeIterator {
            stack: node.read().unwrap().children.clone(),
        }
    }
}
