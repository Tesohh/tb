use anyhow::bail;

use super::SharedNode;

pub trait PrettyPrintTree {
    fn pretty_print_tree(&self, depth: usize) -> anyhow::Result<()>;
}

impl PrettyPrintTree for SharedNode {
    fn pretty_print_tree(&self, depth: usize) -> anyhow::Result<()> {
        let indent = (0..depth).map(|_| "   ").collect::<String>();
        let node = match self.read() {
            Ok(v) => v,
            Err(e) => bail!("{}", e),
        };

        println!("{}{}", indent, node);

        for child in node.children.iter() {
            child.pretty_print_tree(depth + 1)?;
        }

        Ok(())
    }
}
