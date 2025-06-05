use super::Result;
use super::SharedNode;

pub trait PrettyPrintTree {
    fn pretty_print_tree(&self, depth: usize) -> Result<()>;
}

impl PrettyPrintTree for SharedNode {
    fn pretty_print_tree(&self, depth: usize) -> Result<()> {
        let indent = (0..depth).map(|_| "   ").collect::<String>();
        let node = self.read()?;
        println!("{}{}", indent, node);

        for child in node.children.iter() {
            child.pretty_print_tree(depth + 1)?;
        }

        Ok(())
    }
}
