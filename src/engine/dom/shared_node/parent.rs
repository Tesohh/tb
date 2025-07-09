use super::{Error, Result, SharedNode, UnreachableError};

pub trait Parent
where
    Self: Sized,
{
    fn parent(&self) -> Result<Self>;
}

impl Parent for SharedNode {
    fn parent(&self) -> Result<Self> {
        self.read()
            .or(Err(Error::Poison))?
            .parent
            .clone()
            .ok_or(UnreachableError::NoParent)?
            .upgrade()
            .ok_or(Error::MissingParentUpgrade)
    }
}
