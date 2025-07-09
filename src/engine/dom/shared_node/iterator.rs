use super::{Error, Result, SharedNode};

pub struct NodeIterator {
    stack: Vec<SharedNode>,
}

impl Iterator for NodeIterator {
    type Item = Result<SharedNode>;

    fn next(&mut self) -> Option<Self::Item> {
        // remove the current node from the stack, which will then be returned
        let node = self.stack.pop()?;

        // add the nodes's children to the stack
        let children = match node.read() {
            Ok(guard) => guard.children.clone(),
            Err(_) => return Some(Err(Error::Poison)),
        };
        self.stack.extend(children);

        Some(Ok(node))
    }
}

impl TryFrom<&SharedNode> for NodeIterator {
    type Error = super::Error;

    fn try_from(node: &SharedNode) -> std::result::Result<Self, Self::Error> {
        Ok(NodeIterator {
            stack: node.read()?.children.clone(),
        })
    }
}
